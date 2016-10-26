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
