use std::str::FromStr;
use temporal_client::{Client, RetryClient};
use temporal_sdk::sdk_client_options;
use temporal_sdk_core::Url;

pub async fn get_client() -> Result<RetryClient<Client>, anyhow::Error> {
    let server_options = sdk_client_options(Url::from_str("http://localhost:7233")?).build()?;

    let client = server_options.connect("default", None, None).await?;

    Ok(client)
}
