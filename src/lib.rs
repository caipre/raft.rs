const RAFT_CLUSTER_SIZE: usize = 5;

enum Role {
    Leader,
    Candidate,
    Follower,
}

struct ServerId(usize);
struct LogEntry(usize);

struct Server {
    role: Role,
    id: ServerId,

    // persistent state
    //
    term: usize,            // current term
    voted_for: ServerId,    // vote in most recent election
    log: Vec<LogEntry>,     // applied state

    // volatile state
    // (lost at termination)
    //
    election: time::Instant, // last election
    cindex: usize,           // commit index
    aindex: usize,           // apply index

    // volatile, leader state
    // (reinitialized after election)
    //
    nindex: [usize; RAFT_CLUSTER_SIZE], // index of next log entry to send to each server
    mindex: [usize; RAFT_CLUSTER_SIZE], // index of highest log entry known on each server
}

enum RaftError {
    TermError,
}

trait Server {
    fn vote(&self, term: usize, candidate: ServerId,
            lindex: usize, lterm: usize) -> Result<_, RaftError>;
}

trait Leader {
    fn publish_entries(&self);
}

trait Candidate {
    fn init(&mut self);

    fn request_votes(&mut self);

    fn become_leader(&mut self);
}

trait Follower {
    fn append_entries(&mut self, term: usize, leader: ServerId, cindex: usize,
                      pindex: usize, pterm: usize, entries: Option<LogEntry>) -> Result<_, RaftError>;
}
