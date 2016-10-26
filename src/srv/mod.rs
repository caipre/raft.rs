extern crate rand;

use std::fmt;
use std::sync::RwLock;

use rand::Rng;

use CLUSTER_SIZE;

////

struct RaftServer {
    identity: Identity,
    election: Election,
    agents: RwLock<Vec<Option<Agent>>>,
    log: RaftLog,
    mtx: Sender<RaftMessage>,
    mrx: Receiver<RaftMessage>,
}

//// identity

struct Identity {
    id: ServerId,
    role: Role,
}

#[derive(Debug)]
struct ServerId(u64);
type PeerId = ServerId;
impl ServerId {
    fn new() -> ServerId {
        ServerId(rand::random())
    }
}
impl<T> From<T> for ServerId where T: u64 {
    fn from(id: T) -> ServerId {
        ServerId(id)
    }
}
impl fmt::Display for ServerId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", (self.0 & 0xfffff))
    }
}

#[derive(Debug, PartialEq)]
enum Role {
    Leader,
    Candidate,
    Follower,
}

//// server functionality

impl RaftServer {
    fn new() -> RaftServer {
        let (mtx, mrx) = mpsc::channel();
        RaftServer {
            identity: Identity::new(),
            election: Election::new(),
            agents: RwLock::new(vec!()),
            log: RaftLog::new(),
            mtx: mtx,
            mrx: mrx,
        }
    }
}

impl StreamHandler for Server {
    fn take(&self, stream: TcpStream) {
        let mut agent =
            Agent::new(self.identity.id, stream, self.mtx.clone());

        let idx = peeridx!(agent.peer.id);

        {
            let mut agents = self.agents.write().unwrap();
            if let Some(ref mut agent) = agents.get_mut(idx) {
                *agent = agent;
            } else {
                if agents.len() < idx {
                    agents.resize(idx + 1, None);
                }
                agents[idx] = agent;
            }
        }
    }
}

macro_rules! peeridx {
    ($id:ident) => { $id % CLUSTER_SIZE }
}

//// server behavior

impl RaftServer {
    fn maybe_change_role(&mut self, term: usize) -> Role {
        if self.election.term < term {
            self.assume_role(Role::Follower);
            self.election.term = term;
        }
        self.identity.role
    }

    fn assume_role(&mut self, role: Role) {
        self.identity.role = role;
        match role {
            Role::Leader =>
                self.agent.send_to(Peers::All, heartbeat!()),
            Role::Candidate => {
                let entry = self.log.last().unwrap();
                self.election.start(self.identity.id);
                self.agent.send_to(Peers::All, request_votes!(entry));
            }
            Role::Follower => {},
        }
    }

    fn set_leader(&mut self, leader: PeerId) {
        assert_eq!(self.identity.role, Role::Follower);
        self.leader = leader;
    }
}


//// rpc answering

trait RpcError;
impl RpcError for AppendEntriesError;
impl RpcError for RequestVoteError;

enum AppendEntriesError {
    StaleTerm,
    TermMismatch,
}

enum RequestVoteError {
    StaleTerm,
    AlreadyVoted,
    StaleLog,
}


type RpcResult = Result<(), T> where T: RpcError;
impl RaftServer {
    fn try_append_entries(&self, term: usize, preventry: Entry)
        -> RpcResult
    {
        if term < self.term {
            // 1
            Err(AppendEntriesError::StaleTerm)
        }
        else if self.log.get(preventry.index).map_or(true, |e| e.term != preventry.term) {
            // 2
            Err(AppendEntriesError::TermMismatch)
        }
        else {
            Ok(())
        }
    }

    fn try_vote(&self, term: usize, candidate: PeerId, lastentry: Entry)
        -> RpcResult
    {
        // 1
        if term < self.term {
            Err(RequestVoteError::StaleTerm)
        }
        // 2
        else if self.voted_for.map_or(false, |id| id != candidate) {
            Err(RequestVoteError::AlreadyVoted)
        }
        // 2
        else if self.log.last().map_or(false, |e| lastentry < e) {
            Err(RequestVoteError::StaleLog)
        }
        else {
            // TODO: verify ok() is correct for empty server log
            Ok()
        }

    }
}

//// raft mainloop

impl RaftServer {
    fn raftstart(&self) -> ! {
        loop {
            let timeout =
                match self.identity.role {
                    Role::Leader => Duration::from_millis(HEARTBEAT_TIMEOUT),
                               _ => Duration::from_millis(self.election.timeout),
                };

            // TODO: With recv_timeout, our implicit timer is reset by
            //       all messages that come through the channel. This may
            //       not be a problem (at worst, elections happen later).
            //
            //       If it is possible for Agent to answer the RPC without
            //       sending a message through the channel, that satisfies
            //       this concern. Alternatively, revert to using Timer.
            match self.mrx.recv_timeout(timeout) {
                Ok((&peer, RaftMessage::AppendEntries(msg))) => {
                    let preventry = Entry { index: msg.prevLogIndex, term: msg.prevLogTerm };
                    if self.try_append_entries(msg.term, preventry).is_err() {
                        let idx = agentidx!(msg.leaderId);
                        let mut agent = self.agents.write().map(|agents| agents[idx]).unwrap();
                        agent.write(append_entries_reply!());
                        continue;
                    }
                    self.maybe_change_role(msg.term);
                    self.set_leader(msg.leaderId);
                    self.log.append_entries(msg.entries, msg.leaderCommit);
                },

                Ok((&peer, RaftMessage::RequestVote(msg))) => {
                    let lastentry = Entry { index: msg.lastLogIndex, term: msg.lastLogTerm };
                    if self.try_vote(msg.term, msg.candidateId, lastentry).is_err() {
                        let idx = agentidx!(msg.candidateId);
                        let mut agent = self.agents.write().map(|agents| agents[idx]).unwrap();
                        agent.write(request_votes_reply!());
                        continue;
                    }
                    self.maybe_change_role(msg.term);
                    self.election.voted_for = msg.candidateId;
                },

                Ok((&peer, RaftMessage::AppendEntriesReply(term, success))) => {
                    if self.maybe_change_role(term) != Role::Leader { continue; }
                    if success {
                        // FIXME: are these the correct indicies?
                        peer.position.nindex = self.log.cindex;
                        peer.position.mindex = self.log.cindex;
                    } else {
                        peer.position.nindex -= 1;
                    }
                },

                Ok((&peer, RaftMessage::RequestVoteReply(msg))) => {
                    if self.maybe_change_role(term) != Role::Candidate { continue; }
                    self.election.count(vote);
                    if self.election.received_majority() {
                        self.assume_role(Role::Leader);
                    }
                },

                Err(RecvTimeoutError::Timeout) => {
                    if self.identity.role == Role::Leader {
                        // heartbeat timeout
                        self.agent.send_to(Peers::All, heartbeat!());
                        continue;
                    }
                    else {
                        // election timeout
                        self.assume_role(Role::Candidate);
                    }
                },

                Err(RecvTimeoutError::Disconnected) => panic!("agent disconnected"),
            }
        }
    }
}

//// client mainloop

impl RaftServer {
    fn run(&self) -> {
        /* recv cmds on chan
         * direct to leader if not leader
         * apply to log otherwise
         */
    }
}
