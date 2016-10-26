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


