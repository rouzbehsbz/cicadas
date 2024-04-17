use std::{collections::HashMap, sync::Mutex, time::Duration};

use crate::http::StatusCodeCategory;

pub struct Storage {
    response_times: HashMap<StatusCodeCategory, Mutex<Vec<u128>>>,
}

impl Storage {
    pub fn new() -> Self {
        let mut response_times = HashMap::new();

        response_times.insert(StatusCodeCategory::Success, Mutex::new(Vec::new()));
        response_times.insert(StatusCodeCategory::Redirection, Mutex::new(Vec::new()));
        response_times.insert(StatusCodeCategory::ClientError, Mutex::new(Vec::new()));
        response_times.insert(StatusCodeCategory::ServerError, Mutex::new(Vec::new()));
        response_times.insert(StatusCodeCategory::Informational, Mutex::new(Vec::new()));
        response_times.insert(StatusCodeCategory::Failed, Mutex::new(Vec::new()));

        Self { response_times }
    }

    pub fn add_response_time(
        &self,
        status_code_category: StatusCodeCategory,
        response_time: Duration,
    ) {
        //TODO: make sure no error will occur here
        self.response_times
            .get(&status_code_category)
            .unwrap()
            .lock()
            .unwrap()
            .push(response_time.as_millis())
    }

    pub fn get_avg_response_time(&self, status_code_category: StatusCodeCategory) -> f64 {
        let mut times = 0;
        //TODO: make sure no error will occur here
        let mut count = self
            .response_times
            .get(&status_code_category)
            .unwrap()
            .lock()
            .unwrap()
            .len();

        if count == 0 {
            count = 1
        }

        //TODO: make sure no error will occur here
        for time in self
            .response_times
            .get(&status_code_category)
            .unwrap()
            .lock()
            .unwrap()
            .iter()
        {
            times += time
        }

        times as f64 / count as f64
    }

    pub fn get_total_avg_response_time(&self) -> f64 {
        //TODO: Write this better
        let mut total_avg_response_time = 0.;
        let mut non_zores_response_time = 0.;

        let success_avg_response_time = self.get_avg_response_time(StatusCodeCategory::Success);
        let redirectional_avg_response_time =
            self.get_avg_response_time(StatusCodeCategory::Redirection);
        let client_error_avg_response_time =
            self.get_avg_response_time(StatusCodeCategory::ClientError);
        let server_error_avg_response_time =
            self.get_avg_response_time(StatusCodeCategory::ServerError);
        let informational_avg_response_time =
            self.get_avg_response_time(StatusCodeCategory::Informational);
        let failed_avg_response_time = self.get_avg_response_time(StatusCodeCategory::Failed);

        if success_avg_response_time != 0. {
            total_avg_response_time += success_avg_response_time;
            non_zores_response_time += 1.;
        }
        if redirectional_avg_response_time != 0. {
            total_avg_response_time += redirectional_avg_response_time;
            non_zores_response_time += 1.;
        }
        if client_error_avg_response_time != 0. {
            total_avg_response_time += client_error_avg_response_time;
            non_zores_response_time += 1.;
        }
        if server_error_avg_response_time != 0. {
            total_avg_response_time += server_error_avg_response_time;
            non_zores_response_time += 1.;
        }
        if informational_avg_response_time != 0. {
            total_avg_response_time += informational_avg_response_time;
            non_zores_response_time += 1.;
        }
        if failed_avg_response_time != 0. {
            total_avg_response_time += failed_avg_response_time;
            non_zores_response_time += 1.;
        }

        total_avg_response_time / non_zores_response_time as f64
    }

    pub fn get_requests_count(&self, status_code_category: StatusCodeCategory) -> usize {
        //TODO: make sure no error will occur here
        self.response_times
            .get(&status_code_category)
            .unwrap()
            .lock()
            .unwrap()
            .len()
    }

    pub fn get_total_requests_count(&self) -> usize {
        let success_requests_count = self.get_requests_count(StatusCodeCategory::Success);
        let redirectional_requests_count = self.get_requests_count(StatusCodeCategory::Redirection);
        let client_error_requests_count = self.get_requests_count(StatusCodeCategory::ClientError);
        let server_error_requests_count = self.get_requests_count(StatusCodeCategory::ServerError);
        let informational_requests_count =
            self.get_requests_count(StatusCodeCategory::Informational);
        let failed_requests_count = self.get_requests_count(StatusCodeCategory::Failed);

        success_requests_count
            + redirectional_requests_count
            + client_error_requests_count
            + server_error_requests_count
            + informational_requests_count
            + failed_requests_count
    }
}
