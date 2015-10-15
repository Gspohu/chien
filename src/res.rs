use iron::prelude::*;

use super::error::*;
use iron::headers;
use iron::status::Status;
use rustc_serialize::Encodable;
use rustc_serialize::json::as_json;
use std::io::Write;

pub use iron::method;
pub use iron::status;

#[macro_export]
macro_rules! req_methods {
    (
        $req:expr,
        $(
            $method:ident => $res:expr
        ),*
    ) => {
        match $req.method {
            $(::iron::method::$method => $res,)*
            _ => $crate::res::err_method_not_allowed()
        }
    }
}

#[test]
fn test_req_methods() {
    let req1 = unsafe { super::test::new_req(method::Delete) };
    let req2 = unsafe { super::test::new_req(method::Get) };
    let req3 = unsafe { super::test::new_req(method::Post) };
    for &(ref req, ref ok, ref err) in &[
        (req1, Some("DELETE didn't return error"), None),
        (req2, None, Some("GET returned error")),
        (req3, None, Some("POST returned error")),
    ] {
        match req_methods!(
            req,
            Get => {
                Ok(Response::with(status::Ok))
            },
            Post => {
                Ok(Response::with(status::Ok))
            }
        ) {
            Ok(_) => {
                if let &Some(ref e) = ok {
                    panic!("{}", e);
                }
            }
            Err(_) => {
                if let &Some(ref e) = err {
                    panic!("{}", e);
                }
            }
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
