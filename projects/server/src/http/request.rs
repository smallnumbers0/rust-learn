
use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};

pub struct Request {
    path: String, 
    query_string: Option<String>, //Option Enum 
    method: Method,
}

impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {

    }
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    //GET /search?name=abc&sort=1 HTTP/1.1  //Example http request

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let string = String::from("hello");
        unimplemented!()
    }
}


pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }   
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}

// trait Encrypt {
//     fn encrypt(&self) -> Self;
// }

// impl Encrypt for String {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
// }

// impl Encrypt for &[u8] {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
// }
