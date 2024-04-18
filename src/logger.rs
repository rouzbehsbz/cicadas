use std::sync::Arc;

use prettytable::{row, Table};

use crate::{errors::AppResult, http::StatusCodeCategory, parser::Arguments, storage::Storage};

pub struct Logger;

impl Logger {
    pub fn show_overview(arguments: &Arguments, storage: Arc<Storage>) -> AppResult<()> {
        println!("Target: ({}) {}", arguments.method, arguments.target);

        if let Some(proxy) = &arguments.proxy {
            println!("Proxy: {}", proxy)
        }

        println!("Connections: {}", arguments.connections);
        println!("Total Requests: {}", storage.get_total_requests_count()?);
        println!(
            "Average Response Time: {} ms \n",
            storage.get_total_avg_response_time()?
        );

        Ok(())
    }

    pub fn show_results(storage: Arc<Storage>) -> AppResult<()> {
        println!("Detailed Results");

        let mut table = Table::new();

        table.add_row(row![
            "Type",
            "Total",
            "Avg Time (ms)",
            "Lowest Time (ms)",
            "Highest Time (ms)"
        ]);
        table.add_row(row![
            "Success (2xx)",
            storage.get_requests_count(StatusCodeCategory::Success)?,
            storage.get_avg_response_time(StatusCodeCategory::Success)?,
            storage.get_min_reponse_time(StatusCodeCategory::Success)?,
            storage.get_max_reponse_time(StatusCodeCategory::Success)?,
        ]);
        table.add_row(row![
            "Redirection (3xx)",
            storage.get_requests_count(StatusCodeCategory::Redirection)?,
            storage.get_avg_response_time(StatusCodeCategory::Redirection)?,
            storage.get_min_reponse_time(StatusCodeCategory::Redirection)?,
            storage.get_max_reponse_time(StatusCodeCategory::Redirection)?,
        ]);
        table.add_row(row![
            "Client Error (4xx)",
            storage.get_requests_count(StatusCodeCategory::ClientError)?,
            storage.get_avg_response_time(StatusCodeCategory::ClientError)?,
            storage.get_min_reponse_time(StatusCodeCategory::ClientError)?,
            storage.get_max_reponse_time(StatusCodeCategory::ClientError)?,
        ]);
        table.add_row(row![
            "Server Error (5xx)",
            storage.get_requests_count(StatusCodeCategory::ServerError)?,
            storage.get_avg_response_time(StatusCodeCategory::ServerError)?,
            storage.get_min_reponse_time(StatusCodeCategory::ServerError)?,
            storage.get_max_reponse_time(StatusCodeCategory::ServerError)?,
        ]);
        table.add_row(row![
            "Informational (1xx)",
            storage.get_requests_count(StatusCodeCategory::Informational)?,
            storage.get_avg_response_time(StatusCodeCategory::Informational)?,
            storage.get_min_reponse_time(StatusCodeCategory::Informational)?,
            storage.get_max_reponse_time(StatusCodeCategory::Informational)?,
        ]);
        table.add_row(row![
            "Failed",
            storage.get_requests_count(StatusCodeCategory::Failed)?,
            storage.get_avg_response_time(StatusCodeCategory::Failed)?,
            storage.get_min_reponse_time(StatusCodeCategory::Failed)?,
            storage.get_max_reponse_time(StatusCodeCategory::Failed)?,
        ]);

        //TODO: alien method! can causd panice
        table.printstd();

        Ok(())
    }
}
