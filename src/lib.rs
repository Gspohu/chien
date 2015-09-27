#[macro_use] extern crate hyper;
extern crate iron;
extern crate mount;
extern crate rustc_serialize;
mod token;
#[macro_use] mod res;
mod req;

use iron::prelude::*;
use mount::Mount;
use self::res::*;
use self::req::*;

#[derive(RustcEncodable)]
struct HelloWorld {
    msg: &'static str,
}

fn main(req: &mut Request) -> IronResult<Response> {
    req_methods!(
        req,
        Get => res_data(
            status::Ok,
            HelloWorld { msg: "Hello! :)" }
        )
    )
}

pub type App = Iron<Chain>;
pub fn app() -> App {
    let mut mount = Mount::new();
    mount.mount("/api/dev/", main);
    mount.mount("/api/", main);

    let mut chain = Chain::new(mount);
    chain.link_before(VerifyAcceptable);
    Iron::new(chain)
}
