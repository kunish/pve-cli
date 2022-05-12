use anyhow::Result;
use dotenv::dotenv;

mod api;
mod app;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    if cfg!(debug_assertions) {
        dotenv().ok();
    }

    app::run().await?;

    Ok(())
}
