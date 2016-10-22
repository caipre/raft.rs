macro_rules! identify {
    ($id:ident) => (
        Identify {
            serverId: id,
        }
    )
}
macro_rules! heartbeat {
    () => append_entries!(vec!()),
}

// TODO: entry depends on peer
macro_rules! append_entries {
    ($entryfn:expr, $entries:expr) => {
        let entry = $entryfn.
        AppendEntries {
            term: self.term,
            leaderId: self.identity.id,
            prevLogIndex: .index,
            prevLogTerm: entry.term,
            entries: $entries,
            leaderCommit: self.log.cindex,
        }
    }
}

macro_rules! request_votes {
    ($entry:ident) => {
        RequestVote {
            term: self.term,
            candidateId: self.identity.id,
            lastLogIndex: $entry.index,
            lastLogTerm: $entry.term,
        }
    }
}
