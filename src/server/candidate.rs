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

