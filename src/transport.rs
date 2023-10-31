use anyhow::Result;
use std::sync::mpsc::Receiver;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct NetAddr(pub String);

pub struct RPC {
    pub from: NetAddr,
    pub payload: Vec<char>,
}

pub trait Transport<'a> {
    type PeerTransport;
    fn consume(&self) -> &Receiver<RPC>;
    fn connect(&mut self, transport: &'a Self::PeerTransport) -> Result<()>;
    fn send_message(&self, to: &NetAddr, payload: Vec<char>) -> Result<()>;
    fn get_addr(&self) -> &NetAddr;
}
