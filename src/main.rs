use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use parser::Parser;

use crate::{http::HttpClientBlocking, pool::ThreadPool};

mod http;
mod parser;
mod pool;

//TODO: need to handle errors just like clap
fn main() {
    let parser = Parser::new();
    let mut thread_pool = ThreadPool::new();
    let arguments = parser.get_arguments().unwrap();
    let http_client = HttpClientBlocking::from_arguments(&arguments).unwrap();
    let responses = Arc::new(Mutex::new(Vec::new()));
    let timeouts = Arc::new(Mutex::new(Vec::new()));

    for _ in 0..arguments.connections {
        let http_client = http_client.clone();
        let responses = responses.clone();
        let timeouts = timeouts.clone();

        thread_pool.add(Box::new(move || {
            let start_time = Instant::now();

            loop {
                let end_time = Instant::now();
                let elapsed_time = end_time - start_time;

                if elapsed_time < Duration::from_secs(arguments.duration) {
                    if let Ok(response) = http_client.call() {
                        //TODO: Make sure proper error handling implemented here
                        responses.lock().unwrap().push(response);
                    } else {
                        timeouts.lock().unwrap().push(0);
                    }
                } else {
                    break;
                }
            }
        }));
    }

    ThreadPool::wait_execution(thread_pool);

    println!("{:?}", responses.lock().unwrap().len());
    println!("{:?}", timeouts.lock().unwrap().len())
}
