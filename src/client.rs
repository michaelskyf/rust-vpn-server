use tokio::io::{ReadHalf, WriteHalf, AsyncRead, AsyncWrite};

/*pub struct TCPClient
{
    pub rx: Box<ReadHalf>,
    pub tx: Box<WriteHalf>
}*/

pub trait Client
{
    fn rx(self) -> dyn AsyncRead + Unpin;
    fn tx(self) -> dyn AsyncRead + Unpin;
}