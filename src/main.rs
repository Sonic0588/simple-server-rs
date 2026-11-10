use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

fn handle_line(line: &str, out: &mut String) {
    let line = line.trim_end();
    let resp = if let Some((cmd, text)) = line.split_once(' ') {
        match cmd {
            "ECHO" => text,
            "UPPER" => &text.to_uppercase(),
            _ => "ERR unknown command",
        }
    } else {
        match line {
            "PING" => "PONG",
            _ => "ERR unknown command",
        }
    };

    out.push_str(resp);
    out.push('\n');
}

async fn handle_client(mut socket: TcpStream) -> std::io::Result<()> {
    let (recv, mut tx) = socket.split();
    let mut reader = BufReader::new(recv);
    let mut buffer = String::new();
    let mut response = String::new();
    loop {
        buffer.clear();
        response.clear();
        let bytes_read = reader.read_line(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }

        handle_line(&buffer, &mut response);
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_line_ping() {
        let mut out = String::new();
        handle_line("PING\r\n", &mut out);
        assert_eq!(out, "PONG\n");
    }

    #[test]
    fn test_handle_line_ping_lowercase() {
        let mut out = String::new();
        handle_line("ping", &mut out);
        assert_eq!(out, "ERR unknown command\n");
    }

    #[test]
    fn test_handle_line_echo() {
        let mut out = String::new();
        handle_line("ECHO foo", &mut out);
        assert_eq!(out, "foo\n");
    }

    #[test]
    fn test_handle_line_echo_blank() {
        let mut out = String::new();
        handle_line("ECHO", &mut out);
        assert_eq!(out, "ERR unknown command\n");
    }

    #[test]
    fn test_handle_line_echo_trip_end() {
        let mut out = String::new();
        handle_line("ECHO ", &mut out);
        assert_eq!(out, "ERR unknown command\n");
    }

    #[test]
    fn test_handle_line_echo_a_b() {
        let mut out = String::new();
        handle_line("ECHO a b", &mut out);
        assert_eq!(out, "a b\n");
    }

    #[test]
    fn test_handle_line_upper() {
        let mut out = String::new();
        handle_line("UPPER foo", &mut out);
        assert_eq!(out, "FOO\n");
    }
}