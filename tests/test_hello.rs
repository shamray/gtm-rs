mod common;

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn test_hello_no_name() -> Result<()> {
    let addr = common::spawn_test_server().await?;
    let hc = httpc_test::new_client(&format!("http://{addr}"))?;
    let response = hc.do_get("/hello").await?;

    let json = response.json_body()?;

    assert_eq!(json!({"response": "Hello, world"}), json);

    Ok(())
}

#[tokio::test]
async fn test_hello_with_name() -> Result<()> {
    let addr = common::spawn_test_server().await?;
    let hc = httpc_test::new_client(&format!("http://{addr}"))?;
    let response = hc.do_get("/hello?name=Rusty").await?;

    let json = response.json_body()?;

    assert_eq!(json!({"response": "Hello, Rusty"}), json);

    Ok(())
}

#[tokio::test]
async fn test_hello2() -> Result<()> {
    let addr = common::spawn_test_server().await?;
    let hc = httpc_test::new_client(&format!("http://{addr}"))?;
    let response = hc.do_get("/hello2/Rusty").await?;

    let json = response.json_body()?;

    assert_eq!(json!({"response": "Hello, Rusty"}), json);

    Ok(())
}
