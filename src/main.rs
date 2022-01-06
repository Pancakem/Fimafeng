#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate tinytemplate;

/// Fimafeng server configuration
mod config;
/// File manager handles file serving and templating
mod file_manager;
/// http definitions
mod http;
/// Simple logger for requests and responses
mod log;
/// HTTP request parser in **nom**
mod parser;
/// HTTP Request object
mod request;
/// HTTP Response object
mod response;
/// Handles incoming connections;
mod server;

use crate::config::Config;
use crate::server::Server;

use std::sync::{Arc, Barrier};
use structopt::StructOpt;
use threadpool::ThreadPool;

/// The web server commandline options
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
                cfg.tls,
            );

            server.listen_and_serve();

            barrier.wait();
        });
    }
    barrier.wait();
}
