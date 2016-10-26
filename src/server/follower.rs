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
