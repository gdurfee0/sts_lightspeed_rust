use std::io;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::windows::named_pipe::ServerOptions;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Create a named pipe server
    let mut server = ServerOptions::new()
        .access_inbound(true)
        .access_outbound(true)
        .first_pipe_instance(true)
        .create(r"\\.\pipe\my-pipe")?;

    println!("Named pipe server created. Waiting for a client to connect...");

    // Wait for a client to connect
    server.connect().await?;
    println!("Client connected!");

    // Read data from the client
    let mut buffer = vec![0u8; 1024];
    let bytes_read = server.read(&mut buffer).await?;
    println!(
        "Received from client: {}",
        String::from_utf8_lossy(&buffer[..bytes_read])
    );

    // Write data to the client
    let response = b"Hello from the server!";
    server.write_all(response).await?;
    println!("Sent to client: {}", String::from_utf8_lossy(response));

    // Close the connection
    server.disconnect()?;
    println!("Disconnected from client.");

    Ok(())
}
