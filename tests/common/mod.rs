use std::env;
use dotenv::dotenv;
use sqlx::PgPool;
use testcontainers::clients::Cli;
use testcontainers::{Container, RunnableImage};
use testcontainers::images::postgres::Postgres;
use rustActixGraphql::drivers::postgres::create_connection_pool;

pub async fn setup(docker: &Cli) -> (Container<Postgres>, PgPool) {
    dotenv().ok();
    let pg_container = setup_database(docker);
    let pool = create_connection_pool().await;
    // run_migrations(&mut pool.await.expect("Can't get DB connection"));
    (pg_container, pool)
}

fn setup_database(docker: &Cli) -> Container<Postgres> {
    let pg_container = docker.run(get_pg_image());
    let pg_port = pg_container.get_host_port_ipv4(5432);
    env::set_var(
        "DATABASE_URL",
        format!(
            "postgres://postgres:password@localhost:{}/preferences-ms",
            pg_port
        ),
    );
    pg_container
}

fn get_pg_image() -> RunnableImage<Postgres> {
    RunnableImage::from(Postgres::default())
        .with_env_var(("POSTGRES_DB", "references-ms"))
        .with_env_var(("POSTGRES_PASSWORD", "123456789"))
}
