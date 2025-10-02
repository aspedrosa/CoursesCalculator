use axum::{routing::post, Router, Json};
use core::{fetch_event};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/fetch_events/", post(move |Json(body): Json<Vec<u32>>| {
        async move {
            let futures = body.into_iter().map(fetch_event);

            let results = futures::future::join_all(futures).await;
            // map errors and return Json
            Ok::<_, axum::http::StatusCode>(Json(results))
        }
    }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}