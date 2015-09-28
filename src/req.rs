use hyper::mime::Attr::Charset;
use hyper::mime::Value::Utf8;
use hyper::mime::{Mime, TopLevel, SubLevel};
use iron::headers::{Accept, AcceptCharset};
use iron::headers::Charset;
use iron::middleware::BeforeMiddleware;
use iron::prelude::*;
use super::res::*;

pub struct VerifyAcceptable;

fn mime_is_acceptable(m: &Mime) -> bool {
    let &Mime(ref tlevel, ref slevel, ref params) = m;

    // neither */*, application/*, nor application/json => JSON is not OK
    if tlevel != &TopLevel::Star && (
        tlevel != &TopLevel::Application ||
        (
            slevel != &SubLevel::Json &&
            slevel != &SubLevel::Star
        )
    ) {
        return false
    }

    let mut has_charset = false;

    // any params are UTF => UTF-8 is OK
    for &(ref attr, ref value) in params {
        if attr == &Charset {
            // charset is UTF-8 => UTF-8 is OK
            if value == &Utf8 {
                return true
            }
            has_charset = true
        }
    }

    // no charset given => UTF-8 is OK
    // charset is not UTF-8 => UTF-8 is not OK
    !has_charset
}

fn charset_is_acceptable(c: &Charset) -> bool {
    // charset is UTF-8 => UTF-8 is OK
    if let &Charset::Ext(ref s) = c {
        s == "utf8" ||
        s == "UTF8" ||
        s == "utf-8" ||
        s == "UTF-8"
    // charset is anything else => UTF-8 is not OK
    } else {
        false
    }
}

impl BeforeMiddleware for VerifyAcceptable {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        // make sure that charset is UTF-8
        if let Some(&AcceptCharset(ref charsets)) =
                req.headers.get::<AcceptCharset>() {
            println!("C {:#?}", charsets);
            if charsets.is_empty() { return Ok(()) }
            for qitem in charsets {
                if charset_is_acceptable(&qitem.item) { return Ok(()) }
            }
            return err_not_acceptable()
        }

        // make sure that acceptable type is UTF-8 JSON
        if let Some(&Accept(ref mimes)) = req.headers.get::<Accept>() {
            println!("M {:#?}", mimes);
            if mimes.is_empty() { return Ok(()) }
            for qitem in mimes {
                if mime_is_acceptable(&qitem.item) { return Ok(()) }
            }
            return err_not_acceptable()
        }

        Ok(())
    }
}

#[test]
fn test_verify_acceptable() {
}
