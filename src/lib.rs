#[macro_use] extern crate hyper;
extern crate iron;
extern crate mount;
extern crate postgres;
extern crate rustc_serialize;
extern crate toml;
mod config;
mod error;
#[macro_use] mod res;
mod req;
mod test;
mod token;

use iron::prelude::*;
use mount::Mount;
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
pub fn app() -> App {
    let c = match Config::new() {
        Ok(c) => c,
        Err(e) => panic!("error:\n{:#?}", e),
    };

    let mut mount = Mount::new();
    mount.mount("/api/dev/", main);
    mount.mount("/api/", main);

    let mut chain = Chain::new(mount);
    chain.link_before(VerifyAcceptable);
    Iron::new(chain)
}

#[test]
fn test_app_loads_without_panic() {
    app();
}
