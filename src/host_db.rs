use std::sync::Arc;
use ipnet::IpNet;
use tokio::sync::mpsc::{Sender, self};

mod db;
mod entry_guard;
mod address_pool;
use self::{entry_guard::EntryGuard, db::DB};

use crate::Packet;
use std::net::IpAddr;

use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct HostDB
{
    lock: Arc<RwLock<DB>>
}

impl HostDB
{
    pub fn new(net: &IpNet) -> Self
    {
        HostDB
        {
            lock: Arc::new(RwLock::new(DB::new(net)))
        }
    }

    pub async fn register(&mut self) -> Option<EntryGuard>
    {
        match self.lock.write().await.register()
        {
            Some(ip) =>
            {
                Some(EntryGuard { lock: &self.lock, data: ip })
            },
            None =>
            {
                None
            }
        }
    }

    pub async fn register_with_ip(&mut self, ip: &IpAddr) -> Option<EntryGuard>
    {
        match self.lock.write().await.register_with_ip(ip)
        {
            Some(ip) =>
            {
                Some(EntryGuard { lock: &self.lock, data: ip })
            },
            None =>
            {
                None
            }
        }
    }

    pub async fn get(&self, ip: &IpAddr) -> Option<Sender<Packet>>
    {
        self.lock.read().await.get(ip)
    }
}