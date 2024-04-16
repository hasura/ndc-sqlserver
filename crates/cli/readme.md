# ndc-sqlserver-cli

ndc-sqlserver-cli is used to configure a deployment of ndc-sqlserver.

## Create a configuration

Create a configuration in a new directory using the following commands:

1. Initialize a configuration:

   ```sh
   CONNECTION_URI='<sqlserver-connection-string>' cargo run --bin ndc-sqlserver-cli -- --context='<directory>'  initialize
   ```

2. Update the configuration by introspecting the database:

   ```sh
   CONNECTION_URI='<sqlserver-connection-string>' cargo run --bin ndc-sqlserver-cli -- --context='<directory>'  update
   ```
