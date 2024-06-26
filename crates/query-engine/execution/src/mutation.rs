use crate::{
    helpers::{execute_statement, rollback_on_exception},
    query::execute_query,
};
use bytes::{BufMut, Bytes, BytesMut};
use query_engine_metrics::metrics;
use query_engine_sql::sql::{
    self,
    ast::With,
    execution_plan::{
        MutationOperationExecutionPlan, NativeMutationOperationExecutionPlan,
        StoredProcedureExecutionPlan,
    },
    string::SQL,
};
use query_engine_translation::translation::mutation::generate_native_mutation_response_cte;
use std::collections::{BTreeMap, HashMap};
use tiberius::ExecuteResult;

use crate::error::{Error, MutationError, NativeMutationResponseParseError};

use tracing::{info_span, Instrument};

/// Runs the `plan` in a transaction. If this function returns an error,
/// then the transaction should be rolled back.
async fn execute_mutations_transaction(
    plan: sql::execution_plan::MutationExecutionPlan,
    connection: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
) -> Result<Bytes, Error> {
    execute_statement(connection, "BEGIN TRANSACTION".to_string()).await?;

    // this buffer represents the JSON response
    let mut buffer = BytesMut::new();
    buffer.put(&[b'{'][..]); // we start by opening an object
    buffer.put(&b"\"operation_results\":"[..]); // specify the key for MutationResponse
    buffer.put(&[b'['][..]); // open the responses array

    let mut i = plan.mutations.into_iter();

    if let Some(mutation) = i.next() {
        execute_mutation(connection, mutation, &mut buffer).await?;

        for mutation in i {
            buffer.put(&[b','][..]); // each result, except the first, is prefixed by a ','

            execute_mutation(connection, mutation, &mut buffer).await?;
        }
    }

    buffer.put(&[b']'][..]); // Close the operation_results array

    buffer.put(&[b'}'][..]); // we end by closing the object

    execute_statement(connection, "COMMIT".to_string()).await?;

    Ok(buffer.freeze())
}

/// Convert the rows obtained after the mutation query that gets executed to
/// a JSON value. Each column of a row is parsed according to its type in the DB.
fn convert_mutation_response_to_json(db_results: &[tiberius::Row]) -> Result<String, Error> {
    // Each element of the vector corresponds to the results of a single SQL statement.
    let mut db_results_json: Vec<HashMap<String, Option<serde_json::Value>>> = Vec::new();

    for row in db_results.iter() {
        let mut row_result_json = HashMap::new();

        let cols = row.columns().to_owned();

        for col in cols.into_iter() {
            match col.column_type() {
                tiberius::ColumnType::Int1 => {
                    let int_val = row
                        .try_get::<u8, &str>(col.name())
                        .map_err(Error::TiberiusError)?;
                    row_result_json.insert(
                        col.name().to_owned(),
                        int_val.map(|i| serde_json::Value::Number(i.into())),
                    );
                }

                tiberius::ColumnType::Int2 => {
                    let int_val = row
                        .try_get::<i16, &str>(col.name())
                        .map_err(Error::TiberiusError)?;
                    row_result_json.insert(
                        col.name().to_owned(),
                        int_val.map(|i| serde_json::Value::Number(i.into())),
                    );
                }

                tiberius::ColumnType::Int4 => {
                    let int_val = row
                        .try_get::<i32, &str>(col.name())
                        .map_err(Error::TiberiusError)?;
                    row_result_json.insert(
                        col.name().to_owned(),
                        int_val.map(|i| serde_json::Value::Number(i.into())),
                    );
                }

                tiberius::ColumnType::Int8 | tiberius::ColumnType::Intn => {
                    let int_val = row
                        .try_get::<i64, &str>(col.name())
                        .map_err(Error::TiberiusError)?;
                    row_result_json.insert(
                        col.name().to_owned(),
                        int_val.map(|i| serde_json::Value::Number(i.into())),
                    );
                }

                tiberius::ColumnType::Float4
                | tiberius::ColumnType::Float8
                | tiberius::ColumnType::Floatn => {
                    let float_val = row
                        .try_get::<f64, &str>(col.name())
                        .map_err(Error::TiberiusError)?;
                    let json_float_val = match float_val {
                        Some(f) => Some(serde_json::Number::from_f64(f).ok_or(Error::Mutation(
                            MutationError::NativeMutation {
                                column_name: col.name().to_string(),
                                column_type: col.column_type(),
                                error: NativeMutationResponseParseError::InvalidJSONFloatNumber,
                            },
                        ))?),
                        None => None,
                    };

                    row_result_json.insert(
                        col.name().to_owned(),
                        json_float_val.map(serde_json::Value::Number),
                    );
                }

                tiberius::ColumnType::NChar
                | tiberius::ColumnType::NVarchar
                | tiberius::ColumnType::NText
                | tiberius::ColumnType::BigVarChar => {
                    let string_val = row
                        .try_get::<&str, &str>(col.name())
                        .map_err(Error::TiberiusError)?;

                    row_result_json.insert(
                        col.name().to_owned(),
                        string_val.map(|s| serde_json::Value::String(s.to_string())),
                    );
                }

                tiberius::ColumnType::Datetime
                | tiberius::ColumnType::Daten
                | tiberius::ColumnType::Datetime2
                | tiberius::ColumnType::Datetime4
                | tiberius::ColumnType::DatetimeOffsetn
                | tiberius::ColumnType::Datetimen => {
                    let date_time_str = row
                        .try_get::<&str, &str>(col.name())
                        .map_err(Error::TiberiusError)?;
                    row_result_json.insert(
                        col.name().to_owned(),
                        date_time_str.map(|d| serde_json::Value::String(d.to_string())),
                    );
                }

                _ => {
                    return Err(Error::Mutation(MutationError::NativeMutation {
                        column_name: col.name().to_string(),
                        column_type: col.column_type(),
                        error: NativeMutationResponseParseError::UnknownType(col.column_type()),
                    }))
                }
            }
        }

        db_results_json.push(row_result_json);
    }

    serde_json::to_string(&db_results_json)
        .map_err(|e| Error::Mutation(MutationError::JSONSerializationError(e)))
}

pub async fn execute_mutations(
    mssql_pool: &bb8::Pool<bb8_tiberius::ConnectionManager>,
    metrics: &metrics::Metrics,
    plan: sql::execution_plan::MutationExecutionPlan,
) -> Result<Bytes, Error> {
    let acquisition_timer = metrics.time_connection_acquisition_wait();
    let connection_result = mssql_pool
        .get()
        .instrument(info_span!("Acquire connection"))
        .await
        .map_err(Error::ConnectionPool);
    let mut connection = acquisition_timer.complete_with(connection_result)?;

    let query_timer = metrics.time_mutation_execution();
    let mutation_response = rollback_on_exception(
        execute_mutations_transaction(plan, &mut connection).await,
        &mut connection,
    )
    .await;
    query_timer.complete_with(mutation_response)
}

async fn execute_mutation(
    connection: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
    mutation_plan: MutationOperationExecutionPlan,
    buffer: &mut (impl BufMut + Send),
) -> Result<(), Error> {
    match mutation_plan {
        MutationOperationExecutionPlan::NativeMutation(native_mutation_plan) => {
            execute_native_mutation(connection, native_mutation_plan, buffer).await
        }
        MutationOperationExecutionPlan::StoredProcedure(stored_procedure_plan) => {
            execute_stored_procedure(connection, stored_procedure_plan, buffer).await
        }
    }
}

async fn execute_stored_procedure(
    connection: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
    plan: StoredProcedureExecutionPlan,
    buffer: &mut (impl BufMut + Send),
) -> Result<(), Error> {
    let mut sql = SQL::new();

    plan.stored_procedure_sql_query.to_sql(&mut sql);

    // User provided native mutation query.
    let mut mssql_query = tiberius::Query::new(sql.sql);

    // bind parameters....
    for param in sql.params.clone().into_iter() {
        match param {
            sql::string::Param::String(string) => {
                mssql_query.bind(string);
            }
            // Variables are not used with mutations.
            sql::string::Param::Variable(_) => {}
        }
    }

    // go!
    let _ = mssql_query
        .execute(connection)
        .await
        .map_err(Error::TiberiusError)?;

    let mut response_selection_sql = SQL::new();

    plan.response_selection.to_sql(&mut response_selection_sql);

    println!("response selection SQL is {}", response_selection_sql.sql);

    // Execute the SQL query and append the response obtained to the `buffer`.
    execute_query(
        connection,
        &response_selection_sql,
        &BTreeMap::new(),
        buffer,
    )
    .await?;

    Ok(())
}

/// Execute the mutation query.
async fn execute_native_mutation(
    connection: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
    native_mutation_plan: NativeMutationOperationExecutionPlan,
    buffer: &mut (impl BufMut + Send),
) -> Result<(), Error> {
    let mutation_query = &native_mutation_plan.mutation_sql_query;

    // User provided native mutation query.
    let mut mssql_query = tiberius::Query::new(mutation_query.sql.as_str());

    // bind parameters....
    for param in mutation_query.params.clone().into_iter() {
        match param {
            sql::string::Param::String(string) => {
                mssql_query.bind(string);
            }
            // Variables are not used with mutations.
            sql::string::Param::Variable(_) => {}
        }
    }

    // go!
    let stream = mssql_query
        .query(connection)
        .await
        .map_err(Error::TiberiusError)?;

    // Execute the native mutation and collect its response.
    let native_mutation_response = stream.into_results().await.map_err(Error::TiberiusError)?;

    // We expect each Native mutation to return exactly one row set.
    if native_mutation_response.len() > 1 {
        return Err(Error::Mutation(
            MutationError::NativeMutationMoreThanOneRowSet {
                native_mutation_name: native_mutation_plan.native_mutation_name.clone(),
            },
        ));
    }

    // Parse the response received from the DB and convert it into JSON to prepare
    // it for the next stage i.e. to select fields from the response obtained.
    let response_json =
        convert_mutation_response_to_json(native_mutation_response.first().unwrap_or(&Vec::new()))?;

    // Create a CTE with the above obtained `response_json` which will make it
    // suitable to query the JSON value through an SQL query.
    let mutation_response_cte = generate_native_mutation_response_cte(
        response_json,
        native_mutation_plan.response_selection.response_json_schema,
        native_mutation_plan
            .response_selection
            .response_cte_table_alias,
    );

    let mut response_selection_select = native_mutation_plan.response_selection.response_select;

    // Add the CTE to the `response_selection_select`.
    response_selection_select.with = With {
        common_table_expressions: vec![mutation_response_cte],
    };

    let mut response_selection_sql = SQL::new();

    response_selection_select.to_sql(&mut response_selection_sql);

    // Execute the SQL query and append the response obtained to the `buffer`.
    execute_query(
        connection,
        &response_selection_sql,
        &BTreeMap::new(),
        buffer,
    )
    .await?;

    Ok(())
}
