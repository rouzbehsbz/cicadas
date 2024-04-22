use crate::{http::HttpClientBlocking, pool::ThreadPool};
use app::App;
use errors::AppResult;
use http::StatusCodeCategory;
use logger::Logger;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use storage::Storage;

mod app;
mod errors;
mod http;
mod logger;
mod output;
mod pool;
mod storage;

fn main() {
    let mut app = App::new();

    match app_handler(&app) {
        Ok(_) => {}
        Err(error) => app.throw_error(error),
    }
}

fn app_handler(app: &App) -> AppResult<()> {
    let mut thread_pool = ThreadPool::new();
    let arguments = Arc::new(app.get_arguments()?);
    let storage = Arc::new(Storage::new());

    for _ in 0..arguments.connections {
        let storage = storage.clone();
        let arguments = arguments.clone();

        thread_pool.add(Box::new(move || {
            let http_client = HttpClientBlocking::from_arguments(&arguments)?;
            let start_time = Instant::now();

            loop {
                if Instant::now() - start_time < Duration::from_secs(arguments.duration) {
                    let response_start_time = Instant::now();
                    let response = http_client.call();
                    let elapsed_response_time = Instant::now() - response_start_time;

                    if let Ok(response) = response {
                        storage
                            .add_response_time(response.status().into(), elapsed_response_time)?;
                    } else {
                        storage
                            .add_response_time(StatusCodeCategory::Failed, elapsed_response_time)?;
                    }
                } else {
                    break;
                }
            }

            Ok(())
        }));
    }

    ThreadPool::wait_execution(thread_pool)?;

    Logger::show_overview(&arguments, storage.clone())?;
    Logger::show_results(storage)?;

    Ok(())
}
