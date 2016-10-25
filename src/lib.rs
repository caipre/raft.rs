#![feature(mpsc_select)]

#[macro_use]
extern crate log;
extern crate rand;

#[macro_use]
mod macros;

mod timer;

use std::cmp::{self, Ordering};
use std::default::Default;
use std::fmt;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use std::sync::RwLock;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use rand::Rng;

use timer::Timer;

pub fn start(port: u16, peers: Vec<String>) {
    let mut server = RaftServer::new();
    RaftServer::start(&mut server, port, peers);
}

//// server

struct RaftServer {
    identity: Identity,
    election: Election,
    peers: Peers,
    log: RaftLog,
}

impl RaftServer {
    fn new() -> RwLock<RaftServer> {
        RwLock::new(
            RaftServer {
                identity: Identity::new(),
                election: Election::new(),
                peers: Peers::new(),
                log: RaftLog::new(),
            }
        )
    }
}

//// identity

#[derive(Debug)]
struct Identity {
    id: ServerId,
    role: Role,
    leader: Option<ServerId>,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
struct ServerId(u64);

impl ServerId {
    fn new() -> ServerId {
        ServerId(rand::random())
    }
}

impl From<u64> for ServerId {
    fn from(id: u64) -> ServerId {
        ServerId(id)
    }
}

impl FromStr for ServerId {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<ServerId, Self::Err> {
        let id = try!(s.parse());
        Ok(ServerId(id))
    }
}

impl fmt::Debug for ServerId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", (self.0 & 0xffffff))
    }
}

impl fmt::Display for ServerId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
enum Role {
    Leader,
    Candidate,
    Follower,
}

impl Identity {
    fn new() -> Identity {
        Identity {
            id: ServerId::new(),
            role: Role::Follower,
            leader: None,
        }
    }
}

//// election

struct Election {
    term: usize,
    timer: Timer,
    voted_for: Option<ServerId>,
    votes: u8,
}

impl Election {
    fn new() -> Election {
        let timeout = {
            let mut rng = rand::thread_rng();
            rng.gen_range(4000, 5000)
        };

        Election {
            term: 0,
            timer: Timer::from_millis(timeout),
            voted_for: None,
            votes: 0,
        }
    }
}

//// peers

struct Peers {
    peers: Vec<Peer>,
    socket: Option<UdpSocket>,
}

struct Peer {
    id: Option<ServerId>,
    addr: SocketAddr,
    positions: Option<Positions>,
    sent: Option<Positions>,
}

struct Positions {
    nindex: usize,
    mindex: usize,
}

impl Peers {
    fn new() -> Peers {
        Peers {
            peers: vec![],
            socket: None,
        }
    }

    fn set_peer_id(&mut self, id: ServerId, addr: SocketAddr) {
        if let Some(ref mut peer) = self.peers.iter_mut().find(|peer| peer.addr == addr) {
            peer.id = Some(id);
        }
        // else {
        //     self.0.push(Peer {
        //         id: Some(id),
        //         addr: addr,
        //         positions: None,
        //         sent: None,
        //     });
        // }
    }
}

impl Peer {
    fn new(addr: &str) -> Peer {
        Peer {
            id: None,
            addr: SocketAddr::from_str(addr).unwrap(),
            positions: None,
            sent: None,
        }
    }
}

impl Deref for Peers {
    type Target = Vec<Peer>;
    fn deref(&self) -> &Vec<Peer> {
        &self.peers
    }
}

impl DerefMut for Peers {
    fn deref_mut(&mut self) -> &mut Vec<Peer> {
        &mut self.peers
    }
}


//// raftlog

struct RaftLog {
    cindex: usize,
    aindex: usize,
    entries: Vec<LogEntry>,
}

#[derive(Debug)]
#[derive(Eq)]
struct LogEntry {
    index: usize,
    term: usize,
    // FIXME:
    // key: String,
    // val: String,
}

impl Default for LogEntry {
    fn default() -> LogEntry {
        LogEntry {
            index: 0,
            term: 0,
        }
    }
}

impl RaftLog {
    fn new() -> RaftLog {
        RaftLog {
            cindex: 0,
            aindex: 0,
            entries: vec![],
        }
    }

    fn append(&mut self, entries: Vec<LogEntry>, cindex: usize) {
        self.check_conflict(&entries);
        self.append_new(entries);
        self.commit_to(cindex);
        self.apply();
    }

    fn check_conflict(&mut self, entries: &Vec<LogEntry>) {
        // 3. If an existing entry conflicts with a new one (same index
        //    but different terms), delete the existing entry and all that
        //    follow it.
        let mut conflict = None;
        for entry in entries {
            match self.entries.get(entry.index) {
                None => break,
                Some(&ref e @ LogEntry { .. }) => {
                    if e.term != entry.term {
                        info!("conflicting entries: {:?} != {:?}", e, entry);
                        conflict = Some(entry.index);
                        break;
                    }
                },
            }
        }
        if let Some(idx) = conflict {
            self.entries.truncate(idx); // XXX: off by 1?
        }
    }

    fn append_new(&mut self, entries: Vec<LogEntry>) {
        // 4. Append any new entries not already in the log.
        let idx = entries.last().map_or(0, |e| e.index);
        let mut news = entries.into_iter().skip_while(|e| e.index < idx).collect::<Vec<LogEntry>>();
        info!("append {} new entries to log", news.len());
        self.entries.append(&mut news);
    }

    fn commit_to(&mut self, cindex: usize) {
        // 5. If leaderCommit > commitIndex, set commitIndex =
        //    min(leaderCommit, index of last new entry)
        if self.cindex < cindex {
            let idx = self.entries.last().map_or(cindex, |e| e.index);
            self.cindex = cmp::min(cindex, idx);
            info!("advance commit index to {}", self.cindex);
        }
    }

    fn apply(&mut self) {
        // Rules for All Servers:
        // If commitIndex > lastApplied: increment lastApplied, apply
        // log[lastApplied] to state machine.
        while self.aindex < self.cindex {
            self.aindex = self.cindex;
            info!("advance apply index to {}", self.aindex);
        }
    }
}

impl PartialEq for LogEntry {
    fn eq(&self, other: &LogEntry) -> bool {
        self.term == other.term
        && self.index == other.index
    }
}

impl Ord for LogEntry {
    fn cmp(&self, other: &LogEntry) -> Ordering {
        //  If the logs have last entries with different terms, then the
        //  log with the later term is more up-to-date. If the logs end
        //  with the same term, then whichever log is longer is more
        //  up-to-date. (section 5.4.1)
        match self.term.cmp(&other.term) {
            Ordering::Equal => self.index.cmp(&other.index),
            ordering => ordering,
        }
    }
}

impl PartialOrd for LogEntry {
    fn partial_cmp(&self, other: &LogEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

//// server api

trait RequestHandler<T, U> {
    fn inspect_request(&self, request: &T) -> RpcResult;
    fn handle_request(&mut self, request: T) -> U;
}

trait ReplyHandler<T> {
    fn handle_reply_from(&mut self, from: ServerId, reply: T);
}

#[derive(Debug)]
enum RpcError {
    AlreadyVoted,
    LogEntryMissing,
    StaleLogEntry,
    StaleTerm,
    TermMismatch,
}

type RpcResult = Result<(), RpcError>;

impl RequestHandler<AppendEntriesRequest, AppendEntriesReply> for RaftServer {
    fn inspect_request(&self, request: &AppendEntriesRequest) -> RpcResult {
        // 1. Reply false if term < currentTerm.
        if request.term < self.election.term {
            return Err(RpcError::StaleTerm);
        }

        // 2. Reply false if log doesn’t contain an entry at prevLogIndex
        //    whose term matches prevLogTerm.
        match self.log.entries.get(request.prevLogIndex) {
            None => return Err(RpcError::LogEntryMissing),
            Some(&LogEntry { ref term, .. }) => {
                if *term != request.prevLogTerm {
                    return Err(RpcError::TermMismatch);
                }
            },
        }

        Ok(())
    }

    fn handle_request(&mut self, request: AppendEntriesRequest) -> AppendEntriesReply {
        if let Err(err) = self.inspect_request(&request) {
            info!(":: deny request ({:?})", err);
            return reply!(self, false)
        }

        self.election.timer.reset();
        self.maybe_change_role(request.term);
        self.set_leader(request.leaderId);
        self.log.append(request.entries, request.leaderCommit);
        return reply!(self, true);
    }
}

impl RequestHandler<RequestVotesRequest, RequestVotesReply> for RaftServer {
    fn inspect_request(&self, request: &RequestVotesRequest) -> RpcResult {
        // 1. Reply false if term < currentTerm.
        if request.term < self.election.term {
            return Err(RpcError::StaleTerm);
        }

        // 2. If votedFor is null or candidateId, and candidate’s log is at
        //    least as up-to-date as receiver’s log, grant vote.
        if self.election.voted_for.map_or(false, |id| id != request.candidateId) {
            return Err(RpcError::AlreadyVoted);
        }

        let lastentry =
            LogEntry { index: request.lastLogIndex, term: request.lastLogTerm };
        if self.log.entries.last().map_or(false, |entry| lastentry < *entry) {
            return Err(RpcError::StaleLogEntry);
        }

        Ok(())
    }

    fn handle_request(&mut self, request: RequestVotesRequest) -> RequestVotesReply {
        if let Err(err) = self.inspect_request(&request) {
            info!(":: deny request ({:?})", err);
            return reply!(self, false)
        }

        self.election.timer.reset();
        self.maybe_change_role(request.term);
        self.election.voted_for = Some(request.candidateId);
        return reply!(self, true);
    }
}

impl ReplyHandler<AppendEntriesReply> for RaftServer {
    fn handle_reply_from(&mut self, from: ServerId, reply: AppendEntriesReply) {
        self.maybe_change_role(reply.term);
        if self.identity.role != Role::Leader {
            return;
        }

        let ref mut _peer =
            self.peers.iter_mut()
                .find(|peer| peer.id.unwrap() == from)
                .unwrap();

        if reply.success {
            warn!("unimplemented");
            // (*peer).positions.unwrap().nindex = peer.sent.unwrap().nindex;
            // (*peer).positions.unwrap().mindex = peer.sent.unwrap().mindex;
        } else {
            warn!("unimplemented");
            // TODO: send append_entries again for reply.serverId
            // (*peer).positions.unwrap().nindex -= 1;
        }
    }
}

impl ReplyHandler<RequestVotesReply> for RaftServer {
    fn handle_reply_from(&mut self, from: ServerId, reply: RequestVotesReply) {
        self.maybe_change_role(reply.term);
        if self.identity.role != Role::Candidate {
            info!("not a candidate, ignore vote");
            return;
        }

        if reply.voteGranted {
            self.election.votes += 1;
            info!("yes vote from {} (count: {})", from, self.election.votes);
            if self.election.votes as usize > (self.peers.len() / 2) {
                info!("seen majority");
                self.change_to(Role::Leader);
            }
        } else {
            warn!("saw vote rejection")
        }
    }
}

//// server behavior

impl RaftServer {
    fn change_to(&mut self, role: Role) {
        info!("role: {:?} -> {:?}", self.identity.role, role);
        self.identity.role = role;
        match role {
            Role::Leader => {
                self.identity.leader = None;
                self.send_msg(heartbeat!(self), To::AllPeers);
            },

            Role::Candidate => {
                self.start_new_election();
            },

            Role::Follower => {},
        }
    }

    fn maybe_change_role(&mut self, term: usize) {
        if self.election.term < term {
            info!("term: {} -> {}", self.election.term, term);
            self.election.term = term;
            self.change_to(Role::Follower);
        }
    }

    fn set_leader(&mut self, leader: ServerId) {
        assert_eq!(self.identity.role, Role::Follower);
        info!("leader: {} -> {}", self.identity.leader.unwrap(), leader);
        self.identity.leader = Some(leader);
    }

    fn start_new_election(&mut self) {
        assert_eq!(self.identity.role, Role::Candidate);
        info!("starting election (term {})", self.election.term);
        self.election.term += 1;
        self.election.votes = 1;
        self.election.voted_for = Some(self.identity.id);
        self.election.timer.reset();
        self.send_msg(request_votes!(self), To::AllPeers);
    }
}

//// server functionality

impl RaftServer {
    fn listen(socket: UdpSocket, tx: Sender<(SocketAddr, RaftMsg)>) {
        loop {
            let mut buf = [0; 1500]; // mtu
            let (amt, src) = socket.recv_from(&mut buf).unwrap();
            let msg = RaftMsg::from_str(String::from_utf8(buf[..amt].to_vec()).unwrap().as_str()).unwrap();
            info!("recv_msg {:?} {:?} from {}", msg.xtype, msg.xpart, msg.sender);
            tx.send((src, msg)).unwrap();
         }
    }

    fn start(server: &mut RwLock<RaftServer>, port: u16, argpeers: Vec<String>) -> ! {
        {
            let server = server.read().unwrap();
            info!("server id {:?} :: recv port {} :: election timeout {}ms",
                  server.identity.id, port, server.election.timer.timeout);
        }

        let election = { server.write().unwrap().election.timer.take() };

        let mut timer = Timer::from_millis(1500);
        let heartbeat = timer.take();

        let (tx, cluster) = mpsc::channel();
        let socket = UdpSocket::bind(bindaddr!(port)).unwrap();
        {
            let socket = socket.try_clone().unwrap();
            thread::spawn(move|| RaftServer::listen(socket, tx));
        }

        {
            let ref mut peers = server.write().unwrap().peers;
            peers.socket = Some(socket);
            for addr in argpeers {
                info!("add peer addr {}", addr);
                peers.push(Peer::new(addr.as_str()));
            }
        }

        // mainloop
        loop {
            select! {
                _tick = election.recv() => {
                    let mut server = server.get_mut().unwrap();
                    if server.identity.role == Role::Candidate {
                        info!("election failed (votes {})", server.election.votes);
                    }
                    info!("───────────────────── election timeout ─────────────────────");
                    server.change_to(Role::Candidate);
                },

                _tick = heartbeat.recv() => {
                    let server = server.read().unwrap();
                    if server.identity.role == Role::Leader {
                        info!("send heartbeats");
                        server.send_msg(heartbeat!(server), To::AllPeers);
                    }
                },

                pair = cluster.recv() => {
                    let (src, msg) = pair.unwrap();
                    let mut server = server.get_mut().unwrap();
                    server.peers.set_peer_id(msg.sender, src);
                    match msg.data {
                        MsgData::AppendEntriesRequest(request) => {
                            let reply = server.handle_request(request);
                            server.send_msg(raftmsg!(server.identity.id, reply), To::PeerAddr(src));
                        },

                        MsgData::RequestVotesRequest(request) => {
                            let reply = server.handle_request(request);
                            server.send_msg(raftmsg!(server.identity.id, reply), To::PeerAddr(src));
                        },

                        MsgData::AppendEntriesReply(reply) => {
                            server.handle_reply_from(msg.sender, reply);
                        },

                        MsgData::RequestVotesReply(reply) => {
                            server.handle_reply_from(msg.sender, reply);
                        },
                    }
                }
            }
        }
    }
}

//// server communication

enum To {
    AllPeers,
    PeerId(ServerId),
    PeerAddr(SocketAddr),
}

impl RaftServer {
    fn send_msg(&self, msg: RaftMsg, to: To) {
        let buf = msg.to_string();
        match to {
            To::AllPeers => {
                for peer in self.peers.iter() {
                    info!("send_msg {:?} {:?} to {}", msg.xtype, msg.xpart, peer.addr);
                    self.peers.socket.as_ref().unwrap().send_to(buf.as_bytes(), peer.addr);
                }
            },

            To::PeerAddr(addr) => {
                info!("send_msg {:?} {:?} to {}", msg.xtype, msg.xpart, addr);
                self.peers.socket.as_ref().unwrap().send_to(buf.as_bytes(), addr);
            },

            To::PeerId(_) => {
                unimplemented!();
            }
        }
    }
}

//// raftmsg

struct RaftMsg {
    xtype: ExchangeType,
    xpart: ExchangePart,
    sender: ServerId,
    data: MsgData,
}

#[derive(Debug, PartialEq)]
enum ExchangeType {
    AppendEntries,
    RequestVotes,
    InstallSnapshot,
}

#[derive(Debug, PartialEq)]
enum ExchangePart {
    Request,
    Reply,
}

enum MsgData {
    AppendEntriesRequest(AppendEntriesRequest),
    RequestVotesRequest(RequestVotesRequest),
    AppendEntriesReply(AppendEntriesReply),
    RequestVotesReply(RequestVotesReply),
}

#[allow(non_snake_case)]
struct AppendEntriesRequest {
    term: usize,
    leaderId: ServerId,
    prevLogIndex: usize,
    prevLogTerm: usize,
    entries: Vec<LogEntry>,
    leaderCommit: usize,
}

#[allow(non_snake_case)]
struct RequestVotesRequest {
    term: usize,
    candidateId: ServerId,
    lastLogIndex: usize,
    lastLogTerm: usize,
}

#[allow(non_snake_case)]
struct AppendEntriesReply {
    term: usize,
    success: bool,
}

#[allow(non_snake_case)]
struct RequestVotesReply {
    term: usize,
    voteGranted: bool,
}

impl ToString for RaftMsg {
    fn to_string(&self) -> String {
        format!("{}:{:?}:{:?}:{}",
                self.sender, self.xtype, self.xpart,
                self.data.to_string())
    }
}

struct RaftParseError;

impl fmt::Debug for RaftParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "unable to parse".fmt(f)
    }
}

impl FromStr for RaftMsg {
    type Err = RaftParseError;
    fn from_str(s: &str) -> Result<RaftMsg, Self::Err> {
        macro_rules! parse { ($iter:ident) => { $iter.next().unwrap().parse().unwrap() } }

        let mut iter = s.split(":");
        debug!("{:?}", iter.clone().collect::<Vec<_>>());

        let sender = iter.next().unwrap().parse::<ServerId>().unwrap();
        let xtype = iter.next().unwrap().parse::<ExchangeType>().unwrap();
        let xpart = iter.next().unwrap().parse::<ExchangePart>().unwrap();

        match xtype {
            ExchangeType::AppendEntries => {
                match xpart {
                    ExchangePart::Request => {
                        let data =
                            AppendEntriesRequest {
                                term: parse!(iter),
                                leaderId: parse!(iter),
                                prevLogIndex: parse!(iter),
                                prevLogTerm: parse!(iter),
                                entries: vec![],
                                leaderCommit: parse!(iter),
                            };
                        Ok(raftmsg!(sender, data))
                    },

                    ExchangePart::Reply => {
                        let data =
                            AppendEntriesReply {
                                term: parse!(iter),
                                success: parse!(iter),
                            };
                        Ok(raftmsg!(sender, data))
                    },
                }
            },

            ExchangeType::RequestVotes => {
                match xpart {
                    ExchangePart::Request => {
                        let data =
                            RequestVotesRequest {
                                term: parse!(iter),
                                candidateId: parse!(iter),
                                lastLogIndex: parse!(iter),
                                lastLogTerm: parse!(iter),
                            };
                        Ok(raftmsg!(sender, data))
                    },

                    ExchangePart::Reply => {
                        let data =
                            RequestVotesReply {
                                term: parse!(iter),
                                voteGranted: parse!(iter),
                            };
                        Ok(raftmsg!(sender, data))
                    },
                }
            }

            ExchangeType::InstallSnapshot => {
                unimplemented!()
            }
        }
    }
}

impl FromStr for ExchangeType {
    type Err = RaftParseError;
    fn from_str(s: &str) -> Result<ExchangeType, Self::Err> {
        match s {
            "AppendEntries" => Ok(ExchangeType::AppendEntries),
            "RequestVotes" => Ok(ExchangeType::RequestVotes),
            _ => Err(RaftParseError),
        }
    }
}

impl FromStr for ExchangePart {
    type Err = RaftParseError;
    fn from_str(s: &str) -> Result<ExchangePart, Self::Err> {
        match s {
            "Request" => Ok(ExchangePart::Request),
            "Reply" => Ok(ExchangePart::Reply),
            _ => Err(RaftParseError),
        }
    }
}

impl ToString for MsgData {
    fn to_string(&self) -> String {
        match *self {
            MsgData::AppendEntriesRequest(ref request) =>
                format!("{}:{}:{}:{}:{}",
                    request.term, request.leaderId, request.prevLogIndex, request.prevLogTerm, request.leaderCommit),

            MsgData::RequestVotesRequest(ref request) =>
                format!("{}:{}:{}:{}",
                        request.term, request.candidateId, request.lastLogIndex, request.lastLogTerm),

            MsgData::AppendEntriesReply(ref reply) =>
                format!("{}:{}", reply.term, reply.success),

            MsgData::RequestVotesReply(ref reply) =>
                format!("{}:{}", reply.term, reply.voteGranted),
        }
    }
}

from!(ExchangeType, ExchangeType::AppendEntries, AppendEntriesRequest);
from!(ExchangeType, ExchangeType::AppendEntries, AppendEntriesReply);
from!(ExchangeType, ExchangeType::RequestVotes, RequestVotesRequest);
from!(ExchangeType, ExchangeType::RequestVotes, RequestVotesReply);

from!(ExchangePart, ExchangePart::Request, AppendEntriesRequest);
from!(ExchangePart, ExchangePart::Request, RequestVotesRequest);
from!(ExchangePart, ExchangePart::Reply, AppendEntriesReply);
from!(ExchangePart, ExchangePart::Reply, RequestVotesReply);

fromdata!(MsgData, MsgData::AppendEntriesRequest, AppendEntriesRequest);
fromdata!(MsgData, MsgData::AppendEntriesReply, AppendEntriesReply);
fromdata!(MsgData, MsgData::RequestVotesRequest, RequestVotesRequest);
fromdata!(MsgData, MsgData::RequestVotesReply, RequestVotesReply);

impl From<(usize, bool)> for AppendEntriesReply {
    fn from(pair: (usize, bool)) -> AppendEntriesReply {
        AppendEntriesReply {
            term: pair.0,
            success: pair.1
        }
    }
}

impl From<(usize, bool)> for RequestVotesReply {
    fn from(pair: (usize, bool)) -> RequestVotesReply {
        RequestVotesReply {
            term: pair.0,
            voteGranted: pair.1
        }
    }
}
