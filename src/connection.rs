use tokio::io::{AsyncRead, AsyncWrite};
use crate::{HostDB, Config, Result};
use crate::host_db::DBEntryGuard;

pub(crate) struct Handler<Stream>
where
    Stream: AsyncRead + AsyncWrite
{
    stream: Stream,
    db: HostDB
}