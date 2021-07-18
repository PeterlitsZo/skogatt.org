extern crate peterlits_com_server as server;

/// Main function. The entrypoint of the server.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (data, conn) = server::init().await?;
    server::run(data, conn).await?;

    Ok(())
}
