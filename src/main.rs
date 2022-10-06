extern crate rustActixGraphql;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use rustActixGraphql::drivers::postgres::create_connection_pool;
use rustActixGraphql::graphql::{configure_service, create_schema_with_context};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = create_connection_pool().await;

    let schema = web::Data::new(create_schema_with_context(pool));

    println!("GraphiQL IDE: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .configure(configure_service)
            .app_data(schema.clone())
    })
        .bind("0.0.0.0:8000")?
        .run()
        .await
}