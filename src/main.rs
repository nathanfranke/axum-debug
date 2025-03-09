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
    Ok(id.to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::try_init()?;

    let router = Router::new().route("/", routing::get(get));

    let server_endpoint = dotenvy::var("SERVER_ENDPOINT")?;
    let listener = TcpListener::bind(&server_endpoint).await?;

    info!("listening on http://127.0.0.1:{}", listener.local_addr()?.port());

    axum::serve(listener, router).await?;

    Ok(())
}
