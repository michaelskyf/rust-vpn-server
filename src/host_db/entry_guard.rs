use std::sync::Arc;
use tokio::sync::RwLock;
use std::net::IpAddr;

use super::DB;

pub struct EntryGuard<'a>
{
    lock: &'a RwLock<DB>,
    data: IpAddr,
    /// TODO: Remove when AsyncDrops became a language feature
    dropped: bool
}

impl<'a> EntryGuard<'a>
{
    pub fn get(&self) -> IpAddr
    {
        self.data
    }

    pub async fn async_drop(mut self)
    {
        if self.dropped == true
        {
            return;
        }

        self.dropped = true;
        self.lock.write().await.unregister(&self.data);
    }
}

/// TODO: This should be replaced with AsyncDrop once that becomes a feature in the language
/// Late removing of the entry in the DB may lead to invalid mpsc queues
/// Alternatively free the resources by using async_drop() on DBEntryGuard
impl<'a> Drop for EntryGuard<'a>
{
    fn drop(&mut self)
    {
        if self.dropped == true
        {
            return;
        }

        let new_guard = EntryGuard { lock: self.lock, data: self.data, dropped: self.dropped };

        tokio::spawn(async move
        {
            new_guard.async_drop().await;
        });
    }
}

/*
/// Implementation for AsyncDrop
impl AsyncDrop for EntryGuard
{
    async fn drop(&mut self)
    {
        self.lock.write().await.unregister(&self.data);
    }
}
*/