use crate::error::Error;

/// Helper function to run simple SQL statements from
/// which we don't expect any response back.
pub(crate) async fn execute_statement(
    connection: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
    sql: String,
) -> Result<(), Error> {
    connection
        .simple_query(sql)
        .await
        .map_err(Error::TiberiusError)?;
    Ok(())
}

/// Match on the result and execute a rollback statement against the database if we run into an
/// error.

pub async fn rollback_on_exception<T>(
    result: Result<T, Error>,
    connection: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
) -> Result<T, Error> {
    if result.is_err() {
        // If rolling back fails, ignore it.
        let _ = execute_statement(connection, "ROLLBACK".to_string()).await;
    }
    result
}
