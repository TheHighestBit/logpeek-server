mod routes;

use axum::Router;
use log::info;
use routes::router_setup;

pub async fn run() {
    let app: Router = router_setup();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}