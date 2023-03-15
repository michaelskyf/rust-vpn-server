use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::Result;

#[derive(Debug)]
pub struct Connection<Stream>
where
    Stream: AsyncReadExt + AsyncWriteExt + Sized + Unpin
{
    stream: Stream

    // rxbuf?
}

impl<Stream> Connection<Stream>
where
    Stream: AsyncReadExt + AsyncWriteExt + Sized + Unpin
{
    fn new(stream: Stream) -> Connection<Stream>
    {
        Connection
        {
            stream
        }
    }

    async fn read_packet(&mut self, buf: &mut [u8]) -> Result<usize>
    {
        Ok(self.stream.read(buf).await?)
    }

    async fn write_packet(&mut self, data: &[u8]) -> Result<()>
    {
        Ok(self.stream.write_all(&data).await?)
    }
}

#[cfg(test)]
mod test
{
    use tokio_test::io::Mock;
    use crate::Connection;

    #[tokio::test]
    async fn read_packet_normal()
    {
        let mut buf = [0_u8; 1500];
        const MSG: &[u8] = b"Hello, Cruel World!";

        let mut builder = tokio_test::io::Builder::new();
        let mock = builder.read(MSG).build();
        let mut conn = Connection::new(mock);

        let result = conn.read_packet(&mut buf).await.unwrap();

        assert_eq!(&buf[0..result], MSG);
    }

    #[tokio::test]
    async fn read_packet_small_buffer()
    {
        let mut buf = [0_u8; 10];
        const MSG: &[u8] = b"Hello, Cruel World!";

        let mut builder = tokio_test::io::Builder::new();
        let mock = builder.read(MSG).build();
        let mut conn = Connection::new(mock);

        let result = conn.read_packet(&mut buf).await.unwrap();
        assert_eq!(&buf[0..result], &MSG[0..10]);

        let result = conn.read_packet(&mut buf).await.unwrap();
        assert_eq!(&buf[0..result], &MSG[10..19]);

    }

    #[tokio::test]
    async fn write_packet_normal()
    {
        const MSG: &[u8] = b"Hello, Cruel World!";

        let mut builder = tokio_test::io::Builder::new();
        let mock = builder.write(MSG).build();
        let mut conn = Connection::new(mock);

        conn.write_packet(MSG).await.unwrap();
    }
}

