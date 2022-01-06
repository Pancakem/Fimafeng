use crate::config::Tls;
use crate::file_manager::FileManager;
use crate::log::{log_request, log_response};
use crate::request::Request;
use crate::response::Response;
use rustls::{Certificate, PrivateKey};
use rustls::{ServerConfig, ServerConnection, Stream as TlsStream};
use rustls_pemfile::Item::{PKCS8Key, RSAKey};
use rustls_pemfile::{certs, read_one};
use std::fs;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use std::sync::Arc;
use threadpool::ThreadPool;

/// Web server name
static NAME: &str = "Fimafeng";

/// Server object
pub struct Server {
    port: u16,
    host: String,
    file_manager: FileManager,
    // number of threads
    threads: usize,
    server_config: Option<ServerConfig>,
    has_tls: bool,
}

impl Server {
    /// Returns a new Server with a properly initialized file manager
    pub fn new(host: &str, web_dir: &str, port: u16, threads: usize, opt_tls: Option<Tls>) -> Self {
        let mut has_tls = false;
        let mut server_config = None;
        if let Some(tls) = opt_tls {
            server_config = Server::make_config(tls.cert.as_str(), tls.key.as_str());
            has_tls = server_config.is_some();
        }

        Self {
            host: host.to_string(),
            port,
            file_manager: FileManager::new(web_dir),
            threads,
            has_tls,
            server_config,
        }
    }

    /// Builds a  serverconfig
    /// should be invoked once
    fn make_config(cert: &str, key: &str) -> Option<ServerConfig> {
        ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(
                Server::load_certs(cert).unwrap(),
                Server::load_private_key(key).unwrap(),
            )
            .ok()
    }

    /// Reads certificate
    fn load_certs(filename: &str) -> Option<Vec<Certificate>> {
        let cert_raw = fs::read_to_string(filename).unwrap();
        let mut reader = BufReader::new(cert_raw.as_bytes());
        let cert = certs(&mut reader)
            .ok()?
            .iter()
            .map(|v| Certificate(v.clone()))
            .collect();

        Some(cert)
    }

    /// Reads private key
    fn load_private_key(filename: &str) -> Option<PrivateKey> {
        let key_raw = fs::read_to_string(filename).unwrap();
        let mut reader = BufReader::new(key_raw.as_bytes());
        match read_one(&mut reader).ok()? {
            Some(RSAKey(key)) => Some(PrivateKey(key)),
            Some(PKCS8Key(key)) => Some(PrivateKey(key)),
            _ => None,
        }
    }

    /// Checks if provided port is available for binding
    pub fn port_availability(&self) -> bool {
        TcpListener::bind((self.host.as_str(), self.port)).is_ok()
    }

    /// Listens for connections and services the requests
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

            let mut tls_config = None;
            if self.has_tls {
                tls_config = Some(Arc::new(self.server_config.clone().unwrap()));
            }

            let fm = self.file_manager.clone();
            pool.execute(move || handle_connection(&fm, stream, tls_config));
        }

        println!("Server shutting down");
    }
}

pub fn handle_connection(
    fm: &FileManager,
    mut stream: TcpStream,
    // The option helps the function determine if tls is enabled
    tls_config: Option<Arc<ServerConfig>>,
) {
    let mut buffer = [0; 512];

    let req: Request;

    if let Some(tls_cfg) = tls_config {
        // create tls session
        let mut session = ServerConnection::new(tls_cfg).unwrap();
        let mut stream = TlsStream::new(&mut session, &mut stream);
        stream.read(&mut buffer).unwrap();
        let http_req_str = str::from_utf8(&buffer).unwrap();
        req = Request::try_from(http_req_str).unwrap();
    } else {
        stream.read(&mut buffer).unwrap();
        let http_req_str = str::from_utf8(&buffer).unwrap();
        req = Request::try_from(http_req_str).unwrap();
    }

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
