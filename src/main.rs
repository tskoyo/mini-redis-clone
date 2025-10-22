use anyhow::Ok;
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

type Db = HashMap<String, String>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(addr).await?;
    println!("Mini redis listening : {}", addr);

    let db: Db = HashMap::new();

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection: {}", addr);

        let db = db.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, db).await {
                println!("Error handling: {}: {:?}", addr, e);
            }
        });
    }
}

async fn handle_connection(mut socket: TcpStream, db: Db) -> anyhow::Result<()> {
    let (reader, mut writer) = socket.split();

    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes = buf_reader.read_line(&mut line).await?;
        if bytes == 0 {
            break;
        }

        let parts = line.trim().split_whitespace().collect::<Vec<_>>();
        if parts.is_empty() {
            continue;
        }

        let cmd = parts[0].to_uppercase();
        match cmd.as_str() {
            "PING" => {
                writer.write_all(b"+PONG\r\n").await?;
            }
            "GET" => {
                println!("Command is: GET");
            }
            "SET" => {
                println!("Command is: SET");
            }
            "DEL" => {
                println!("Command is: DEL");
            }
            "EXISTS" => {
                println!("Command is: EXISTS");
            }
            _ => {
                writer.write_all(b"-ERR unknown command\r\n").await?;
            }
        }

        // let trimmed = line.trim().to_uppercase();
        // if trimmed == "PING" {
        //     writer.write_all(b"+PONG\r\n").await?;
        // } else {
        //     writer.write_all(b"-ERR unknown command\r\n").await?;
        // }
    }

    Ok(())
}
