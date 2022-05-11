use actix_web::ResponseError;
use sqlx::pool::PoolConnection;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub struct DBPool {
    pub pool: Pool<Postgres>,
}

impl DBPool {
    pub async fn connect() -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("DATABASE_URL").expect("Env DATABASE_URL unset"))
            .await
            .unwrap();
        Self { pool }
    }

    pub async fn get_connection(&self) -> Result<PoolConnection<Postgres>, DBError> {
        self.pool.acquire().await.map_err(|err| {
            println!("{err}");
            DBError::Connection
        })
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
pub enum DBError {
    #[display(fmt = "DB connection error")]
    Connection,
    #[display(fmt = "Unable to write in db. Check query")]
    Write,
    #[display(fmt = "Unable to read from db. Check query")]
    Read,
    #[display(fmt = "Unable to delete from db. Check query")]
    Delete,
}
impl ResponseError for DBError {}
