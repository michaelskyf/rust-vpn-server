use std::net::IpAddr;

use tokio::{io::{AsyncRead, AsyncWrite}, sync::mpsc::Receiver};
use crate::{HostDB, Config, Result, packet::Packet, Error, host_db::entry_guard::EntryGuard};

pub(crate) struct Handler<'a, Stream>
where
    Stream: AsyncRead + AsyncWrite
{
    stream: Stream,
    mq_rx: Receiver<Packet>,
    ip_guard: EntryGuard<'a>
}

impl<'a, Stream> Handler<'a, Stream>
where
    Stream: AsyncRead + AsyncWrite
{
    pub async fn run(stream: Stream, mut db: HostDB) -> Result<()>
    {
        let (ip_guard, mq_rx) = db.register().await.ok_or::<Error>("Failed to register the connection".into())?;

        let data = Handler { stream, mq_rx, ip_guard };

        data.init().await?;

        data.handle().await?;
        
        // TODO: Remove when AsyncDrop becomes a part of the language
        data.ip_guard.async_drop().await;

        Ok(())
    }

    async fn init(&self) -> Result<()>
    {
        Ok(())
    }

    async fn handle(&self) -> Result<()>
    {
        Ok(())
    }
}