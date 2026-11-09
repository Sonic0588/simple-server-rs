use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

fn handle_line(line: &str) -> String {
    if let Some((cmd, text)) = line.trim_end().split_once(' ') {
        match cmd {
            "ECHO" => text.to_string(),
            "UPPER" => text.to_uppercase().to_string(),
            _ => "ERR unknown command".to_string(),
        }
    } else {
        match line.trim_end() {
            "PING" => "PONG".to_string(),
            _ => "ERR unknown command".to_string(),
        }
    }
}

async fn handle_client(mut socket: TcpStream) -> std::io::Result<()> {
    let (tx, mut rx) = socket.split();
    let mut reader = BufReader::new(tx);
    let mut buffer = String::new();
    loop {
        let bytes_read = reader.read_line(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }

        let response = handle_line(&buffer);
        buffer.clear();

        rx.write_all(response.as_bytes()).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 1. bind
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    // 2. loop + accept
    loop {
        let (socket, _) = listener.accept().await?;

        // 3. spawn задачу на клиента
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Ошибка при обработке клиента: {}", e);
            }
        });
    }
}
