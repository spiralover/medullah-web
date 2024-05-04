use std::env;
use lapin::{Connection, ConnectionProperties};

pub async fn establish_rabbit_connection(env_prefix: &String) -> Connection {
    let dsn: String = env::var(format!("{}_RMQ_DSN", env_prefix)).unwrap();
    Connection::connect(dsn.as_str(), ConnectionProperties::default())
        .await
        .unwrap()
}
