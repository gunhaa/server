use std::fmt::{write, Display, Formatter, Result as FmtResult};

// Copy trait는 Clone trait에 의존성을 가지고 있다.
// Copy는 값을 복사하는 기능을, clone을 참조값을 복사하는 기능을 가진 trait이다.
#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}