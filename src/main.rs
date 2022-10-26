extern crate rustActixGraphql;

use std::rc::Rc;
use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use rustActixGraphql::events::reducer::Reducer;
use rustActixGraphql::infrastructure::postgres::create_connection_pool;
use rustActixGraphql::infrastructure::scylla::create_scylla_session;
use rustActixGraphql::graphql::{configure_service, create_schema_with_context};
use rustActixGraphql::infrastructure::kafka::create_producer;
use scylla::Session;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = create_connection_pool().await;
    let session: Session = create_scylla_session().await;
    let session = Arc::new(session);
    let reducer = Reducer{
        db: Rc::new(pool.clone())
    };
    actix_rt::spawn(async move { reducer.start_consumer().await });
    let producer = create_producer();
    let schema = web::Data::new(create_schema_with_context(pool, session, producer));
    println!("GraphiQL IDE: http://localhost:6000");

    HttpServer::new(move || {
        App::new()
            .configure(configure_service)
            .app_data(schema.clone())
    })
        .bind("0.0.0.0:6000")?
        .run()
        .await
}