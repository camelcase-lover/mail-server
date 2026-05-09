use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

use crate::pop3::session::Pop3Session;

pub async fn start_pop3_server() -> std::io::Result<()>{

    let listener = TcpListener::bind("127.0.0.1:1100").await?;
    println!("Pop3 Server listening on 127.0.0.1:1100");

    loop{
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move{
            if let Err(err) = handle_client(socket).await{
                eprintln!("Pop3 error", err);
            }
        });
        }
}

async fn handle_client(stream: TcpStream) -> std::io::Result<()>{

    let (reader, mut writer) = stream.into_split();
    writer.write_all(b"+OK Pop3 server ready\r\n").await?;

    let mut session = Pop3Session::new();

    loop {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line).await?;

        if bytes == 0{
            break;
        }
        let command = line.trim();
        println!("CLIENT: {}", command);

        let response = session.handle_command(command).await?;

        println!("SERVER: {}", response.trim());
        writer.write_all(response.as_bytes()).await?;
    } 

    println!("Client disconnected");
    Ok(())
}