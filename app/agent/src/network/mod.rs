mod client;
mod connection;
mod handshake;
mod multiplex;
mod stream;

pub use client::Client;
pub(crate) use connection::{Connection, ReadHalf, WriteHalf};
use handshake::perform_handshake;
pub use multiplex::MultiplexManager;
