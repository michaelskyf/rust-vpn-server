use std::{collections::HashMap, net::IpAddr};
use ipnet::IpNet;
use tokio::sync::mpsc::{Sender, self};

use crate::packet::Packet;

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
    pub fn new(net: &IpNet) -> Self
    {
        todo!()
    }

    pub fn register_with_ip(&mut self, ip: &IpAddr) -> Option<IpAddr>
    {
        todo!()
    }

    pub fn register(&mut self) -> Option<IpAddr>
    {
        todo!()
    }

    pub fn unregister(&mut self, ip: &IpAddr)
    {
        self.map.remove(ip);
        todo!()
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