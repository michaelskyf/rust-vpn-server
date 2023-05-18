use tokio::io::{AsyncRead, AsyncWrite};
use crate::{HostDB, Config, Result};

pub(crate) struct Handler<Stream>
where
    Stream: AsyncRead + AsyncWrite
{
    stream: Stream,
    db: HostDB
}