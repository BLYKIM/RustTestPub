use postgres::{Client, Error, NoTls};

///
/// # Errors
/// todo
pub fn postgres() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/testdb", NoTls)?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS author (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            country         VARCHAR NOT NULL
            )
    ",
    )?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS book  (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            author_id       INTEGER NOT NULL REFERENCES author
            )
    ",
    )?;

    Ok(())
}

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let manager = PostgresConnectionManager::new("host=localhost user=postgres dbname=mydb", NoTls);
    let pool = Pool::builder().build(manager).await.unwrap();

    let connection = pool.get().await?;

    // Create table
    connection
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS users (
            username TEXT PRIMARY KEY,
            password TEXT NOT NULL
        )
    ",
        )
        .await?;

    // Insert data
    connection
        .execute(
            "INSERT INTO users (username, password) VALUES ($1, $2)",
            &[&"user1", &"password1"],
        )
        .await?;

    // Query data
    let rows = connection
        .query("SELECT * FROM users WHERE username=$1", &[&"user1"])
        .await?;
    let username: &str = rows[0].get(0);
    let password: &str = rows[0].get(1);
    println!("username: {}, password: {}", username, password);

    Ok(())
}
