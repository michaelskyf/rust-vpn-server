use std::{collections::HashMap, net::IpAddr};
use ipnet::IpNet;
use tokio::sync::mpsc::{Sender, self, Receiver};

use crate::Packet;

use super::address_pool::AddressPool;
use super::entry_guard::EntryGuard;

#[derive(Debug)]
pub(super) struct DB
{
    pool: AddressPool,
    map: HashMap<IpAddr, Sender<Packet>>
}

impl DB
{
    pub fn new(net: IpNet) -> Self
    {
        DB
        {
            pool: AddressPool::new(net),
            map: Default::default()
        }
    }

    pub fn register_with_ip(&mut self, ip: &IpAddr) -> Option<(IpAddr, Receiver<Packet>)>
    {
        let ip = self.pool.remove_specific(ip)?;
        let (tx, rx) = mpsc::channel(128); // TODO: Make the queue user-configurable
        self.map.insert(ip, tx);

        Some((ip, rx))
    }

    pub fn register(&mut self) -> Option<(IpAddr, Receiver<Packet>)>
    {
        let ip = self.pool.remove()?;
        let (tx, rx) = mpsc::channel(128); // TODO: Make the queue user-configurable
        self.map.insert(ip, tx);

        Some((ip, rx))
    }

    pub fn unregister(&mut self, ip: IpAddr)
    {
        self.map.remove(&ip);
        self.pool.insert(ip);
    }

    pub fn get(&self, ip: &IpAddr) -> Option<Sender<Packet>>
    {
        self.map.get(ip).cloned()
    }

    /// Returns number of used IPs in the database
    pub fn len(&self) -> usize
    {
        self.map.len()
    }
    
}