use core::fmt;
use std::net::SocketAddr;

pub use httpc_test::Client;

pub async fn spawn_test_server() -> anyhow::Result<SocketAddr> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;

    tokio::spawn(async move {
        axum::serve(listener, gtm::app::app().unwrap())
            .await
            .unwrap();
    });

    Ok(addr)
}

pub fn create_test_client<T: fmt::Display>(addr: T) -> anyhow::Result<Client> {
    let client = httpc_test::new_client(&format!("http://{addr}"))?;

    Ok(client)
}
