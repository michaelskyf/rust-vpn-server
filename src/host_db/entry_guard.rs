use crate::HostDB;
use std::net::IpAddr;

pub struct EntryGuard
{
    db: HostDB,
    data: IpAddr,
    /// TODO: Remove when AsyncDrops became a language feature
    dropped: bool
}

impl EntryGuard
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
        self.db.state.write().await.map.remove(&self.data);
    }
}

/// TODO: This should be replaced with AsyncDrop once that becomes a feature in the language
/// Late removing of the entry in the DB may lead to invalid mpsc queues
/// Alternatively free the resources by using async_drop() on DBEntryGuard
impl Drop for EntryGuard
{
    fn drop(&mut self)
    {
        if self.dropped == true
        {
            return;
        }

        let new_guard = EntryGuard { db: self.db.clone(), data: self.data, dropped: self.dropped };

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
        new_guard.async_drop().await;
    }
}
*/