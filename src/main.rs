use dotenv::dotenv;

mod api;
mod app;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    app::run().await?;

    Ok(())
}
