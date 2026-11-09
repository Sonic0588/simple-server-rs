use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 1. bind
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    // 2. loop + accept
    loop {
        let (mut socket, _) = listener.accept().await?;
        
        // 3. spawn задачу на клиента
        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            loop {
                let bytes_read = socket.read(&mut buf).await?;

                if bytes_read == 0 {
                    break;
                }

                socket.write_all(&buf[..bytes_read]).await?;
            }

        Ok::<_, std::io::Error>(())
        });
    }
}
