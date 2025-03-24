use anyhow::Result;
use async_std::net::TcpStream;
use tokio::spawn;
use tokio_postgres::{Client, Error, NoTls};

async fn main() -> Result<(), Error> {
    let conn_str = "host=127.0.0.1 user=corentin password=test dbname=todo_list";

    let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await?;

    spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    println!("Connected to PostgreSQL!");
    Ok(())
}

async fn connect_through_port() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&CONN_STR_PORT)?;

    let tcp = TcpStream::connect(config.get_addr()).await?;

    tcp.set_nodelay(true)?;

    let client = Client::connect(config, tcp).await?;
    println!("Successfully connected to server.");

    client.close().await?;

    Ok(())
}
