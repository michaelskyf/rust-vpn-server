use tokio::sync::RwLock;
use std::net::IpAddr;

use super::DB;

pub struct EntryGuard<'a>
{
    pub(super) lock: &'a RwLock<DB>,
    pub(super) data: IpAddr
}

impl<'a> EntryGuard<'a>
{
    pub fn get(&self) -> IpAddr
    {
        self.data
    }

    /// TODO: Replace with AsyncDrop in the future
    pub async fn async_drop(self)
    {
        self.lock.write().await.unregister(&self.data);
    }
}

/*
/// Implementation for AsyncDrop once it drops in the language
impl<'a> AsyncDrop for EntryGuard<'a>
{
    async fn drop(&mut self)
    {
        self.lock.write().await.unregister(&self.data);
    }
}
*/

#[cfg(test)]
mod test
{
    use ipnet::IpNet;

    use crate::HostDB;

    #[tokio::test]
    async fn test()
    {
        let mut db = HostDB::new(&IpNet::new("192.168.1.0".parse().unwrap(), 24).unwrap());
        let guard = db.register().await.unwrap();
        
        let data = guard.get();
        guard.async_drop().await;
        
    }
}