#![allow(dead_code)]
#![allow(unused_imports)]

mod connection;
pub use connection::Connection;

mod conn_handler;
use conn_handler::Handler;

pub mod conn_db;
use conn_db::ConnDb;

mod shutdown;
use shutdown::Shutdown;

pub mod server;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;