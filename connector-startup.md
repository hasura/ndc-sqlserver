## Overview

`ndc-sqlserver` provides a Hasura Data Connector to the SQL Server database, which can expose and run GraphQL queries
via the Hasura v3 Project.

## Getting started

### Step 1: Prerequisites

1. Install the [new Hasura CLI](https://hasura.io/docs/3.0/cli/installation/) -  to quickly and easily create and manage
   Hasura projects and builds
2. Install the [Hasura VS Code extension](https://marketplace.visualstudio.com/items?itemName=HasuraHQ.hasura)
3. Have a SQL Server database - for supplying data to the API

### Step 2: Login to Hasura

After our prerequisites are taken care of, login to Hasura Cloud with the CLI:

```bash
ddn login
```

This will open up a browser window and initiate an OAuth2 login flow. If the browser window doesn't open automatically,
use the link shown in the terminal output to launch the flow.

### Step 3: Create a new project

We'll use the create project command to create a new project:

```bash
ddn create project --dir ./my-first-supergraph
```

The CLI will respond with information about your new project, including the console URL:

```
+-------------+-----------------------------------------------------+
| Name        | <NAME>                                              |
+-------------+-----------------------------------------------------+
| ID          | <ID>                                                |
+-------------+-----------------------------------------------------+
| Console URL | https://console.hasura.io/project/<NAME>            |
+-------------+-----------------------------------------------------+
```

Additionally, it will log some information about the
[project](https://hasura.io/docs/3.0/project-configuration/projects).

### Step 4: Add the SQL Server Connector


A [connector manifest](https://hasura.io/docs/3.0/supergraph-modeling/build-manifests#connector-manifests) is the file
which contains the details of the connector's configuration. This tells Hasura DDN what capabilities the connector has
and how to build that connector for your data source.

Let's move into the project directory:

```bash
cd my-first-supergraph
```
Then, create a connector manifest by passing a name — in this case `app_connector` — to the `add connector-manifest`
command:

```bash
ddn add connector-manifest app_connector --subgraph app --hub-connector hasura/postgres --type cloud
```

Open your project in VS Code and open the `app_connector.build.hml` file in our project. We can then add the
`CONNECTION_URI`'s value:

```
# other configuration above
CONNECTION_URI:
  value: "<sql-server-database-connection-string>"
```

### Step 5: Build GraphQL API

We can use dev mode to watch our project and create new builds as changes are made to our metadata:

```bash
ddn dev
```

We'll see the CLI creates our first build, displays the URL for our project's Console, and continues to watch for
changes 

```
INF Models and commands added to the project successfully
INF Doing a supergraph build...
INF Building SupergraphManifest "base"...
◑+---------------+----------------------------------------------------------------------------------------------------+
| Build Version | 3405408c06                                                                                         |
+---------------+----------------------------------------------------------------------------------------------------+
| Description   | Dev build - Tue, 02 Apr 2024                                                                       |
|               | 13:36:57 CDT                                                                                       |
+---------------+----------------------------------------------------------------------------------------------------+
| API URL       | https://<PROJECT_NAME>-default-3405408c06.ddn.hasura.app/graphql                                   |
+---------------+----------------------------------------------------------------------------------------------------+
| Console URL   | https://console.hasura.io/project/<PROJECT_NAME>/environment/default/build/3405408c06/graphql      |
+---------------+----------------------------------------------------------------------------------------------------+
| Project Name  | <PROJECT_NAME>                                                                                     |
+---------------+----------------------------------------------------------------------------------------------------+
INF Starting ConnectorManifest watcher for connector "app_connector" in subgraphName "app"
INF Starting ConnectorLink watcher for connector "app_connector" in subgraphName "app"
```

### Step 6: Run query

Use the GraphiQL Explorer in Console to create your own query.


