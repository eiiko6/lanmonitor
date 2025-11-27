use axum::{
    Router,
    http::{Method, header},
};
use std::{
    env::var,
    net::{Ipv4Addr, SocketAddr, UdpSocket},
};
use tower_http::cors::{Any, CorsLayer};

mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);

    let app = Router::new().merge(routes::routes()).layer(cors);

    let port = var("LANMONITOR_SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Listening on {addr}");

    let sock = UdpSocket::bind("0.0.0.0:0")?;
    sock.connect("8.8.8.8:80")?;

    let local = sock.local_addr()?;
    let ip = match local.ip() {
        std::net::IpAddr::V4(v4) => v4,
        _ => Ipv4Addr::UNSPECIFIED,
    };

    tracing::info!("Local IP to use: {ip}:8080");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();

    Ok(())
}
