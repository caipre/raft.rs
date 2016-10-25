#![feature(mpsc_select)]

#[macro_use]
extern crate slog;
extern crate slog_term;

extern crate rand;
extern crate itertools;

#[macro_use]
mod macros;

mod timer;

use std::cmp::{self, Ordering};
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::num::ParseIntError;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use std::sync::RwLock;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use itertools::Itertools;
use rand::Rng;
use slog::DrainExt;
use slog::ser;

use timer::Timer;

pub fn start(id: ServerId, port: u16, peers: Vec<String>) {
    let term = slog_term::streamer().build();
    let drain = slog::LevelFilter::new(term, slog::Level::Trace).fuse();
    let logger = slog::Logger::root(drain, o!());

    let mut server = RaftServer::new(&logger, id, peers);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move|| RaftServer::listen(&logger, port, tx));
    RaftServer::mainloop(&mut server, rx);
}

#[derive(Debug)]
enum Topic {
    Identity,
    Election,
    Peers,
    Network,
    RaftMsg,
    RaftLog,
}

//// server

struct RaftServer {
    identity: Identity,
    election: Election,
    heartbeat: Timer,
    peers: Peers,
    log: RaftLog,
    logger: slog::Logger,
}

impl RaftServer {
    fn new(logger: &slog::Logger, id: ServerId, peers: Vec<String>)
        -> RwLock<RaftServer> {
        let logger = logger.new(o!());
        let cluster_size = peers.len() + 1;
        info!(logger, "raft server";
              "id" => id,
              "cluster size" => cluster_size);
        RwLock::new(
            RaftServer {
                identity: Identity::new(&logger, id),
                election: Election::new(&logger, (cluster_size / 2) as u8),
                heartbeat: Timer::from_millis(1500),
                peers: Peers::new(&logger, peers),
                log: RaftLog::new(&logger),
                logger: logger,
            }
        )
    }

    fn is_leader(&self) -> bool {
        self.identity.role == Role::Leader
    }

    fn is_candidate(&self) -> bool {
        self.identity.role == Role::Candidate
    }

    fn is_follower(&self) -> bool {
        self.identity.role == Role::Follower
    }
}

//// identity

#[derive(Debug)]
struct Identity {
    id: ServerId,
    role: Role,
    leader: Option<ServerId>,
    logger: slog::Logger,
}

type ServerId = usize;

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
enum Role {
    Leader,
    Candidate,
    Follower,
}

impl Identity {
    fn new(logger: &slog::Logger, id: ServerId) -> Identity {
        Identity {
            id: id,
            role: Role::Follower,
            leader: None,
            //logger: logger.new(o!("topic" => dbg!(Topic::Identity))),
            logger: logger.new(o!()),
        }
    }

    fn set_role(&mut self, role: Role) {
        if self.role != role {
            info!(self.logger, "change role";
                  "from" => dbg!(self.role), "to" => dbg!(role));
            self.role = role;
        }

        if self.role == Role::Leader {
            self.set_leader(None);
        }
    }

    fn set_leader(&mut self, leader: Option<ServerId>) {
        if self.leader.is_some() && leader.is_some()
            && self.leader.unwrap() != leader.unwrap() {
            info!(self.logger, "change leader";
                    "from" => self.leader, "to" => leader);
        } else if leader.is_none() {
            info!(self.logger, "abandon leader"; "was" => self.leader);
        }
        self.leader = leader;
    }
}

//// election

struct Election {
    term: usize,
    timer: Timer,
    voted_for: Option<ServerId>,
    votes: u8,
    votes_required: u8,
    logger: slog::Logger,
}

impl Election {
    fn new(logger: &slog::Logger, votes_required: u8) -> Election {
        let timeout = {
            let mut rng = rand::thread_rng();
            rng.gen_range(2000, 5000)
        };

        //let logger = logger.new(o!("topic" => dbg!(Topic::Election)));
        let logger = logger.new(o!());

        info!(logger, "election timeout {}ms", timeout);

        Election {
            term: 0,
            timer: Timer::from_millis(timeout),
            voted_for: None,
            votes: 0,
            votes_required: votes_required,
            logger: logger,
        }
    }

    fn set_term(&mut self, term: usize) {
        info!(self.logger, "change term";
            "from" => self.term, "to" => term);
        self.term = term;
    }

    fn vote_for(&mut self, id: ServerId) {
        info!(self.logger, "vote for {}", id);
        self.voted_for = Some(id);
    }

    fn start_election(&mut self, id: ServerId) {
        warn!(self.logger, "start an election"; "term" => self.term);
        self.term += 1;
        self.votes = 1;
        self.voted_for = Some(id);
        self.timer.reset();
    }

    fn add_vote(&mut self) {
        self.votes += 1;
        info!(self.logger, "receive yes vote"; "votes" => self.votes);
    }

    fn majority(&self) -> bool {
        let majority = self.votes > self.votes_required;
        if majority {
            info!(self.logger, "election success");
        }
        majority
    }

    fn fail(&self) {
        warn!(self.logger, "election failure"; "votes" => self.votes);
    }
}

//// peers

struct Peers {
    peers: Vec<Peer>,
    logger: slog::Logger,
}

struct Peer {
    id: ServerId,
    socket: UdpSocket,
    addr: SocketAddr,
    positions: Option<Positions>,
    sent: Option<Positions>,
}

struct Positions {
    nindex: usize,
    mindex: usize,
}

#[derive(Debug)]
enum To {
    AllPeers,
    PeerId(ServerId),
}

impl Peers {
    fn new(logger: &slog::Logger, argpeers: Vec<String>) -> Peers {
        //let logger = logger.new(o!("topic" => dbg!(Topic::Peers)));
        let logger = logger.new(o!());

        let mut peers = vec![];
        for peer in argpeers {
            let mut pieces = peer.splitn(2, ":");
            debug!(logger, "peer: {:?}",
                   pieces.clone().collect::<Vec<_>>());

            let id =
                pieces.next()
                    .map(|id| ServerId::from_str(id).ok())
                    .expect("unable to parse peer id")
                    .unwrap();

            let addr =
                pieces.next()
                    .map(|addr| SocketAddr::from_str(addr).ok())
                    .expect("unable to parse peer addr")
                    .unwrap();

            peers.push(Peer::new(id, addr));
        }

        Peers {
            peers: peers,
            logger: logger,
        }
    }

    fn send_msg(&self, msg: RaftMsg, to: To) {
        debug!(self.logger, "send message";
              "xtype" => dbg!(msg.xtype),
              "xpart" => dbg!(msg.xpart),
              "to" => dbg!(to));

        let buf = msg.to_string();
        let sendmsg = |peer: &Peer| {
            peer.socket.send_to(buf.as_bytes(), peer.addr);
        };

        match to {
            To::AllPeers => {
                self.peers.iter()
                    .foreach(sendmsg);
            },

            To::PeerId(id) => {
                self.peers.iter()
                    .find(|peer| peer.id == id)
                    .and_then(|peer| Some(sendmsg(peer)));
            },
        }
    }
}

impl Peer {
    fn new(id: ServerId, addr: SocketAddr) -> Peer {
        let socket = {
            let mut rng = rand::thread_rng();
            rng.gen_iter::<u16>()
                .filter(|n| n > &1000u16).take(5)
                .map(|port| UdpSocket::bind(bindport!(port)))
                .find(|socket| socket.is_ok())
                .expect("unable to find port to bind to")
                .unwrap()
        };

        Peer {
            id: id,
            addr: addr,
            socket: socket,
            positions: None,
            sent: None,
        }
    }
}

//// raftlog

struct RaftLog {
    cindex: usize,
    aindex: usize,
    entries: Vec<LogEntry>,
    logger: slog::Logger,
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

impl RaftLog {
    fn new(logger: &slog::Logger) -> RaftLog {
        RaftLog {
            cindex: 0,
            aindex: 0,
            entries: vec![],
            //logger: logger.new(o!("topic" => dbg!(Topic::RaftLog))),
            logger: logger.new(o!()),
        }
    }

    fn append(&mut self, entries: Vec<LogEntry>, cindex: usize) {
        self.check_conflict(&entries);
        self.append_new(entries);
        self.commit_to(cindex);
        self.apply();
    }

    fn check_conflict(&mut self, entries: &Vec<LogEntry>) {
        trace!(self.logger, "check_conflict");
        // 3. If an existing entry conflicts with a new one (same index
        //    but different terms), delete the existing entry and all that
        //    follow it.
        let mut conflict = None;
        for entry in entries {
            match self.entries.get(entry.index) {
                None => break,
                Some(&ref e @ LogEntry { .. }) => {
                    if e.term != entry.term {
                        info!(self.logger, "log entry conflict";
                              "ours" => dbg!(e), "theirs" => dbg!(entry));
                        conflict = Some(entry.index);
                        break;
                    }
                },
            }
        }
        if let Some(idx) = conflict {
            info!(self.logger, "truncate"; "index" => idx);
            self.entries.truncate(idx); // XXX: off by 1?
        } else {
            // was none
        }
    }

    fn append_new(&mut self, entries: Vec<LogEntry>) {
        trace!(self.logger, "append_new");
        // 4. Append any new entries not already in the log.
        let idx = entries.last().map_or(0, |e| e.index);
        let mut news = entries.into_iter().skip_while(|e| e.index < idx).collect::<Vec<LogEntry>>();
        info!(self.logger, "append new entries"; "count" =>  news.len());
        self.entries.append(&mut news);
    }

    fn commit_to(&mut self, cindex: usize) {
        trace!(self.logger, "commit_to");
        // 5. If leaderCommit > commitIndex, set commitIndex =
        //    min(leaderCommit, index of last new entry)
        if self.cindex < cindex {
            let idx = self.entries.last().map_or(cindex, |e| e.index);
            let new = cmp::min(cindex, idx);
            info!(self.logger, "advance commit index";
                  "from" => self.cindex, "to" => new);
            self.cindex = new;
        }
    }

    fn apply(&mut self) {
        trace!(self.logger, "apply");
        // Rules for All Servers:
        // If commitIndex > lastApplied: increment lastApplied, apply
        // log[lastApplied] to state machine.
        if self.aindex < self.cindex {
            info!(self.logger, "advance apply index";
                  "from" => self.aindex, "to" => self.cindex);
            self.aindex = self.cindex;
        }
    }
}

impl Default for LogEntry {
    fn default() -> LogEntry {
        LogEntry {
            index: 0,
            term: 0,
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
        trace!(self.logger, "inspect_request");
        // 1. Reply false if term < currentTerm.
        if request.term < self.election.term {
            return Err(RpcError::StaleTerm);
        }

        // 2. Reply false if log doesn’t contain an entry at prevLogIndex
        //    whose term matches prevLogTerm.
        match self.log.entries.get(request.prevLogIndex) {
            // None => return Err(RpcError::LogEntryMissing),
            None => return Ok(()),
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
            info!(self.logger, "reject request"; "reason" => dbg!(err));
            return reply!(self, false)
        }

        self.election.timer.reset();
        self.check_term(request.term);
        self.identity.set_leader(Some(request.leaderId));
        self.log.append(request.entries, request.leaderCommit);
        return reply!(self, true);
    }
}

impl RequestHandler<RequestVotesRequest, RequestVotesReply> for RaftServer {
    fn inspect_request(&self, request: &RequestVotesRequest) -> RpcResult {
        trace!(self.logger, "inspect_request");
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
            info!(self.logger, "reject request"; "reason" => dbg!(err));
            return reply!(self, false)
        }

        self.election.timer.reset();
        self.check_term(request.term);
        self.election.vote_for(request.candidateId);
        return reply!(self, true);
    }
}

impl ReplyHandler<AppendEntriesReply> for RaftServer {
    fn handle_reply_from(&mut self, from: ServerId, reply: AppendEntriesReply) {
        self.check_term(reply.term);
        if !self.is_leader() {
            info!(self.logger, "ignore message");
            return;
        }

        // let ref mut _peer =
        //     self.peers.iter_mut()
        //         .find(|peer| peer.id.unwrap() == from)
        //         .unwrap();

        if reply.success {
            info!(self.logger, "update peer positions");
            // (*peer).positions.unwrap().nindex = peer.sent.unwrap().nindex;
            // (*peer).positions.unwrap().mindex = peer.sent.unwrap().mindex;
        } else {
            info!(self.logger, "retry append entries";
                  "new nindex" => -1);
            // TODO: send append_entries again for reply.serverId
            // (*peer).positions.unwrap().nindex -= 1;
        }
    }
}

impl ReplyHandler<RequestVotesReply> for RaftServer {
    fn handle_reply_from(&mut self, from: ServerId, reply: RequestVotesReply) {
        self.check_term(reply.term);
        if !self.is_candidate() {
            info!(self.logger, "ignore message");
            return;
        }

        if reply.voteGranted {
            self.election.add_vote();

            if self.election.majority() {
                self.change_to(Role::Leader);
            }
        } else {
            warn!(self.logger, "saw vote rejection")
        }
    }
}

//// server behavior

impl RaftServer {
    fn check_term(&mut self, term: usize) {
        trace!(self.logger, "check_term");
        if self.election.term < term {
            self.election.set_term(term);
            self.change_to(Role::Follower);
        }
    }

    fn change_to(&mut self, role: Role) {
        self.identity.set_role(role);
        match role {
            Role::Leader =>
                self.peers.send_msg(heartbeat!(self), To::AllPeers),

            Role::Candidate =>
                self.campaign(),

            Role::Follower =>
                pass!(),
        }
    }

    fn campaign(&mut self) {
        assert!(self.is_candidate());
        self.identity.set_leader(None);
        self.election.start_election(self.identity.id);
        self.peers.send_msg(request_votes!(self), To::AllPeers);
    }
}

//// server functionality

impl RaftServer {
    fn listen(logger: &slog::Logger, port: u16, tx: Sender<RaftMsg>) {
        let socket = UdpSocket::bind(bindport!(port)).unwrap();
        //let logger = logger.new(o!("topic" => dbg!(Topic::Network)));
        let logger = logger.new(o!());
        info!(logger, "listening"; "port" => port);
        loop {
            let mut buf = [0; 1500]; // mtu
            let (amt, src) = socket.recv_from(&mut buf).unwrap();
            debug!(logger, "message received";
                   "raw contents" => String::from_utf8_lossy(&buf[..amt]).into_owned());
            if let Ok(msgstr) = String::from_utf8(buf[..amt].to_vec()) {
                match RaftMsg::from_str(msgstr.as_str()) {
                    Ok(raftmsg) => {
                            info!(logger, "message received";
                                "xtype" => dbg!(raftmsg.xtype),
                                "xpart" => dbg!(raftmsg.xpart),
                                "from" => raftmsg.sender);
                            tx.send(raftmsg).unwrap();
                        },

                    Err(RaftParseError(err)) => {
                        warn!(logger, "message invalid"; "reason" => err);
                    },
                }
            }
         }
    }

    fn mainloop(server: &mut RwLock<RaftServer>, message: Receiver<RaftMsg>) -> ! {
        let election = { server.write().unwrap().election.timer.take_rx() };
        let heartbeat = { server.write().unwrap().heartbeat.take_rx() };

        loop {
            select! {
                _tick = election.recv() => {
                    let mut server = server.get_mut().unwrap();
                    if server.is_candidate() {
                        server.election.fail();
                    }
                    if !server.is_leader() {
                        server.change_to(Role::Candidate);
                    }
                },

                _tick = heartbeat.recv() => {
                    let server = server.read().unwrap();
                    if server.is_leader() {
                        server.peers.send_msg(heartbeat!(server), To::AllPeers);
                    }
                },

                msg = message.recv() => {
                    let msg = msg.unwrap();
                    let mut server = server.get_mut().unwrap();
                    info!(server.identity.logger, "identity";
                          "role" => dbg!(server.identity.role),
                          "leader" => server.identity.leader);
                    match msg.data {
                        MsgData::AppendEntriesRequest(data) => {
                            let reply = server.handle_request(data);
                            server.peers.send_msg(
                                raftmsg!(server.identity.id, reply),
                                To::PeerId(msg.sender));
                        },

                        MsgData::RequestVotesRequest(data) => {
                            let reply = server.handle_request(data);
                            server.peers.send_msg(
                                raftmsg!(server.identity.id, reply),
                                To::PeerId(msg.sender));
                        },

                        MsgData::AppendEntriesReply(data) => {
                            server.handle_reply_from(msg.sender, data);
                        },

                        MsgData::RequestVotesReply(data) => {
                            server.handle_reply_from(msg.sender, data);
                        },
                    }
                }
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

#[derive(Debug)]
struct RaftParseError(String);

impl Error for RaftParseError {
    fn description(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for RaftParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.as_str().fmt(f)
    }
}

impl From<ParseIntError> for RaftParseError {
    fn from(err: ParseIntError) -> RaftParseError {
        RaftParseError(err.description().to_string())
    }
}

impl FromStr for RaftMsg {
    type Err = RaftParseError;
    fn from_str(s: &str) -> Result<RaftMsg, Self::Err> {
        macro_rules! parse { ($iter:ident) => { $iter.next().unwrap().parse().unwrap() } }

        let mut iter = s.split(":");

        let sender = try!(iter.next().unwrap().parse::<ServerId>());
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
            _ => Err(RaftParseError(String::from("unknown exchange type"))),
        }
    }
}

impl FromStr for ExchangePart {
    type Err = RaftParseError;
    fn from_str(s: &str) -> Result<ExchangePart, Self::Err> {
        match s {
            "Request" => Ok(ExchangePart::Request),
            "Reply" => Ok(ExchangePart::Reply),
            _ => Err(RaftParseError(String::from("unknown exchange part"))),
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
