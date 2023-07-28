use super::graphql::Schema;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Request,
};
use async_graphql_warp::{graphql, GraphQLResponse};
use std::net::SocketAddr;
pub use warp::{
    header::headers_cloned, http::Response as HttpResponse, reject, reply, Filter, Rejection, Reply,
};

const BEARER: &str = "Bearer ";

/// Runs the GraphQL server.
///
/// Note that `key` is not compatible with the DER-encoded key extracted by
/// rustls-pemfile.
pub async fn serve(schema: Schema, addr: SocketAddr, cert: Vec<u8>, key: Vec<u8>) {
    let graphql_filter = warp::header::optional::<String>("authorization")
        .and(graphql(schema))
        .and_then(graphql_handler);

    let route_home = warp::path::end().map(|| "");
    let routes = graphql_filter
        .or(graphql_playground_filter())
        .or(route_home);

    let server = warp::serve(routes).tls().cert(cert).key(key).bind(addr);

    println!("listening on https://{addr:?}");
    server.await;
}

fn graphql_playground_filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let html = playground_source(GraphQLPlaygroundConfig::new("/graphql"));
    warp::path!("graphql" / "playground").map(move || {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(html.clone())
    })
}

async fn graphql_handler(
    auth_header: Option<String>,
    (schema, mut request): (Schema, Request),
) -> Result<GraphQLResponse, Rejection> {
    let is_allow_query = request
        .operation_name
        .clone()
        .map_or(false, |operation_name| {
            let queries = vec!["signIn"];
            queries.contains(&operation_name.as_str())
        });

    let resp = schema.execute(request).await;
    Ok::<_, Rejection>(async_graphql_warp::GraphQLResponse::from(resp))
}
