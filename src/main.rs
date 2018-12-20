use quicli::prelude::*;
use std::{thread, time};
use structopt::StructOpt;

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger("head")?;

    let check_interval = time::Duration::new(args.check_interval_seconds.into(), 0);

    loop {
        println!("waiting for a job ok");
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

    #[structopt(flatten)]
    verbosity: Verbosity,

    #[structopt(long = "once", short = "O")]
    once: bool,
}
