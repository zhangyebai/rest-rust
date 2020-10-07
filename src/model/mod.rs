pub mod region_model;

use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::Utf8Error;
use actix_web::error;
use actix_web::http::StatusCode;
use actix_http::{Response, ResponseBuilder};
use actix_web::body::Body;

#[derive(Serialize, Deserialize, Debug)]
pub struct R<T> {
    pub code: i8,
    pub message: String,
    pub data: Option<T>,
}

impl<T> R<T> {
    pub fn ok(data: Option<T>) -> R<T> {
        R {
            code: 0,
            message: "success".to_string(),
            data,
        }
    }

    pub fn err(data: Option<T>) -> R<T> {
        R {
            code: -1,
            message: "error".to_string(),
            data,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct E {
    pub code: i8,
    pub message: String,
}

impl E {
    pub fn build(code: i8, message: String) -> E {
        E {
            code,
            message,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Page<T> {
    pub page: u32,

    pub size: u32,

    pub pages: u64,

    pub total: u64,

    pub data: Vec<T>,
}

impl<T> Page<T> {
    pub fn empty() -> Page<T> {
        Page {
            page: 0u32,
            size: 0u32,
            pages: 0u64,
            total: 0u64,
            data: Vec::new(),
        }
    }

    pub fn page(page: u32, size: u32, total: u64, data: Vec<T>) -> Page<T> {
        let ps = match size {
            s if s > 0 => {
                let ps = total / size as u64;
                match total % size as u64 {
                    de if de > 0 => ps + 1,
                    _ => ps
                }
            }
            _ => 0u64,
        };
        Page::build(page, size, ps, total, data)
    }

    pub fn build(page: u32, size: u32, pages: u64, total: u64, data: Vec<T>) -> Page<T> {
        Page { page, size, pages, total, data }
    }
}


#[derive(Debug)]
pub enum Ex {
    ParseIntError(std::num::ParseIntError),
    Utf8Error(std::str::Utf8Error),
    IoError(std::io::Error),
}

impl std::error::Error for Ex{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            Ex::IoError(ref e) => Some(e),
            Ex::Utf8Error(ref e) => Some(e),
            Ex::ParseIntError(ref e) => Some(e),
        }
    }
}

impl Display for Ex{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Ex::IoError(ref e) => e.fmt(f),
            Ex::Utf8Error(ref e) => e.fmt(f),
            Ex::ParseIntError(ref e) => e.fmt(f),
        }
    }
}

impl error::ResponseError for Ex{
    fn status_code(&self) -> StatusCode {
        //StatusCode::from_u16(200).unwrap()
        StatusCode::OK
    }

    fn error_response(&self) -> Response<Body> {
        ResponseBuilder::new(self.status_code())
            .set_header("content-type", "application/json; charset=utf-8")
            .body(serde_json::to_string(&E::build(-1, self.to_string().clone())).unwrap())
    }

}

impl From<ParseIntError> for Ex {
    fn from(s: std::num::ParseIntError) -> Self {
        Ex::ParseIntError(s)
    }
}

impl From<std::io::Error> for Ex {
    fn from(s: std::io::Error) -> Self {
        Ex::IoError(s)
    }
}

impl From<Utf8Error> for Ex {
    fn from(s: std::str::Utf8Error) -> Self {
        Ex::Utf8Error(s)
    }
}