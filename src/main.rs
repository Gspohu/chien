extern crate chien;
extern crate iron;
pub use iron::prelude::*;
fn main() {
    chien::app().unwrap().http("localhost:3000").unwrap();
}
