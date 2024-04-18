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
    let arguments = Arc::new(parser.get_arguments().unwrap());
    let storage = Arc::new(Storage::new());

    for _ in 0..arguments.connections {
        let storage = storage.clone();
        let arguments = arguments.clone();

        thread_pool.add(Box::new(move || {
            let http_client = HttpClientBlocking::from_arguments(&arguments).unwrap();
            let start_time = Instant::now();

            loop {
                if Instant::now() - start_time < Duration::from_secs(arguments.duration) {
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
