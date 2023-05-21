#![allow(dead_code)]
#![allow(unused_imports)]

mod connection;
use connection::Handler;

mod config;
use config::Config;

pub mod host_db;
use host_db::HostDB;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;
pub type Packet = packet::ip::Packet<Vec<u8>>;