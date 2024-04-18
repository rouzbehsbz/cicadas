use crate::{http::HttpClientBlocking, pool::ThreadPool};
use http::StatusCodeCategory;
use logger::Logger;
use parser::Parser;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use storage::Storage;

mod http;
mod logger;
mod parser;
mod pool;
mod storage;

//TODO: need to handle errors just like clap
fn main() {
    let parser = Parser::new();
    let mut thread_pool = ThreadPool::new();
    let arguments = parser.get_arguments().unwrap();
    let http_client = HttpClientBlocking::from_arguments(&arguments).unwrap();
    let storage = Arc::new(Storage::new());

    for _ in 0..arguments.connections {
        let http_client = http_client.clone();
        let storage = storage.clone();

        thread_pool.add(Box::new(move || {
            let start_time = Instant::now();

            loop {
                let elapsed_time = Instant::now() - start_time;

                if elapsed_time < Duration::from_secs(arguments.duration) {
                    let response_start_time = Instant::now();
                    let response = http_client.call();
                    let elapsed_response_time = Instant::now() - response_start_time;

                    if let Ok(response) = response {
                        storage.add_response_time(response.status().into(), elapsed_response_time)
                    } else {
                        storage.add_response_time(StatusCodeCategory::Failed, elapsed_response_time)
                    }
                } else {
                    break;
                }
            }
        }));
    }

    ThreadPool::wait_execution(thread_pool);

    Logger::show_overview(&arguments, storage.clone());
    Logger::show_results(storage);
}
