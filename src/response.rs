use crate::http::{HTTPVersion, STATUS_CODE_MAPPING};
use chrono::Utc;

pub struct Response {
    http_version: HTTPVersion,
    status_code: u16,
    content: String,
    content_type: String,
    content_length: u64,
    server_name: String,
}

impl Response {
    pub fn new(
        http_version: HTTPVersion,
        status_code: u16,
        content: String,
        content_type: String,
        content_length: u64,
        server_name: String,
    ) -> Self {
        Self {
            http_version,
            status_code,
            content,
            content_type,
            content_length,
            server_name,
        }
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        format!(
            r#"{} {} {}
Server: {}
Date: {}
Content-Type: {}
Content-Length: {}
Connection: keep-alive

{}
"#,
            self.http_version.to_string(),
            self.status_code,
            STATUS_CODE_MAPPING.get(&self.status_code).unwrap(),
            self.server_name,
            Utc::now().to_rfc2822(),
            self.content_type,
            self.content_length,
            self.content,
        )
    }
}
