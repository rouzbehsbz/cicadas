use std::thread::{self, JoinHandle};

use crate::errors::{AppResult, ErrorType};

pub type Job = Box<dyn FnOnce() -> AppResult<()> + Send + 'static>;

pub struct Worker {
    thread: JoinHandle<AppResult<()>>,
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

    pub fn wait_execution(pool: Self) -> AppResult<()> {
        for worker in pool.workers {
            match worker.thread.join() {
                Ok(_) => {}
                Err(_) => return Err(ErrorType::InvalidError),
            }
        }

        Ok(())
    }
}
