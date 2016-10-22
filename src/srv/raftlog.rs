use std::cmp::Ordering;

struct RaftLog {
    cindex: usize,
    aindex: usize,
    entries: Vec<Entry>,
}

impl RaftLog {
    pub fn append_entries(&mut self, entries: &Vec<Entry>, cindex: usize) {
        self.append(entries);
        self.commit_to(cindex);
        self.apply();
    }

    fn append(&mut self, entries: &Vec<Entry>) {
        // AppendEntries step 3
        for entry in entries {
            if self.entries.get(entry.index).map_or(false, |e| e.term != entry.term) {
                self.entries.split_off(entry.index);
                break;
            }
        }

        // AppendEntries step 4
        let news = entries.iter().skip_while(|e| e.index < self.index.len() - 1).collect();
        self.entries.append(news);
    }

    fn commit(&mut self, cindex: usize) {
        // AppendEntries step 5
        if self.cindex < cindex {
            self.cindex = cmp::min(lcidx, self.entries.last().map(lcidx, |e| e.index));
        }
    }

    fn apply(&mut self) {
        while self.aindex < self.cindex {
            debug!("apply index: {}, commit index: {}", self.aindex, self.cindex);
            self.aindex = self.cindex;
        }
    }
}
}

#[derive(Eq)]
struct Entry {
    index: usize,
    term: usize,
    // key: String
    // val: Vec<u8>
}


impl PartialEq for Entry {
    fn eq(&self, other: &Entry) -> bool {
        self.term == other.term
        && self.index == other.index
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        match self.term.cmp(other.term) {
            Ordering::Equal => self.index.cmp(other.index),
            ordering => ordering,
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
