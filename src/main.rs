use quicli::prelude::*;
use std::{thread, time};
use structopt::StructOpt;

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger("head")?;

    let check_interval = time::Duration::new(args.check_interval_seconds.into(), 0);
    let job_board_client = JobBoardClient {
        url: args.job_board_url,
        processor_id: "xyz".to_string(),
    };

    loop {
        println!("waiting for a job ok");
        let job_id = job_board_client.fetch_job_id();

        if job_id > 0 {
            println!("got a nonzero job id wow");
            let job = job_board_client.fetch_job(job_id);
            println!("got job {}", job.id);
        }

        if args.once {
            println!("noping out after waiting once");
            break;
        }
        thread::sleep(check_interval);
    }

    Ok(())
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "check-interval-seconds", short = "c", default_value = "10")]
    check_interval_seconds: u8,

    #[structopt(
        long = "job-board-url",
        short = "u",
        default_value = "http://guest:guest@localhost:5555/"
    )]
    job_board_url: String,

    #[structopt(long = "queue-name", short = "q", default_value = "builds.default")]
    queue_name: String,

    #[structopt(flatten)]
    verbosity: Verbosity,

    #[structopt(
        long = "once",
        short = "O",
        help = "Check for available job once and exit"
    )]
    once: bool,
}

use reqwest;

#[derive(Debug)]
struct JobBoardClient {
    url: String,
    processor_id: String,
}

impl JobBoardClient {
    fn fetch_job_id(&self) -> u64 {
        0
    }

    fn fetch_job(&self, id: u64) -> Job {
        Job { id: id }
    }
}

#[derive(Debug)]
struct Job {
    id: u64,
}
