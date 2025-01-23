use hookdeck_backend::{create_server, PORT};
use log::info;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let server = create_server();
    server.await?.await?;
    info!("Started server on localhost:{PORT}");

    Ok(())
}
