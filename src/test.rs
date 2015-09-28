use iron::Url;
use iron::headers::Headers;
use iron::method::Method;
use iron::prelude::*;
use iron::typemap::TypeMap;
use std::mem;
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};

pub unsafe fn new_req<'a, 'b>(m: Method) -> Request<'a, 'b> {
    Request {
        url: Url::parse("http://localhost:300").unwrap(),
        remote_addr: SocketAddr::V4(
            SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 3000),
        ),
        local_addr: SocketAddr::V4(
            SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 30000),
        ),
        headers: Headers::new(),
        body: mem::uninitialized(),
        method: m,
        extensions: TypeMap::new(),
    }
}

