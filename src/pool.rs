use std::thread::{self, JoinHandle};

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    thread: JoinHandle<()>,
}

impl Worker {
    pub fn new(job: Job) -> Self {
        let thread = thread::spawn(move || job());

        Self { thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new() -> Self {
        let workers = Vec::new();

        Self { workers }
    }

    pub fn add(&mut self, job: Job) {
        let worker = Worker::new(job);

        self.workers.push(worker);
    }

    pub fn wait_execution(pool: Self) {
        for worker in pool.workers {
            //TODO: Watch out errors here
            worker.thread.join().unwrap();
        }
    }
}
