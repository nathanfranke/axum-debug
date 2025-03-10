mod fallible;

use axum::{
    routing,
    Router,
};
use log::info;
use uuid::Uuid;
use tokio::net::TcpListener;

use self::fallible::*;

async fn get() -> Result<String> {
    let id = Uuid::new_v4();
    info!("id: {id}");
    Ok(format!("{id}\n"))
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::builder().filter_level(log::LevelFilter::Info).try_init()?;

    let server_endpoint = dotenvy::var("SERVER_ENDPOINT")?;

    let router = Router::new().route("/", routing::get(get));

    let listener = TcpListener::bind(&server_endpoint).await?;
    info!("listening at http://{}", listener.local_addr()?);

    axum::serve(listener, router).await?;

    Ok(())
}
