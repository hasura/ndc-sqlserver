# SQL Server Connector

[![Docs](https://img.shields.io/badge/docs-v3.x-brightgreen.svg?style=flat)](https://hasura.io/docs/3.0)
[![ndc-hub](https://img.shields.io/badge/ndc--hub-sqlserver-blue.svg?style=flat)](https://hasura.io/connectors/sqlserver)
[![License](https://img.shields.io/badge/license-Apache--2.0-purple.svg?style=flat)](LICENSE.txt)

> **Note:** ADO.NET is the supported connection string format for SQL Server for ndc-sqlserver in DDN. 
> You can find the documentation for ADO.NET SQL Server connection strings [here](https://learn.microsoft.com/en-us/dotnet/framework/data/adonet/connection-string-syntax#sqlclient-connection-strings).
> This is a change from Hasura version 2, where ODBC connection strings were supported.


With this connector, Hasura allows you to instantly create a real-time GraphQL API on top of your data models in
Microsoft SQL Server. This connector supports SQL Server's functionalities listed in the table below, allowing for
efficient and scalable data operations. Additionally, users benefit from all the powerful features of Hasura’s Data
Delivery Network (DDN) platform, including query pushdown capabilities that delegate query operations to the database,
thereby enhancing query optimization and performance.

This connector is built using the [Rust Data Connector SDK](https://github.com/hasura/ndc-hub#rusk-sdk) and implements
the [Data Connector Spec](https://github.com/hasura/ndc-spec).

- [See the listing in the Hasura Hub](https://hasura.io/connectors/sqlserver)
- [Hasura V3 Documentation](https://hasura.io/docs/3.0/)

## Features

Below, you'll find a matrix of all supported features for the SQL Server connector:

| Feature                         | Supported | Notes                                |
|---------------------------------|-----------|--------------------------------------|
| Native Queries + Logical Models | ✅        |                                      |
| Native Mutations                | ✅        |                                      |
| Simple Object Query             | ✅        |                                      |
| Filter / Search                 | ✅        |                                      |
| Simple Aggregation              | ✅        |                                      |
| Sort                            | ✅        |                                      |
| Paginate                        | ✅        |                                      |
| Table Relationships             | ✅        |                                      |
| Views                           | ✅        |                                      |
| Remote Relationships            | ✅        |                                      |
| Stored Procedures               | ✅        |                                      |
| Custom Fields                   | ❌        |                                      |
| Mutations                       | ❌        | Only native mutations are suppported |
| Distinct                        | ✅        |                                      |
| Enums                           | ❌        |                                      |
| Naming Conventions              | ❌        |                                      |
| Default Values                  | ❌        |                                      |
| User-defined Functions          | ❌        |                                      |

## Before you get Started

1. Create a [Hasura Cloud account](https://console.hasura.io)
2. Please ensure you have the [DDN CLI](https://hasura.io/docs/3.0/cli/installation) and [Docker](https://docs.docker.com/engine/install/) installed
3. [Create a supergraph](https://hasura.io/docs/3.0/getting-started/init-supergraph)
4. [Create a subgraph](https://hasura.io/docs/3.0/getting-started/init-subgraph)

The steps below explain how to Initialize and configure a connector for local development. You can learn how to deploy a
connector — after it's been configured — [here](https://hasura.io/docs/3.0/getting-started/deployment/deploy-a-connector).

## Using the SQL Server connector

### Step 1: Authenticate your CLI session

```bash
ddn auth login
```

### Step 2: Configure the connector

Once you have an initialized supergraph and subgraph, run the initialization command in interactive mode while
providing a name for the connector in the prompt:

```bash
ddn connector init <connector-name> -i
```

#### Step 2.1: Choose the `hasura/sqlserver` from the list

#### Step 2.2: Choose a port for the connector

The CLI will ask for a specific port to run the connector on. Choose a port that is not already in use or use the
default suggested port.

#### Step 2.3: Provide the env vars for the connector

> **Note:** The `CONNECTION_URI` is the connection string of the SQL Server database. You can find the documentation for ADO.NET SQL Server connection string formats [here](https://learn.microsoft.com/en-us/dotnet/framework/data/adonet/connection-string-syntax#sqlclient-connection-strings).

| Name           | Description                                      | Required | Default |
|----------------|--------------------------------------------------|----------|---------|
| CONNECTION_URI | The connection string of the SQL Server database | Yes      | N/A     |

## Step 3: Introspect the connector

```bash
ddn connector introspect <connector-name>
```

This will generate a `configuration.json` file that will have the schema of your SQL Server database.

## Step 4: Add your resources

```bash
ddn connector-link add-resources <connector-name>
```

This command will track all the containers in your SQL Server DB as [Models](https://hasura.io/docs/3.0/supergraph-modeling/models).

## Documentation

View the full documentation for the ndc-sqlserver connector [here](./docs/readme.md).

## Contributing

We're happy to receive any contributions from the community. Please refer to our [development guide](./docs/development.md).

## License

The Hasura SQL Server connector is available under the [Apache License
2.0](https://www.apache.org/licenses/LICENSE-2.0).
