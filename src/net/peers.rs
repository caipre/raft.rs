extern crate byteorder;
extern crate pbuf;
extern crate protobuf;

use std::io::BufWriter;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver, Sender};

use byteorder::{LittleEndian, WriteBytesExt};

use BASE_PORT;
use CLUSTER_SIZE;

pub struct PeerStream(TcpStream);
impl PeerStream {
    pub fn trade_ids(id: &ServerId, stream: &TcpStream) -> PeerId {
        stream.write_u64<LittleEndian>(*id).unwrap();
        stream.read_u64<LittleEndian>().unwrap()
    }

    pub fn new(stream: TcpStream, ptx: Sender<T>) -> PeerStream {
        {
            let stream = stream.clone();
            thread::spawn(move|| PeerStream::read(stream, ptx));
        }
        PeerStream(stream)
    }

    fn read<T>(stream: TcpStream, ptx: Sender<T>) -> ! {
        let reader = BufReader::new(stream);
        loop {
            let msgs = pbuf::read_pbuf(reader);
            for msg in msgs {
                ptx.send(msg);
            }
        }
    }

    fn write<T>(&mut self, msg: T) {
        let writer = BufWriter::new(stream);
        writer.write_all(msg);
    }
}
