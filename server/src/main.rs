use tokio::{fs::File, io::AsyncWriteExt, net::TcpListener};
use std::error::Error;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Сервер запущен на 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buffer = vec![0u8; 1024];
            let n = match socket.read(&mut buffer).await {
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(_) => return,
            };

            let received = &buffer[..n];
            let content = String::from_utf8_lossy(received);
            let filename = format!("uploaded_{}.txt", chrono::Utc::now().timestamp());

            let mut file = File::create(&filename).await.unwrap();
            file.write_all(received).await.unwrap();

            let (lines, words, chars) = analyze(&content);

            let response = format!(
                "Имя файла: {}\nСтрок: {}\nСлов: {}\nСимволов: {}",
                filename, lines, words, chars
            );

            let _ = socket.write_all(response.as_bytes()).await;
        });
    }
}

fn analyze(content: &str) -> (usize, usize, usize) {
    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let chars = content.chars().count();
    (lines, words, chars)
}
