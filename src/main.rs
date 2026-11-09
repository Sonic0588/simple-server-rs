use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

fn handle_line(line: &str) -> String {
    let line = line.trim_end();
    if let Some((cmd, text)) = line.split_once(' ') {
        match cmd {
            "ECHO" => text.to_string(),
            "UPPER" => text.to_uppercase(),
            _ => "ERR unknown command".to_string(),
        }
    } else {
        match line {
            "PING" => "PONG".to_string(),
            _ => "ERR unknown command".to_string(),
        }
    }
}

async fn handle_client(mut socket: TcpStream) -> std::io::Result<()> {
    let (recv, mut tx) = socket.split();
    let mut reader = BufReader::new(recv);
    let mut buffer = String::new();
    loop {
        buffer.clear();
        let bytes_read = reader.read_line(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }

        let mut response = handle_line(&buffer);
        response.push('\n');

        tx.write_all(response.as_bytes()).await?;
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
