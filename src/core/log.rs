use std::path::Path;
use tokio::net::UnixDatagram;

pub const LOG_SOCKET_PATH: &str = "/tmp/netero.log.sock";

pub async fn run_log_server() -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(LOG_SOCKET_PATH).exists() {
        std::fs::remove_file(LOG_SOCKET_PATH)?;
    }

    let socket = UnixDatagram::bind(LOG_SOCKET_PATH)?;
    let mut buf = vec![0u8; 64 * 1024];

    loop {
        let (len, _) = socket.recv_from(&mut buf).await?;
        let payload = String::from_utf8_lossy(&buf[..len]);
        println!("{}", payload);
    }
}

pub async fn send_log(kind: &str, payload: &str) {
    let socket = match UnixDatagram::unbound() {
        Ok(sock) => sock,
        Err(_) => return,
    };

    let message = format!("{}\n{}", kind, payload);
    let _ = socket.send_to(message.as_bytes(), LOG_SOCKET_PATH).await;
}
