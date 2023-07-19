use anyhow::Result;
use bb8_postgres::{
    bb8::Pool,
    tokio_postgres::{types::ToSql, Config},
    PostgresConnectionManager,
};
use std::path::Path;
use tokio_postgres_rustls::MakeRustlsConnect;

pub type _Value = dyn ToSql + Sync;

#[derive(Clone)]
pub struct Database {
    pool: Pool<PostgresConnectionManager<MakeRustlsConnect>>,
}

impl Database {
    pub async fn new<P: AsRef<Path>>(url: &str, roots: &[P]) -> Result<Self> {
        let config = url.parse::<Config>()?;

        let tls = MakeRustlsConnect::new(crate::service::tls::build_client_config(roots)?);
        let manager = PostgresConnectionManager::new(config, tls);
        let pool = Pool::builder().build(manager).await?;

        Ok(Database { pool })
    }

    pub async fn check_account(&self, username: &str, password: &str) -> Result<bool> {
        let conn = self.pool.get().await?;
        if let Some(row) = conn
            .query_opt(
                "SELECT user_pw FROM user_table WHERE user_id=$1",
                &[&username],
            )
            .await?
        {
            if let Some(hash) = row.get::<_, &str>(0).strip_prefix("{bcrypt}") {
                return bcrypt::verify(password, hash).map_err(Into::into);
            }
        }
        Ok(false)
    }
}
