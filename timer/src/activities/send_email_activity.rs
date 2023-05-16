use log::info;
use temporal_sdk::ActContext;

pub async fn send_email_activity(
    _ctx: ActContext,
    _payload: Option<String>,
) -> Result<(), anyhow::Error> {
    info!("Sending notification email as the process takes too long.");
    Ok(())
}
