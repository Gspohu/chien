#[macro_use] extern crate hyper;
extern crate iron;
extern crate mount;
extern crate postgres;
extern crate rustc_serialize;
extern crate toml;
mod config;
#[macro_use] mod res;
mod req;
mod test;
mod token;

use iron::prelude::*;
use mount::Mount;
use std::error::Error;
pub use self::config::Config;
pub use self::res::*;
pub use self::req::*;
pub use self::token::*;

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
pub fn app() -> Result<App, Box<Error>> {
    try!(Config::new());

    let mut mount = Mount::new();
    mount.mount("/api/dev/", main);
    mount.mount("/api/", main);

    let mut chain = Chain::new(mount);
    chain.link_before(VerifyAcceptable);
    Ok(Iron::new(chain))
}

#[test]
fn test_app_loads_without_error() {
    app().unwrap();
}
