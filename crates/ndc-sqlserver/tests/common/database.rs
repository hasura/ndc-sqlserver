//! Functions used to create and teardown test databases. Use via helpers in `mod.rs` rather
//! than directly.

use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use uuid;

#[derive(Debug, Clone)]
pub struct MSSQLDatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub db_name: String,
    pub password: String,
}

impl MSSQLDatabaseConfig {
    pub fn construct_uri(&self) -> String {
        let MSSQLDatabaseConfig {
            host,
            port,
            user,
            db_name,
            password,
        }: &MSSQLDatabaseConfig = &self;
        format!(
            "Server={host},{port};Uid={user};Database={db_name};Pwd={password};TrustServerCertificate=true"
        )
    }

    pub fn original_db_config() -> MSSQLDatabaseConfig {
        MSSQLDatabaseConfig {
            host: "localhost".into(),
            port: 64003,
            user: "SA".into(),
            password: "Password!".into(),
            db_name: "Chinook".into(),
        }
    }
}

/// create a fresh db with a random name, return it's name and connection string
pub async fn create_fresh_database(connection_config: MSSQLDatabaseConfig) -> MSSQLDatabaseConfig {
    let id = uuid::Uuid::new_v4();
    let db_name = format!("temp-{}", id);
    create_database_copy(connection_config, &db_name).await
}

pub async fn create_mssql_connection(
    connection_config: &MSSQLDatabaseConfig,
) -> Client<Compat<TcpStream>> {
    let connection_uri = connection_config.construct_uri();
    let config = Config::from_ado_string(&connection_uri).unwrap();

    let tcp = TcpStream::connect(config.get_addr()).await.unwrap();
    tcp.set_nodelay(true).unwrap();

    let connection = Client::connect(config, tcp.compat_write()).await.unwrap();
    connection
}

/// connect to database with `connection_uri` then create a new DB called `new_db_name`
/// which is a copy of the `chinook_template` database.
async fn create_database_copy(
    mut connection_config: MSSQLDatabaseConfig,
    new_db_name: &str,
) -> MSSQLDatabaseConfig {
    let mut connection = create_mssql_connection(&connection_config).await;

    let create_db_sql = format!("CREATE DATABASE \"{new_db_name}\";");

    connection
        .simple_query(create_db_sql.as_str())
        .await
        .unwrap();

    connection_config.db_name = new_db_name.to_string();
    connection_config
}

/// given a connection string, drop a database `db_name`
pub async fn drop_database(db_name: &str, connection_uri: String) -> Result<(), String> {
    let config = Config::from_ado_string(&connection_uri).unwrap();

    let tcp = TcpStream::connect(config.get_addr()).await.unwrap();
    tcp.set_nodelay(true).unwrap();

    println!("Connection config is {config:#?}");

    let mut connection = Client::connect(config, tcp.compat_write()).await.unwrap();

    let drop_db_sql = format!("USE master; ALTER DATABASE  \"{db_name}\" SET SINGLE_USER WITH ROLLBACK IMMEDIATE; DROP DATABASE \"{db_name}\" ");

    // we don't mind if this fails
    match connection.simple_query(drop_db_sql).await {
        Err(e) => {
            println!("Dropping DB {} failed with error: {}", db_name, e);
        }
        Ok(_) => {}
    }
    Ok(())
}

// #[test]
// fn test_same_db_name() {
//     let connection_uri = "postgresql://user:password@internet.com:100/database";
//     assert_eq!(
//         replace_database_name(connection_uri, "database"),
//         connection_uri.to_string()
//     )
// }

// #[test]
// fn test_different_db_name() {
//     let connection_uri = "postgresql://user:password@internet.com:100/database";
//     let expected = "postgresql://user:password@internet.com:100/new-database";

//     assert_eq!(
//         replace_database_name(connection_uri, "new-database"),
//         expected.to_string()
//     )
// }

// #[test]
// fn test_different_db_name_no_port() {
//     let connection_uri = "postgresql://user:password@internet.com/database";
//     let expected = "postgresql://user:password@internet.com/new-database";

//     assert_eq!(
//         replace_database_name(connection_uri, "new-database"),
//         expected.to_string()
//     )
// }
