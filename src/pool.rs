use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

type SafeReceiver<T> = Arc<Mutex<Receiver<Job<T>>>>;
type Job<T> = Box<dyn FnOnce() -> T + Send + 'static>;

pub struct Worker<T>
where
    T: Copy + Clone + Send + 'static,
{
    thread: JoinHandle<Option<T>>,
}

impl<T> Worker<T>
where
    T: Copy + Clone + Send + 'static,
{
    fn new(receiver: SafeReceiver<T>) -> Self {
        let thread = thread::spawn(move || {
            //TODO: Make sure lock is not poisoned here
            loop {
                let job = receiver.lock().unwrap().try_recv();

                if let Ok(job) = job {
                    return Some(job());
                } else {
                    break None;
                }
            }
        });

        Self { thread }
    }
}

pub struct ThreadPool<T>
where
    T: Copy + Clone + Send + 'static,
{
    workers: Vec<Worker<T>>,
    sender: Sender<Job<T>>,
}

impl<T> ThreadPool<T>
where
    T: Copy + Clone + Send + 'static,
{
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = channel::<Job<T>>();
        let safe_receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            let worker = Worker::new(safe_receiver.clone());

            workers.push(worker)
        }

        Self { workers, sender }
    }

    pub fn add(&self, job: Job<T>) {
        //TODO: make sure no error will occur here
        self.sender.send(job).unwrap();
    }

    pub fn aggregate_results(pool: Self) -> Vec<Option<T>> {
        let mut results = Vec::new();

        for worker in pool.workers {
            //TODO: Watch out errors here
            let response = worker.thread.join().unwrap();

            results.push(response);
        }

        results
    }
}
