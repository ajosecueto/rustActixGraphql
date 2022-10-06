use std::env;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;


pub async fn create_connection_pool() -> PgPool {
    let db_url = env::var("DATABASE_URL").expect("Can't get DB URL");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await.expect("Can't get DB connection");
    pool
}


#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_create_connection_pool() {
        create_connection_pool().await;
    }
}