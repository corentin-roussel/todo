use anyhow::Ok;
use async_std::net::TcpStream;
use once_cell::sync::Lazy;
use std::env;
use tiberius::{Client, Config};

static CONN_STR_PORT: Lazy<string> = Lazy::new(|| {
    env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or_else(
        |_| "server=tcp:127.0.0.1\\postgres,5432;database=todo_list;user=corentin;password=test",
    )
});

async fn connect_through_port() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&CONN_STR_PORT)?;
}
