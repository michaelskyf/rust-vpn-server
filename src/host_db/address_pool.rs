use std::{net::IpAddr, collections::{VecDeque, HashMap}};

use ipnet::IpNet;

#[derive(Debug)]
pub(super) struct AddressPool
{
    hosts: HashMap<IpAddr, ()>,
    net: IpNet
}

impl AddressPool
{
    pub fn new(net: IpNet) -> Self
    {
        AddressPool
        {
            hosts: net.hosts().map(|key| (key, ())).collect(),
            net
        }
    }

    fn contains(&self, ip: &IpAddr) -> bool
    {
        self.hosts.contains_key(ip)
    }

    pub fn remove(&mut self) -> Option<IpAddr>
    {
        let ip = *self.hosts.iter().find(|_| true)?.0; // Get any available ip

        self.hosts.remove(&ip);

        Some(ip)
    }

    pub fn remove_specific(&mut self, ip: &IpAddr) -> Option<IpAddr>
    {
        let ip = *self.hosts.iter().find(|(x, _)| *x == ip)?.0;

        self.hosts.remove(&ip);

        Some(ip)
    }

    pub fn r#return(&mut self, ip: IpAddr)
    {
        if self.net.contains(&ip) == false
        {
            return;
        }

        self.hosts.insert(ip, ());
    }
}