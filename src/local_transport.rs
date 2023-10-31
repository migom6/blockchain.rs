use anyhow::Result;

use crate::transport::{NetAddr, Transport, RPC};
use std::{
    collections::HashMap,
    sync::mpsc::{channel, Receiver, Sender},
};

pub struct LocalTransport<'a> {
    addr: NetAddr,
    peers: HashMap<NetAddr, &'a LocalTransport<'a>>,
    rx: Receiver<RPC>,
    tx: Sender<RPC>,
}

impl<'a> LocalTransport<'a> {
    pub fn new(addr: NetAddr) -> Self {
        let (tx, rx) = channel();
        Self {
            addr,
            peers: HashMap::new(),
            rx,
            tx,
        }
    }
}

impl<'a> Transport<'a> for LocalTransport<'a> {
    type PeerTransport = Self;
    fn get_addr(&self) -> &NetAddr {
        &self.addr
    }

    fn connect(&mut self, transport: &'a Self::PeerTransport) -> Result<()> {
        self.peers.insert(transport.addr.clone(), transport);
        Ok(())
    }

    fn consume(&self) -> &Receiver<RPC> {
        &self.rx
    }

    fn send_message(&self, to: &NetAddr, payload: Vec<char>) -> Result<()> {
        self.peers.get(to).unwrap().tx.send(RPC {
            from: self.get_addr().clone(),
            payload,
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::LocalTransport;
    use crate::transport::{NetAddr, Transport};
    use anyhow::{Ok, Result};

    #[test]
    fn test_connect() -> Result<()> {
        let mut transport_a = LocalTransport::new(NetAddr(String::from("local-a")));
        let transport_b = LocalTransport::new(NetAddr(String::from("local-b")));
        let transport_c = LocalTransport::new(NetAddr(String::from("local-c")));

        transport_a.connect(&transport_b)?;
        transport_a.connect(&transport_c)?;

        assert_eq!(
            transport_a
                .peers
                .get(&NetAddr(String::from("local-b")))
                .unwrap()
                .addr,
            *transport_b.get_addr()
        );

        assert_ne!(
            transport_a
                .peers
                .get(&NetAddr(String::from("local-b")))
                .unwrap()
                .addr,
            *transport_c.get_addr()
        );

        Ok(())
    }

    // fn change_value(value: &mut i32) {
    //     *value = 4;
    // }
    // fn get_value(value: &i32) -> &i32 {
    //     value
    // }
    // #[test]
    // fn test_borrow() -> Result<()> {
    //     let mut a = 1;
    //     change_value(&mut a);
    //     get_value(&a);
    //     Ok(())
    // }
}
