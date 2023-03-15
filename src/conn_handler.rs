use tokio::io::{AsyncRead, AsyncWrite};
use crate::Connection;
use crate::conn_db::ConnDb;
use crate::Result;

pub(crate) struct Handler<Stream>
where
    Stream: AsyncRead + AsyncWrite + Sized + Unpin
{
    /// TCP, UDP or TUN connection
    connection: Connection<Stream>,

    /// Database containing all active vpn connections
    conn_db: ConnDb,
}

impl<Stream> Handler<Stream>
where
    Stream: AsyncRead + AsyncWrite + Sized + Unpin
{
    pub(crate) fn new(connection: Connection<Stream>, conn_db: ConnDb) -> Handler<Stream>
    {
        Handler
        {
            connection,
            conn_db
        }
    }

    pub(crate) fn run(&mut self) -> Result<()>
    {
        todo!()
    }
}