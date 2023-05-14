use dotenv::dotenv;

mod activities;
mod worker;
mod workflows;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    worker::start_worker().await?;
    Ok(())
}
