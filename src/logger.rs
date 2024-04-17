use std::sync::Arc;

use prettytable::{row, Table};

use crate::{http::StatusCodeCategory, storage::Storage};

pub struct Logger;

impl Logger {
    pub fn start() {
        println!("Benchmarking ...");
    }

    pub fn show_results(storage: Arc<Storage>, duration: u64) {
        println!(
            "Total {} requests have been sent for {} seconds with total average response time of {} ms\n",
            storage.get_total_requests_count(),
            duration,
            storage.get_total_avg_response_time()
        );

        println!("Results:");

        let mut table = Table::new();

        table.add_row(row!["Type", "Total", "Avg Time (ms)"]);
        table.add_row(row![
            "Success (2xx)",
            storage.get_requests_count(StatusCodeCategory::Success),
            storage.get_avg_response_time(StatusCodeCategory::Success)
        ]);
        table.add_row(row![
            "Redirection (3xx)",
            storage.get_requests_count(StatusCodeCategory::Redirection),
            storage.get_avg_response_time(StatusCodeCategory::Redirection)
        ]);
        table.add_row(row![
            "Client Error (4xx)",
            storage.get_requests_count(StatusCodeCategory::ClientError),
            storage.get_avg_response_time(StatusCodeCategory::ClientError)
        ]);
        table.add_row(row![
            "Server Error (5xx)",
            storage.get_requests_count(StatusCodeCategory::ServerError),
            storage.get_avg_response_time(StatusCodeCategory::ServerError)
        ]);
        table.add_row(row![
            "Informational (1xx)",
            storage.get_requests_count(StatusCodeCategory::Informational),
            storage.get_avg_response_time(StatusCodeCategory::Informational)
        ]);
        table.add_row(row![
            "Failed",
            storage.get_requests_count(StatusCodeCategory::Failed),
            storage.get_avg_response_time(StatusCodeCategory::Failed)
        ]);

        table.printstd();
    }
}
