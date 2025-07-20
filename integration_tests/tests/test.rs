use std::{net::TcpListener, sync::Arc};

use integration_tests::{init_tracing, spawn_test_server};
use sochat_client_lib::{SochatClient, storage::Storage};
use url::Url;

#[tokio::test]
async fn registering_account_works() -> anyhow::Result<()> {
    init_tracing();
    let tcp = TcpListener::bind("127.0.0.1:0")?;
    let addr = tcp.local_addr()?;
    let _g = spawn_test_server(tcp);

    let server_url = Arc::new(Url::parse(&format!("http://{addr}"))?);

    let storage = Storage::connect_in_memory().await?;

    let client = SochatClient::new(storage);
    let _ = client.register_account(server_url.clone()).await?;
    assert_eq!(client.get_accounts().await?.len(), 1);

    let _ = client.register_account(server_url).await?;
    assert_eq!(client.get_accounts().await?.len(), 2);

    Ok(())
}
