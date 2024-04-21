use std::sync::Arc;

use prettytable::{row, Table};

use crate::{app::Arguments, errors::AppResult, http::StatusCodeCategory, storage::Storage};

pub struct Logger;

impl Logger {
    pub fn show_overview(arguments: &Arguments, storage: Arc<Storage>) -> AppResult<()> {
        println!(
            "Benchamrking Target: ({}) {}",
            arguments.method, arguments.target
        );

        if let Some(proxy) = &arguments.proxy {
            println!("Using proxy: {}", proxy)
        }

        println!(
            "Total {} requests have been sent over {} seconds and {} connections.",
            storage.get_total_requests_count()?,
            arguments.duration,
            arguments.connections
        );
        println!(
            "Average response time: {} ms \n",
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
            "Avg (ms)",
            "Lowest (ms)",
            "Highest (ms)"
        ]);
        if storage.get_requests_count(StatusCodeCategory::Success)? != 0 {
            table.add_row(row![
                "2xx",
                storage.get_requests_count(StatusCodeCategory::Success)?,
                storage.get_avg_response_time(StatusCodeCategory::Success)?,
                storage.get_min_reponse_time(StatusCodeCategory::Success)?,
                storage.get_max_reponse_time(StatusCodeCategory::Success)?,
            ]);
        }
        if storage.get_requests_count(StatusCodeCategory::Redirection)? != 0 {
            table.add_row(row![
                "3xx",
                storage.get_requests_count(StatusCodeCategory::Redirection)?,
                storage.get_avg_response_time(StatusCodeCategory::Redirection)?,
                storage.get_min_reponse_time(StatusCodeCategory::Redirection)?,
                storage.get_max_reponse_time(StatusCodeCategory::Redirection)?,
            ]);
        }
        if storage.get_requests_count(StatusCodeCategory::ClientError)? != 0 {
            table.add_row(row![
                "4xx",
                storage.get_requests_count(StatusCodeCategory::ClientError)?,
                storage.get_avg_response_time(StatusCodeCategory::ClientError)?,
                storage.get_min_reponse_time(StatusCodeCategory::ClientError)?,
                storage.get_max_reponse_time(StatusCodeCategory::ClientError)?,
            ]);
        }
        if storage.get_requests_count(StatusCodeCategory::ServerError)? != 0 {
            table.add_row(row![
                "5xx",
                storage.get_requests_count(StatusCodeCategory::ServerError)?,
                storage.get_avg_response_time(StatusCodeCategory::ServerError)?,
                storage.get_min_reponse_time(StatusCodeCategory::ServerError)?,
                storage.get_max_reponse_time(StatusCodeCategory::ServerError)?,
            ]);
        }
        if storage.get_requests_count(StatusCodeCategory::Informational)? != 0 {
            table.add_row(row![
                "1xx",
                storage.get_requests_count(StatusCodeCategory::Informational)?,
                storage.get_avg_response_time(StatusCodeCategory::Informational)?,
                storage.get_min_reponse_time(StatusCodeCategory::Informational)?,
                storage.get_max_reponse_time(StatusCodeCategory::Informational)?,
            ]);
        }
        if storage.get_requests_count(StatusCodeCategory::Failed)? != 0 {
            table.add_row(row![
                "Failed",
                storage.get_requests_count(StatusCodeCategory::Failed)?,
                storage.get_avg_response_time(StatusCodeCategory::Failed)?,
                storage.get_min_reponse_time(StatusCodeCategory::Failed)?,
                storage.get_max_reponse_time(StatusCodeCategory::Failed)?,
            ]);
        }

        //TODO: alien method! can causd panice
        table.printstd();

        Ok(())
    }
}
