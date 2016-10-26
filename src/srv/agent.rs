extern crate byteorder;

use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc::{Sender, Receiver, RecvError};

use byteorder::{LittleEndian, WriteBytesExt};

struct Agent(Peer);

//// peer tracking

struct Peer {
    id: PeerId,
    stream: PeerStream,
    position: Option<Position>,
}

struct Position {
    nindex: usize,
    mindex: usize,
}

impl Agent {
    fn new(id: &ServerId, stream: TcpStream, mtx: Sender<RaftMessage>) -> Agent {
        let peerid = PeerStream::trade_ids(id, &stream);

        let (ptx, prx) = mpsc::channel();
        let peerstream = PeerStream::new(stream, ptx);

        let peer =
            Peer { id: peerid, stream: peerstream, position: None };

        thread::spawn(move|| Agent::recv(prx, mtx));

        Agent(peer)
    }

    fn recv(prx: Receiver<ExchangeType>, mtx: Sender<(Agent, RaftMessage)>) {
        loop {
            match prx.recv() {
                Ok(proto) => mtx.send((&self, proto.into())),
                Err(RecvError) => panic!("sender disconnected"),
            }
        }
    }


}
