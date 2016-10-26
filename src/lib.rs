use std::thread;

const RAFT_CLUSTER_SIZE: usize = 5;

macro_rules! majority {
    ( $(n:ident) ) => { n >= (RAFT_CLUSTER_SIZE/2)+1 };
}

enum Rpc {
    RequestVote,
    AppendEntry,
}

struct RequestVote {
    term: usize,
    candidate: server::ServerId,
    lterm: usize,
    lindex: usize,
}

struct AppendEntries {
    term: usize,
    leader: server::ServerId,
    cindex: usize,
    pindex: usize,
    pterm: usize,
    entries: Option<server::LogEntry>,
}

trait Candidate {
    fn init(&mut self) {
        self.role = Role::Candidate;
        self.term += 1;
        self.election = time::SystemTime::now();
    }

    fn request_votes(&mut self) {
        let (tx, rx) = channel();
        for peer in self.peers {
            peer.request(Rpc::RequestVote, tx.clone())
        }

        let mut yes_votes = 0;
        rx.iter().take_while(|result| {
            if let Err(term) = result {
                self.term = term;
                false
            } else {
                yes_votes += 1;
                if majority!(yes_votes) {
                    self.role = Role::Leader;
                    false
                } else {
                    true
                }
            }
        });
    }
}

trait Follower {
    // 1. Reply false if term < currentTerm (§5.1)
    // 2. Reply false if log doesn’t contain an entry at prevLogIndex whose
    //    term matches prevLogTerm (§5.3)
    // 3. If an existing entry conflicts with a new one (same index but
    //    different terms), delete the existing entry and all that follow
    //    it (§5.3)
    // 4. Append any new entries not already in the log
    // 5. If leaderCommit > commitIndex, set commitIndex =
    //    min(leaderCommit, index of last new entry)
    fn append_entries(&mut self, term: usize, leader: ServerId, cindex: usize,
                      pindex: usize, pterm: usize, entries: Option<LogEntry>) -> Result<(), RaftError>;
}

impl Follower for Server {

}
trait Leader {
    fn publish_entries(&self);
}

impl Leader for Server {

}
struct RaftLog {
    entries: Vec<LogEntry>,
}
struct LogEntry {
    term: usize,
    item: Item,
}
struct Item(usize);

impl RaftLog {
    fn cmp(&self, lterm: usize, lindex: usize) -> Ordering {
        if let Some(term) = self.entries.last().map(|last| last.term) {
            if term == lterm {
                self.cindex.cmp(lindex)
            } else {
                term.cmp(lterm)
            }
        } else {
            Ordering::Less
        }
    }
}

enum RaftError {
    TermError(usize),
}


use super::RAFT_CLUSTER_SIZE;

#[derive(Eq)]
struct ServerId(usize);

enum Role {
    Leader,
    Candidate,
    Follower,
}

struct Server {
    id: ServerId,
    role: Role,

    // persistent state
    //
    term: usize,                 // current term
    voted_for: Option<ServerId>, // vote in most recent election
    log: RaftLog,                // applied state

    // volatile state
    // (lost at termination)
    //
    peers: &[ServerId; RAFT_CLUSTER_SIZE],
    election: time::Instant, // last election
    cindex: usize,           // commit index
    aindex: usize,           // apply index

    // volatile, leader state
    // (reinitialized after election)
    //
    nindex: [usize; RAFT_CLUSTER_SIZE], // index of next log entry to send to each server
    mindex: [usize; RAFT_CLUSTER_SIZE], // index of highest log entry known on each server
}

impl Server {
    fn runloop(&mut self) {
        loop {
            self.
            if

        }
    }
}

trait Server {
    fn vote(&self, term: usize, candidate: ServerId,
            lindex: usize, lterm: usize) -> Result<(), RaftError> {
        if term > self.term
            || (voted_for.map_or(true, |id| id == candidate)
                && self.log.cmp(lterm, lindex) != Ordering::Greater

                (self.log.last().map_or(false, |llt| llt < lterm)
                   || lterm < self.cindex))
        {
            Ok(())
        }
        Err(TermError(self.term));
    }
}
