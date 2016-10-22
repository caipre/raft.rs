use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver, Sender};

pub struct ClientHandler {
    tx: Sender<ClientCommand>,
}

impl ClientHandler {
    pub fn new() -> (ClientHandler, Receiver<ClientCommand>) {
        let (tx, rx) = mpsc::channel();
        let mut handler = ClientHandler { tx: tx };
        (handler, rx)
    }
}

impl StreamHandler for ClientHandler {
    fn add(&self, stream: TcpStream) {
        let reader = BufReader::new(stream);
        let mut buf = String::new();
        reader.read_line(&mut buf);
        println!("{}", buf);
        tx.send(ClientCommand::Nop);
    }
}
