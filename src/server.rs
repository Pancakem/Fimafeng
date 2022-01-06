use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use threadpool::ThreadPool;

use crate::file_manager::FileManager;
use crate::log::{log_request, log_response};
use crate::request::Request;
use crate::response::Response;

/// Web server name
static NAME: &str = "Fimafeng";

/// Server object
pub struct Server {
    port: u16,
    host: String,
    file_manager: FileManager,
    // number of threads
    threads: usize,
}

impl Server {
    /// Returns a new Server with a properly initialized file manager
    pub fn new(host: &str, web_dir: &str, port: u16, threads: usize) -> Self {
        Self {
            host: host.to_string(),
            port,
            file_manager: FileManager::new(web_dir),
            threads,
        }
    }

    /// Checks if provided port is available for binding
    pub fn port_availability(&self) -> bool {
        TcpListener::bind((self.host.as_str(), self.port)).is_ok()
    }

    /// listen_and_serve listens for connections and services the requests
    /// the number of requests it can handle at a time is specified in threads
    pub fn listen_and_serve(&self) {
        println!("Starting instance at {}:{}", self.host, self.port);
        if !self.port_availability() {
            panic!("Cannot bind to port {}", self.port);
        }

        let listener = TcpListener::bind((self.host.as_str(), self.port)).unwrap();

        // threadpool
        let pool = ThreadPool::new(self.threads);

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            let fm = self.file_manager.clone();
            pool.execute(move || handle_connection(&fm, stream));
        }

        println!("Server shutting down");
    }
}

pub fn handle_connection(fm: &FileManager, mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let http_req_str = str::from_utf8(&buffer).unwrap();
    let req = Request::try_from(http_req_str).unwrap();
    log_request(&req);
    let resp: Response;

    if req.target() == "" {
        let file = fm.template_dir(fm.base_path().as_str()).unwrap();
        resp = Response::new(
            req.http_ver(),
            200,
            file.content,
            file.content_type,
            file.content_length,
            NAME.to_string(),
        );
        log_response(&resp);
        stream.write_all(resp.to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }
    // check target
    let target = req.target();
    if FileManager::is_dir(target.as_str()) {
        let file = fm.template_dir(target.as_str()).unwrap();
        resp = Response::new(
            req.http_ver(),
            200,
            file.content,
            file.content_type,
            file.content_length,
            NAME.to_string(),
        );
        log_response(&resp);
        stream.write_all(resp.to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    } else if fm.file_exist(target.as_str()) {
        let file = fm.get_file(target.as_str()).unwrap();
        resp = Response::new(
            req.http_ver(),
            200,
            file.content,
            file.content_type,
            file.content_length,
            NAME.to_string(),
        );
        log_response(&resp);
        stream.write_all(resp.to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }

    // 404
    let file = fm.not_found().unwrap();
    resp = Response::new(
        req.http_ver(),
        404,
        file.content,
        file.content_type,
        file.content_length,
        NAME.to_string(),
    );
    log_response(&resp);
    stream.write_all(resp.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();
}
