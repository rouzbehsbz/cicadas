use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

type SafeReceiver = Arc<Mutex<Receiver<Job>>>;
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(receiver: SafeReceiver) -> Self {
        let thread = thread::spawn(move || {
            //TODO: Make sure lock is not poisoned here
            loop {
                let job = receiver.lock().unwrap().try_recv();

                if let Ok(job) = job {
                    job();
                } else {
                    break;
                }
            }
        });

        Self { thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}
