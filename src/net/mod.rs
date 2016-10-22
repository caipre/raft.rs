use std::net::TcpListener;

pub struct ConnectionHandler<T: StreamHandler>;
impl<T> ConnectionHandler<T> where T: StreamHandler {
    pub fn accept(handler: &T) {
        ConnectionHandler::peerscan(&handler);
        let listener = ConnectionHandler::portscan();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    debug!("new connection from {}", peerport!(stream));
                    handler.take(stream);
                },
                Err(e) => panic!("{}", e),
            }
        }
    }

    fn peerscan(handler: &T) {
        (BASE_PORT..)
            .take(CLUSTER_SIZE)
            .filter_map(|port| TcpStream::connect(bindaddr!(port)).ok())
            .inspect(|stream| debug!("found peer at {}", peerport!(stream)))
            .foreach(|stream| handler.take(stream));
    }

    fn portscan() -> TcpListener {
        (BASE_PORT..)
            .map(|port| TcpListener::bind(bindaddr!(port)))
            .find(|socket| socket.is_ok())
            .inspect(|socket| debug!("bound to port {}", localport!(socket)))
            .expect("no sockets available")
            .unwrap()
    }
}

trait StreamHandler {
    fn take(&self, stream: TcpStream);
}

macro_rules! bindaddr
    { ($port:expr) => { format!("127.0.0.1:{}", $port).as_str() } }

macro_rules! localport
    { ($socket:expr) => { $socket.local_addr.map(|addr| addr.port()).unwrap() } }

macro_rules! peerport
    { ($socket:expr) => { $socket.peer_addr.map(|addr| addr.port()).unwrap() } }
