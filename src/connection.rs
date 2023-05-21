use std::{marker::PhantomData, str::from_utf8};

use tokio::{io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt, split}, sync::{mpsc::Receiver, Mutex}};
use crate::{HostDB, Config, Result, Packet, Error, host_db::entry_guard::EntryGuard};

pub(crate) struct Handler
{

}

impl Handler
{
    pub async fn new<Stream: AsyncRead + AsyncWrite + Unpin>(stream: Stream, db: HostDB) -> Result<()>
    {
        let mut db_clone = db.clone();
        let (ip_guard, mut mq_rx) = db_clone.register().await.ok_or::<Error>("Failed to register the connection".into())?;

        let (mut rx, mut tx) = split(stream);

        // TODO: Handle errors
        let (_, _) = tokio::join!(
            Self::handle_incoming(&mut rx, db.clone()),
            Self::handle_outgoing(&mut tx, &mut mq_rx));

        // TODO: Remove when AsyncDrop becomes a part of the language
        ip_guard.async_drop().await;

        Ok(())
    }

    async fn handle_incoming<R: AsyncReadExt + Unpin>(stream: &mut R, db: HostDB) -> Result<()>
    {
        let mut buf = [0u8; 1500];
        let read = stream.read(&mut buf).await.map_err(|e| e.to_string())?;

        println!("{}", from_utf8(&buf[0..read]).unwrap());

        Ok(())
    }

    async fn handle_outgoing<W: AsyncWrite + Unpin>(stream: &mut W, mq: &mut Receiver<Packet>) -> Result<()>
    {
        stream.write(b"Hello from handle_outgoing()!").await.map_err(|e| e.to_string())?;
        
        loop
        {
            tokio::select!
            {
                Some(packet) = mq.recv() =>
                {
                    stream.write(packet.as_ref()).await.map_err(|e| e.to_string())?;
                },
                else => break
            }
        }

        Ok(())
    }

    async fn init(&self) -> Result<()>
    {
        Ok(())
    }

}

#[cfg(test)]
mod test
{
    use ipnet::IpNet;

    use crate::HostDB;
    use crate::connection::Handler;

    #[tokio::test]
    async fn test()
    {
        let db = HostDB::new(IpNet::new("192.168.1.0".parse().unwrap(), 24).unwrap());
        let mock_stream = tokio_test::io::Builder::new()
            .write(b"Hello from handle_outgoing()!")
            .read(b"Hello from mock! (read)")
            .build();

        let handler = Handler::new(mock_stream, db);

        handler.await.unwrap();
    }
}