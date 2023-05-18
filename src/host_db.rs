use std::{sync::Arc, collections::HashMap, net::IpAddr};
use tokio::sync::mpsc::{Sender, self};

use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct HostDB
{
    state: Arc<RwLock<State>>
}

#[derive(Debug)]
struct State
{
    pool: AddressPool,
    map: HashMap<IpAddr, Sender<[u8; 1500]>>
}

#[derive(Debug)]
struct AddressPool
{

}

pub struct DBEntryGuard
{
    db: HostDB,
    data: IpAddr,
    /// TODO: Remove when AsyncDrops became a language feature
    dropped: bool
}

impl DBEntryGuard
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

        println!("Dropping in async_drop");
        self.dropped = true;
        self.db.state.write().await.map.remove(&self.data);
    }
}

/// TODO: This should be replaced with AsyncDrop once that becomes a feature in the language
/// Late removing of the entry in the DB may lead to invalid mpsc queues
/// Alternatively free the resources by using async_drop() on DBEntryGuard
impl Drop for DBEntryGuard
{
    fn drop(&mut self)
    {
        if self.dropped == true
        {
            return;
        }

        println!("Dropping in Drop");
        let new_guard = DBEntryGuard { db: self.db.clone(), data: self.data, dropped: self.dropped };

        tokio::spawn(async move
        {
            new_guard.async_drop().await;
        });
    }
}

impl State
{
    pub fn new() -> Self
    {
        State { pool: AddressPool {  }, map: HashMap::new() }
    }
}

impl HostDB
{
    pub fn new() -> Self
    {
        HostDB { state: Arc::new(RwLock::new(State::new())) }
    }

    pub async fn register(&mut self) -> Option<DBEntryGuard>
    {
        let (sender, _receiver) = mpsc::channel(1);
        self.state.write().await.map.insert("127.0.0.1".parse().unwrap(), sender);

        Some(DBEntryGuard { db: self.clone(), data: "127.0.0.1".parse().unwrap(), dropped: false })
    }

    /*pub async fn register_with_ip(&mut self, ip: &IpAddr) -> Option<DBEntryGuard>
    {

    }*/

    pub async fn get(&self, ip: &IpAddr) -> Option<Sender<[u8; 1500]>>
    {
        self.state.read().await.map.get(ip).cloned()
    }
}

#[cfg(test)]
mod test
{
    #[tokio::test]
    async fn db_entry_guard_late_drop()
    {
        let mut db = crate::HostDB::new();

        let guard = db.register().await.unwrap();

        drop(guard);

        tokio::task::yield_now().await;
    }

    #[tokio::test]
    async fn db_entry_guard2()
    {
        let mut db = crate::HostDB::new();

        let guard = db.register().await.unwrap();

        guard.async_drop().await;
    }
}