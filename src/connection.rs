use std::{marker::PhantomData, str::from_utf8};

use tokio::{io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt, split}, sync::{mpsc::Receiver, Mutex}};
use crate::{HostDB, Config, Result, packet::Packet, Error, host_db::entry_guard::EntryGuard};

pub(crate) struct Handler<'a>
{
    mq_rx: Receiver<Packet>,
    ip_guard: EntryGuard<'a>
}

impl<'a> Handler<'a>
{
    pub async fn new<Stream: AsyncRead + AsyncWrite + Unpin + Send>(stream: Stream, mut db: HostDB) -> Result<()>
    {
        let (ip_guard, mq_rx) = db.register().await.ok_or::<Error>("Failed to register the connection".into())?;

        let data = Handler { mq_rx, ip_guard };

        let (mut rx, mut tx) = split(stream);

        // TODO: Handle errors
        let (_, _) = tokio::join!(
            Self::handle_incoming(&mut rx),
            Self::handle_outgoing(&mut tx));

        // TODO: Remove when AsyncDrop becomes a part of the language
        data.ip_guard.async_drop().await;

        Ok(())
    }

    async fn handle_incoming<R: AsyncReadExt + Unpin>(stream: &mut R) -> Result<()>
    {
        let mut buf = [0u8; 1500];
        let read = stream.read(&mut buf).await.map_err(|e| e.to_string())?;

        println!("{}", from_utf8(&buf[0..read]).unwrap());

        Ok(())
    }

    async fn handle_outgoing<W: AsyncWrite + Unpin>(stream: &mut W) -> Result<()>
    {
        stream.write(b"Hello from handle_outgoing()!").await.map_err(|e| e.to_string())?;

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