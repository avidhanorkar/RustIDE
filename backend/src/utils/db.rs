use tokio_postgres::{Client, NoTls, Error};


pub async fn connect_db () -> Result<Client, Error> {

    let db_url : String = "host=localhost user=postgres password=1234 dbname=rustdb".to_string();

    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            println!("Connection Error: {}", e);
        }
    });

    Ok(client)
}