use axum::{routing, Router, Json};
use core::{fetch_event};

#[tokio::main]
async fn main() {
    let mut app = Router::new().route("/fetch_events", routing::post(move |Json(body): Json<Vec<u32>>| {
        println!("Received request for events: {:?}", body);

        async move {
            let futures = body.into_iter().map(fetch_event);

            let results = futures::future::join_all(futures).await;
            // map errors and return Json
            Ok::<_, axum::http::StatusCode>(Json(results))
        }
    }));

    app = app.route("/health", routing::get(|| {
        async {
            Ok::<_, axum::http::StatusCode>("Ok")
        }
    }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}