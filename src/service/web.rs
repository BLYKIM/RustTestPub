use crate::service::postgres::Database;
use serde::Deserialize;
use std::net::SocketAddr;
use warp::{Filter, Rejection, Reply};

// need to change types
#[derive(Deserialize)]
struct Query {
    id: String,
    key: String,
}

pub async fn serve(db: Database, addr: SocketAddr, cert: Vec<u8>, key: Vec<u8>) {
    let routes = request(db);
    let server = warp::serve(routes).tls().cert(cert).key(key).bind(addr);

    println!("listening on https://{addr:?}");

    server.await;
}

fn request(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let db = warp::any().map(move || db.clone());
    warp::path!("v1.0" / "rest")
        .and(warp::get())
        .and(warp::query())
        .and(db)
        .and_then(handle_request)
}

async fn handle_request(query: Query, db: Database) -> Result<impl Reply, Rejection> {
    match db.check_account(&query.id, &query.key).await {
        Ok(true) => Ok(warp::reply::json(&"ok")),
        Ok(false) => Ok(warp::reply::json(&"invalid account")),
        Err(_) => Err(warp::reject()),
    }
}
