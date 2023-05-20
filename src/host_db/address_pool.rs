use std::{net::IpAddr, collections::VecDeque};

use ipnet::IpNet;

#[derive(Debug)]
pub(super) struct AddressPool
{
    queue: VecDeque<IpAddr>,
    net: IpNet
}

impl AddressPool
{
    pub fn new(net: IpNet) -> Self
    {
        AddressPool
        {
            queue: net.hosts().collect(),
            net
        }
    }

    fn available(&self, ip: &IpAddr) -> bool
    {
        self.queue.contains(ip)
    }

    pub fn remove(&mut self) -> Option<IpAddr>
    {
        self.queue.pop_back()
    }

    pub fn remove_specific(&mut self, ip: &IpAddr) -> Option<IpAddr>
    {
        if self.available(ip) == false
        {
            return None;
        }

        let index = self.queue.iter().position(|x| x == ip)?;

        self.queue.remove(index)
    }

    pub fn r#return(&mut self, ip: IpAddr)
    {
        if self.net.contains(&ip) == false
        {
            return;
        }

        self.queue.push_back(ip);
    }
}