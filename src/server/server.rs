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
