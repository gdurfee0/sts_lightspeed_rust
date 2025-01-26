use tokio::io::AsyncReadExt;
use tokio::net::windows::named_pipe::ServerOptions;

const STS_TO_TELEMETRY_PIPE_NAME: &str = "\\\\.\\pipe\\sts2telemetry";
//const TELEMETRY_TO_STS_PIPE_NAME: &str = r"\\.\pipe\telemetry-to-sts";

async fn handle_incoming_messages() {
    let mut sts_to_telemetry_pipe = ServerOptions::new()
        .access_inbound(true)
        .access_outbound(true)
        .first_pipe_instance(true)
        .create(STS_TO_TELEMETRY_PIPE_NAME)
        .expect("Failed to create named pipe");
    println!("Waiting for STS to connect to telemetry named pipe.");
    match sts_to_telemetry_pipe.connect().await {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to connect to STS named pipe: {}", e);
            return;
        }
    }
    loop {
        // Read a string from the named pipe
        let mut buffer = vec![0u8; 1024];
        let bytes_read = match sts_to_telemetry_pipe.read(&mut buffer).await {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(bytes_read) => bytes_read,
            Err(e) => {
                println!("Failed to read from client: {}", e);
                break;
            }
        };
        println!(
            "Received from client: {}",
            String::from_utf8_lossy(&buffer[..bytes_read])
        );
    }
}

fn main() {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    rt.block_on(handle_incoming_messages());
}
