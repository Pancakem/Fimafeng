#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate tinytemplate;

mod config;
mod file_manager;
mod http;
mod log;
mod parser;
mod request;
mod response;
mod server;

use crate::config::Config;
use crate::server::Server;

use std::sync::{Arc, Barrier};
use structopt::StructOpt;
use threadpool::ThreadPool;

/// A web server
#[derive(StructOpt)]
struct Cli {
    /// A list of config files
    configs: Vec<String>,
}

fn main() {
    let args = Cli::from_args();

    let cfgs: Vec<Config> = args
        .configs
        .iter()
        .map(|cfg_str| Config::try_from(cfg_str.as_str()).unwrap())
        .collect();

    println!("Fimafeng Started");
    let job_count = cfgs.len();
    let pool = ThreadPool::new(job_count);

    // waits for all threads plus the starter thread
    let barrier = Arc::new(Barrier::new(job_count + 1));
    for cfg in cfgs {
        let barrier = barrier.clone();
        pool.execute(move || {
            let server = Server::new(
                cfg.host.as_str(),
                cfg.directory.as_str(),
                cfg.port,
                cfg.thread_count,
            );

            server.listen_and_serve();

            barrier.wait();
        });
    }
    barrier.wait();
}
