use std::net::SocketAddr;

pub async fn spawn_test_server() -> anyhow::Result<SocketAddr> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;

    tokio::spawn(async move {
        axum::serve(listener, gtm::app::app()).await.unwrap();
    });

    Ok(addr)
}
