# Stored Procedures with ndc-sqlserver

## Introduction

Stored procedures are a powerful feature of SQL Server that allow you to encapsulate logic in the
database. This can be useful for a number of reasons, such as:

- **Performance**: Stored procedures can be precompiled and cached, which can improve performance.
- **Security**: Stored procedures can be used to control access to data.
- **Consistency**: Stored procedures can be used to enforce business rules.
- **Code Reuse**: Stored procedures can be reused across multiple applications.

In this guide, we'll look at how to use stored procedures with the ndc-sqlserver connector.

## Tracking Stored Procedures

The ndc-sqlserver connector can track stored procedures in a SQL Server database.

The stored procedures present in the database can be added by running the following command:

```
ddn connector plugin --connector app/sqlserver/connector.yaml -- update stored-procedures
```

If you want to overwrite the existing stored procedures,

```
ddn connector plugin --connector app/sqlserver/connector.yaml -- update stored-procedures --overwrite
```



After running the above command, the stored procedures will appear in the `$.metadata.storedProcedures`
key of the configuration that is generated by the `update` operation.


Example of a stored procedure configuration that's generated by the introspection:

```json
{
  "storedProcedures": {
    "GetArtistsByName": {
      "name": "GetArtistsByName",
      "schema": "dbo",
      "arguments": {
        "Name": {
          "name": "Name",
          "type": "varchar",
          "nullable": "nullable",
          "isOutput": false,
          "description": null
        }
      },
      "returns": null,
      "description": null
    }
  }
}
```

### Return type of Stored Procedures

The stored procedure generated by the introspection contains `returns` as `null` because a stored
procedure can return multiple result sets or output parameters, so it is not possible for the connector
to automatically determine the return type of the stored procedure. You should manually update the
stored procedure's configuration to include the return type of the stored procedure, as shown below.

The return type of a stored procedure should include the fields that are going to be returned by the
stored procedure. The fields should be defined in the `returns` key of the stored procedure
configuration.

For example, if the stored procedure returns `CustomerId`, `Phone` and `TotalPurchases`,
we can add a return type for it, as following:

```json
{
  "storedProcedures": {
    "GetArtistsByName": {
      "name": "GetArtistsByName",
      "schema": "dbo",
      "arguments": {
        "Name": {
          "name": "Name",
          "type": "varchar",
          "nullable": "nullable",
          "isOutput": false,
          "description": null
        }
      },
      "returns": {
        "CustomerId": {
          "name": "CustomerId",
          "type": "int",
          "nullable": "nonNullable",
          "description": null
        },
        "Phone": {
          "name": "Phone",
          "type": "varchar",
          "nullable": "nonNullable",
          "description": null
        },
        "TotalPurchases": {
          "name": "TotalPurchases",
          "type": "int",
          "nullable": "nonNullable",
          "description": null
        }
      },
      "description": null
    }
  }
}
```

### Marking required arguments as `nonNullable`

If your stored procedure contains a required argument, then you can mark the argument as `nonNullable`
which will enable to throw a validation error as soon as possible. For example, in the above, the `Phone` field
is a required argument, hence it is marked as `nonNullable`.

## Schema of Stored Procedures

## Schema

A stored procedure can be defined using the following fields:


### Native Mutation Object

| Field name    | Type                                                                             | Required | Notes                                                                                                                                                                         |
|---------------|----------------------------------------------------------------------------------|----------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `name`        | Name of the stored procedure                                                     | Yes      |                                                                                                                                                                               |
| `schema`      | Name of the schema of the stored procedure                                       | Yes      |                                                                                                                                                                               |
| `arguments`   | JSON Object <K: Identifier of the column, V: [ArgumentObject](#argument-object)> | Yes      | Schema of the arguments that will be passed to the stored procedure `sql` query.                                                                                              |
| `returns`     | JSON Object <K: Identifier of the column, V: [ColumnObject](#column-object)>     | No       | Schema of the columns that will be returned by the stored procedure. Note that if this key is not present, the stored procedure won't be added to the schema of the connector |
| `description` | String                                                                           | No       | Description of the stored procedure.                                                                                                                                          |



### Column Object

| Field name    | Type   | Required | Notes                                                                |
|---------------|--------|----------|----------------------------------------------------------------------|
| `name`        | String | Yes      | Name of the column that will be returned in the SQL query's response |
| `type`        | String | Yes      | Type of the column.                                                  |
| `description` | String | No       | Description of the column.                                           |
| `nullable`    | String | Yes      | Nullability of the column.                                           |
| `description` | String | No       | Description of the column.                                           |


### Argument Object

| Field name    | Type   | Required | Notes                                                              |
|---------------|--------|----------|--------------------------------------------------------------------|
| `name`        | String | Yes      | Name of the argument.                                              |
| `type`        | String | Yes      | Type of the argument.                                              |
| `nullable`    | String | Yes      | Nullability of the argument.                                       |
| `isOutput`    | String | Yes      | Boolean value to indicate if the argument can contain output value |
| `description` | String | No       | Description of the argument.                                       |
