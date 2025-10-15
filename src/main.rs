use std::fmt::write;

use anyhow::Ok;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufStream};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(addr).await?;
    println!("Mini redis listening : {}", addr);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection: {}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                println!("Error handling: {}: {:?}", addr, e);
            }
        });
    }
}

async fn handle_connection(mut socket: TcpStream) -> anyhow::Result<()> {
    let (reader, mut writer) = socket.split();
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes = buf_reader.read_line(&mut line).await?;
        if bytes == 0 {
            break;
        }

        let trimmed = line.trim().to_uppercase();
        if trimmed == "PING" {
            writer.write_all(b"+PONG\r\n").await?;
        } else {
            writer.write_all(b"-ERR unknown command\r\n").await?;
        }
    }

    Ok(())
}
