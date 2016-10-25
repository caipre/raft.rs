extern crate log;
extern crate rand;

use std::mem;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

use rand::Rng;

pub struct Timer {
    pub timeout: u64,
    pub rx: Option<Receiver<()>>,
    pair: Arc<(Mutex<()>, Condvar)>,
}

impl Timer {
    pub fn from_millis(timeout: u64) -> Timer {
        let (tx, rx) = mpsc::channel();
        let pair = Arc::new((Mutex::new(()), Condvar::new()));

        {
            let pair = pair.clone();
            thread::spawn(move|| {
                let (ref lock, ref condvar) = *pair;
                Timer::start(Duration::from_millis(timeout), lock, condvar, tx);
            });
        }

        Timer {
            timeout: timeout,
            rx: Some(rx),
            pair: pair
        }
    }

    pub fn take(&mut self) -> Receiver<()> {
        self.rx.take().unwrap()
    }

    pub fn reset(&self) {
        let (_, ref condvar) = *(self.pair);
        condvar.notify_one();
    }

    fn start(duration: Duration,
             lock: &Mutex<()>,
             condvar: &Condvar,
             tx: Sender<()>) -> !
    {
        let mut guard = lock.lock().unwrap();
        loop {
            let (guard_, wait_result) = condvar.wait_timeout(guard, duration).unwrap();
            guard = guard_;
            if wait_result.timed_out() {
                tx.send(());
            }
        }
    }
}

