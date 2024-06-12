# Native Mutations with ndc-sqlserver

## Introduction

Native mutations allow you to run custom SQL queries on your SQL Server database that
can be exposed via the Hasura GraphQL engine and modify the database state.

## Setup

Native mutations can be defined by adding them to the `metadata.nativeMutations` section
of the `configuration.json`.

Each query is specified as a parameterized SQL. The return structure of the query must
be explicitly specified in the `columns` field.

Native mutations can take arguments using the `{{argument_name}}` syntax. Arguments must
be specified along with their type.

Note that the arguments are not interpolated, but provided to your data source as parameters,
and therefore must be specific values, not arbitrary SQL.

## Schema

A native mutation can be defined using the following fields:


### Native Mutation Object {#native-mutation-object}

| Field name    | Type                                                                                              | Required | Notes                                                                      |
|---------------|---------------------------------------------------------------------------------------------------|----------|----------------------------------------------------------------------------|
| `sql`         | [Native query SQL Syntax](https://hasura.io/docs/3.0/connectors/postgresql/native-queries/syntax) | Yes      | Parameterized SQL query that needs to be run.                              |
| `columns`     | JSON Object <K: Identifier of the column, V: [ColumnObject](#column-object)>                      | Yes      | Schema of the response that will be obtained after the `sql` query is run. |
| `arguments`   | JSON Object <K: Identifier of the column, V: [ArgumentObject](#argument-object)>                  | Yes      | Schema of the arguments that will be passed to the `sql` query.            |
| `description` | String                                                                                            | No       | Description of the native mutation.                                        |


### Column Object {#column-object}

| Field name    | Type   | Required | Notes                                                                                                                                                       |
|---------------|--------|----------|-------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `name`        | String | Yes      | Name of the column that will be returned in the SQL query's response                                                                                        |
| `type`        | String | Yes      | Type of the column.                                                                                                                                         |
| `nullable`    | String | Yes      | Nullability of the column.                                                                                                                                  |
| `description` | String | No       | Description of the column.                                                                                                                                  |
| `castAs`      | String | No       | If set, the column will be parsed into this type, otherwise would be parsed as `type`. For example, if you have a                                           |
|               |        |          | field called `Name` with type `VARCHAR(100)`, then the `type` should be `varchar` and `castAs` should be set as `VARCHAR(100)` to get the correct response. |


### Argument Object {#argument-object}

| Field name    | Type   | Required | Notes                                                                                                                                                       |
|---------------|--------|----------|-------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `name`        | String | Yes      | Name of the argument.                                                                                        |
| `type`        | String | Yes      | Type of the argument.                                                                                                                                         |
| `nullable`    | String | Yes      | Nullability of the argument.                                                                                                                                  |
| `description` | String | No       | Description of the argument.                                                                                                                                  |


## Configuration

The following is an example of a native mutation which inserts a row
into the `Artist` table and returns the `ArtistId` and the `Name`.

```json
{
    "nativeMutations": {
      "insert_artist_and_return_id": {
        "sql": "INSERT INTO [dbo].[Artist] (ArtistId, Name) OUTPUT inserted.*  VALUES ({{ArtistId}}, {{Name}})",
        "columns": {
          "ArtistId": {
            "name": "ArtistId",
            "type": "int",
            "nullable": "nonNullable",
            "description": null
          },
          "Name": {
            "name": "Name",
            "type": "varchar",
            "nullable": "nullable",
            "description": null,
            "castAs": "varchar(100)"
          }
        },
        "arguments": {
          "ArtistId": {
            "name": "ArtistId",
            "type": "int",
            "nullable": "nonNullable",
            "description": null
          },
          "Name": {
            "name": "Name",
            "type": "varchar",
            "nullable": "nullable",
            "description": null
          }
        },
        "description": null
      }
    }
}
```

This native mutation will then be exposed as a procedure to the Hasura metadata.

## Usage

You can now run the mutation in your GraphQL API:

```graphql
mutation {
  insert_artist_and_return_id(ArtistId: 1, Name: "Bob") {
    returning {
      ArtistId
      Name
    }
    affected_rows
  }
}
```
