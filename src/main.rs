#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unreachable_code)]

use std::net::Ipv4Addr;
use tokio::net::{TcpListener, TcpStream};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io::Result;
use tokio::net::tcp::ReadHalf;
use std::str::{self, from_utf8};

/*async fn client_handler(mut rx: ReadHalf<'_>) -> Result<()>
{
    loop
    {
        let mut buf = [0_u8; 1024];
        let read = rx.read(&mut buf).await?;

        println!("Message: {:?}", &buf[0..read]);
    }

    Ok(())
} */

async fn client_handler(mut socket: TcpStream) -> Result<()>
{
    loop
    {
        socket.write(b"Hello, World!\n").await?;

        let mut buf = [0_u8; 1024];
        let read = socket.read(&mut buf).await?;

        println!("Message: {}", from_utf8(&buf[0..read]).unwrap());
    }

    Ok(())
}

async fn listener_loop(listener: TcpListener) -> Result<()>
{
    println!("Loop!");
    loop
    {
        let (socket, _) = listener.accept().await?;
        println!("Client connected!");

        tokio::spawn(async move { client_handler(socket).await }); 
    }

    Ok(())
}

#[tokio::main]
async fn main()
{
    
    let addresses = vec!["127.0.0.1:1234", "127.0.0.1:12345", "127.0.0.1:12346"];

    for address in addresses
    {
        let listener = TcpListener::bind(address).await.unwrap();
        tokio::spawn(async move { listener_loop(listener).await });
    }

    tokio::join!();
}