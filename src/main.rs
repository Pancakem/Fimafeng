#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate tinytemplate;

mod file_manager;
mod http;
mod log;
mod parser;
mod request;
mod response;
mod server;

use crate::server::Server;

use structopt::StructOpt;

/// A web server
#[derive(StructOpt)]
struct Cli {
    /// The host to bind to
    host: String,
    /// The port to serve on
    port: u16,
    /// The path to the dir to be served
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let server = Server::new(args.host.as_str(), args.path.to_str().unwrap(), args.port);

    server.listen_and_serve();
}
