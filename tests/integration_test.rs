use testcontainers::clients::Cli;

mod common;

#[actix_web::test]
async fn test_setup() {
    let docker = Cli::default();
    let (_pg_container, pool) = common::setup(&docker).await;
}