extern crate chien;
extern crate iron;
pub use iron::prelude::*;
fn main() {
    chien::app().http("localhost:3000").unwrap();
}
