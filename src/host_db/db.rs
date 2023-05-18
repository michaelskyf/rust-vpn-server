use std::{collections::HashMap, net::IpAddr};
use tokio::sync::mpsc::{Sender, self};

use crate::packet::Packet;

#[derive(Debug)]
pub struct DB
{
    pool: AddressPool,
    map: HashMap<IpAddr, Sender<Packet>>
}