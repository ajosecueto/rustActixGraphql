mod mutation;
mod query;
mod subscriptions;

use std::sync::Arc;
use actix_web::{guard, web, HttpRequest, HttpResponse, Result};
use async_graphql::dataloader::DataLoader;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, GraphiQLSource};
use async_graphql::{Context, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use sqlx::PgPool;
use crate::drivers;
use crate::graphql::mutation::MutationRoot;
use crate::graphql::query::QueryRoot;
// use crate::graphql::mutation::MutationRoot;
// use crate::graphql::subscriptions::SubscriptionRoot;


// pub type AppSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;
pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;



pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::post().to(index))
            .route(
                web::get()
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .route(web::get().to(index_graphiql)),
    );
}

async fn index(
    schema: web::Data<AppSchema>,
    http_req: HttpRequest,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut query = req.into_inner();
    // let getting_role_result = common_utils::get_role(http_req);
    // query = query.data(getting_role_result);
    schema.execute(query).await.into()
}

async fn index_ws(
    schema: web::Data<AppSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}


async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("http://localhost:8000")
                .finish(),
        ))
}

pub fn create_schema_with_context(pool: PgPool) -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    // let cloned_pool = Arc::clone(&arc_pool);
    // let details_data_loader =
    //     DataLoader::new(DetailsLoader { pool: cloned_pool }, actix_rt::spawn).max_batch_size(10);


    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        // limits are commented out, because otherwise introspection query won't work
        // .limit_depth(3)
        // .limit_complexity(15)
        .data(pool.clone())
        // .data(details_data_loader)
        .data(drivers::kafka::create_producer())
        .enable_subscription_in_federation()
        .finish()
}