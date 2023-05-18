use std::sync::Arc;
use ipnet::IpNet;

mod entry_guard;
use crate::host_db::entry_guard::EntryGuard;

mod db;
use crate::host_db::db::DB;

use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct HostDB
{
    db: Arc<RwLock<DB>>
}

impl HostDB
{
    pub fn new(net: &IpNet) -> Self
    {
        HostDB
        {
            db: Arc::new(RwLock::new(DB::new(net)))
        }
    }

    pub async fn register(&mut self) -> Option<EntryGuard>
    {

    }

    pub async fn register_with_ip(&mut self, ip: &IpAddr) -> Option<EntryGuard>
    {

    }

    pub async fn get(&self, ip: &IpAddr) -> Option<Sender<[u8; 1500]>>
    {

    }
}