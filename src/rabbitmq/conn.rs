use std::env;

use deadpool_lapin::{Manager, Pool};
use lapin::{Connection, ConnectionProperties};

pub async fn establish_rabbit_connection(env_prefix: &String) -> Connection {
    let dsn: String = env::var(format!("{}_RMQ_DSN", env_prefix)).unwrap();
    Connection::connect(dsn.as_str(), ConnectionProperties::default())
        .await
        .unwrap()
}

pub async fn establish_rabbit_connection_pool(env_prefix: &String) -> Pool {
    let dsn: String = env::var(format!("{}_RMQ_DSN", env_prefix)).unwrap();

    let manager = Manager::new(dsn, ConnectionProperties::default());
    Pool::builder(manager).max_size(10).build().unwrap()
}
