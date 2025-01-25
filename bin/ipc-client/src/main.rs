use std::io;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::windows::named_pipe::ClientOptions;

#[tokio::main]
async fn main() -> io::Result<()> {
    // The pipe name must match the one used by the server
    let pipe_name = r"\\.\pipe\my-pipe";

    println!("Attempting to connect to the named pipe server...");
    let mut client = ClientOptions::new()
        .read(true)
        .write(true)
        .open(pipe_name)?;

    // Connect to the named pipe server
    println!("Connected to the server!");

    // Write data to the server
    let message = b"Hello from the client!";
    client.write_all(message).await?;
    println!("Sent to server: {}", String::from_utf8_lossy(message));

    // Read response from the server
    let mut buffer = vec![0u8; 1024];
    let bytes_read = client.read(&mut buffer).await?;
    println!(
        "Received from server: {}",
        String::from_utf8_lossy(&buffer[..bytes_read])
    );

    Ok(())
}
