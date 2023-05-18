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
    data: IpAddr
}

impl DBEntryGuard
{
    pub fn new(db: HostDB, data: IpAddr) -> Self
    {
        DBEntryGuard
        {
            db,
            data
        }
    }

    pub fn get(&self) -> IpAddr
    {
        self.data
    }
}

impl Drop for DBEntryGuard
{
    fn drop(&mut self)
    {
        let db = self.db.clone();
        let data = self.data;
        tokio::spawn(async move
        {
            println!("Dropping: {}", db.state.read().await.map.len());
            db.state.write().await.map.remove(&data);
            println!("{}", db.state.read().await.map.len());
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

        Some(DBEntryGuard { db: self.clone(), data: "127.0.0.1".parse().unwrap() })
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
    async fn db_entry_guard()
    {
        let mut db = crate::HostDB::new();

        let guard = db.register().await.unwrap();

        println!("{}", db.state.read().await.map.len());

        drop(guard);

        println!("{}", db.state.read().await.map.len());

        tokio::task::yield_now().await;
    }
}