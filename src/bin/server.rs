extern crate raft;

use std::thread;
use std::sync::mpsc::{self, Receiver, Sender};

use raft::*;

fn main() {
    let mut server = RaftServer::new();

    // start rafting
    thread::spawn(|| ConnectionHandler::accept(&server));
    thread::spawn(move|| server.raftstart());


    // accept client commands
    thread::spawn(|| ConnectionHandler::accept(&server));

    server.run();
}
