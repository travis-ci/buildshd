use reqwest;

extern crate serde;
extern crate serde_json;

#[derive(Debug)]
pub struct Client {
    url: String,
    processor_id: String,
    site: String,
    queue: String,
}

impl Client {
    pub fn new(url: String, processor_id: String, site: String, queue: String) -> Client {
        Client {
            url: url,
            processor_id: processor_id,
            site: site,
            queue: queue,
        }
    }

    pub fn fetch_job_id(&self) -> u64 {
        /*
        let client = reqwest::Client::new();
        let resp: JobIDResponse = client
            .post(format!("{}/jobs/pop", self.url))
            .query(&[("queue", self.queue)])
            .header("Content-Type", "application/json")
            .header("Travis-Site", self.site)
            .header("From", self.processor_id)
            .send()
            .json()?;
        match resp.job_id.parse::<u64>() {
            Ok(parsed) => parsed,
            Err(e) => 0u64,
        }
        */
        0u64
    }

    pub fn fetch_job(&self, id: u64) -> Job {
        Job { id: id }
    }
}

/*
#[derive(Deserialize)]
pub struct JobIDResponse {
    job_id: String,
}
*/

#[derive(Debug)]
pub struct Job {
    pub id: u64,
}
