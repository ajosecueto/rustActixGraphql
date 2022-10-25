use std::env;

use scylla::{Session, SessionBuilder};

pub async fn create_scylla_session() -> Session {
    let user = env::var("SCYLLADB_USER").expect("Can't get DB URL");
    let password = env::var("SCYLLADB_PASSWORD").expect("Can't get DB URL");
    let nodes = env::var("SCYLLADB_NODES").expect("Can't get NODES");
    let session = SessionBuilder::new()
    .known_nodes(&nodes.split(",").collect::<Vec<&str>>())
    .user(user, password)
    .build()
    .await.expect("Can connect To SycllaDB");

    session
}