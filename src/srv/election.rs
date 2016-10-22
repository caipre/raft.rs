extern crate rand;

use std::sync::mpsc;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

use rand::Rng;

struct Election {
    term: usize,
    voted_for: Option<PeerId>,
    timeout: Duration,
    votes: u8,
}
impl Election {
    fn new() -> Election {
        let duration = {
            let mut rng = rand::thread_rng();
            rng.gen_range(100, 500)
        };

        Election {
            term: 0,
            voted_for: None,
            timeout: duration,
            votes: 0,
        }
    }

    fn start(&mut self, id: ServerId) {
        self.term += 1;
        self.voted_for = id,
    }

    fn count(&mut self, vote: bool) {
        assert_eq!(self.role, Role::Candidate);
        vote && self.votes += 1;
    }

    fn received_majority(&self) -> bool {
        assert_eq!(self.role, Role::Candidate);
        votes > (CLUSTER_SIZE / 2)
    }
}

pub struct Timer {
    pub timeout: time::Duration,
    pair: Arc<(Mutex<()>, Condvar)>,
}
impl Timer {
    pub fn new() -> (Timer, mpsc::Receiver<Event>) {
        let mut rng = rand::thread_rng();
        let duration = time::Duration::from_millis(4000);

        let (tx, rx) = mpsc::channel();
        let pair = Arc::new((Mutex::new(()), Condvar::new()));
        {
            let pair = pair.clone();
            thread::spawn(move|| {
                let (ref lock, ref condvar) = *pair;
                Timer::start(duration, lock, condvar, tx);
            });
        }

        let timer = Timer { timeout: duration, pair: pair };
        (timer, rx)
    }

    pub fn reset(&self) {
        let (_, ref condvar) = *(self.pair);
        condvar.notify_one();
    }

    fn start(duration: time::Duration,
             lock: &Mutex<()>,
             condvar: &Condvar,
             tx: mpsc::Sender<Event>) -> !
    {
        let mut guard = lock.lock().unwrap();
        loop {
            let (guard_, wait_result) = condvar.wait_timeout(guard, duration).unwrap();
            guard = guard_;
            if wait_result.timed_out() {
                tx.send(Event::Timeout);
            }
        }
    }
}
