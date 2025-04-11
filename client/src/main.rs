use std::{env, fs, io::Result};
use tokio::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Использование: client <имя_файла>");
        return Ok(());
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename)?;

    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    stream.write_all(content.as_bytes()).await?;

    let mut buffer = vec![0u8; 1024];
    let n = stream.read(&mut buffer).await?;
    println!("Ответ от сервера:\n{}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}
