#[derive(Debug)]
pub enum Command {
    Ping,
    Set { key: String, value: String },
    Get { key: String },
    Del { keys: Vec<String> },
    Exists { key: String },
}

impl Command {
    pub fn from_line(line: String) -> Result<Self, String> {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        if parts.is_empty() {
            return Err("No keys provided!".to_owned());
        }

        let cmd = parts[0].to_uppercase();

        match cmd.as_str() {
            "PING" => Ok(Command::Ping),
            "SET" if parts.len() >= 3 => {
                let key = parts[1].to_owned();
                let value = parts[2..].join(" ");

                Ok(Command::Set { key, value })
            }
            "GET" => {
                let key = parts[1].to_owned();
                Ok(Command::Get { key })
            }
            "DEL" if parts.len() >= 2 => {
                let keys = parts[1..].iter().map(|&key| key.to_owned()).collect();

                Ok(Command::Del { keys })
            }
            "EXISTS" if parts.len() == 2 => {
                let key = parts[1].to_owned();

                Ok(Command::Exists { key })
            }
            _ => Err("Invalid command!".to_owned()),
        }
    }
}
