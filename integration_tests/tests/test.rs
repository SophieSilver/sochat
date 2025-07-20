use std::net::TcpListener;

use integration_tests::spawn_test_server;
use sochat_client_lib::{SochatClient, storage::Storage};
use url::Url;

#[tokio::test]
async fn registering_account_works() -> anyhow::Result<()> {
    let tcp = TcpListener::bind("127.0.0.1:0")?;
    let addr = tcp.local_addr()?;
    let _g = spawn_test_server(tcp);

    let server_url = Url::parse(&format!("http://{addr}"))?;

    let storage = Storage::connect_in_memory().await?;

    let client = SochatClient::new(storage);
    let _acc = client.register_account(server_url.into()).await?;

    Ok(())
}
