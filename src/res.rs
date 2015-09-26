use iron::prelude::*;

use iron::headers;
use iron::status::Status;
use rustc_serialize::Encodable;
use rustc_serialize::json::as_json;
use std::error::Error;
use std::fmt;
use std::io::Write;

pub use iron::method;
pub use iron::status;

#[derive(Debug)]
struct UserError;
impl Error for UserError {
    fn description(&self) -> &str {
        "the user was being a dumb"
    }
}
impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ERROR: the user was being a dumb")
    }
}

#[macro_export]
macro_rules! req_methods {
    (
        $req:expr,
        $(
            $method:ident => $res:expr
        ),*
    ) => {
        match $req.method {
            $(method::$method => $res,)*
            _ => err_method_not_allowed()
        }
    }
}

pub fn res_data<E: Encodable>(code: Status, data: E) -> IronResult<Response> {
    let mut buf = Vec::new();
    write!(buf, "{}", as_json(&data));

    let mut res = Response::with((code, buf));
    res.headers.set(headers::ContentType::json());
    Ok(res)
}

pub fn res_empty(code: Status) -> IronResult<Response> {
    let mut res = Response::with(code);
    res.headers.set(headers::ContentType::json());
    Ok(res)
}

pub fn err_method_not_allowed<T>() -> IronResult<T> {
    Err(IronError::new(UserError, status::MethodNotAllowed))
}

pub fn err_not_acceptable<T>() -> IronResult<T> {
    Err(IronError::new(UserError, status::NotAcceptable))
}