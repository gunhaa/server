use std::net::TcpStream;
use std::io::{Write, Result as IoResult};
use std::fmt::{Display, Formatter, Result as FmtResult};

use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code : StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {status_code, body}
    }

                                // 스태틱 디스패치 사용
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()>{
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}

// impl Display for Response {
//     fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // let body = match &self.body {
        //     Some(b) => b,
        //     None => ""
        // };

        // write!(f, "HTTP/1.1 {} {}\r\n\r\n{}",
        // self.status_code,
        // self.status_code.reason_phrase(),
        // body
        // )
//     }
// }