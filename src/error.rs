use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct UserError;
impl Error for UserError {
    fn description(&self) -> &str {
        "bad request"
    }
}
impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ERROR: bad request")
    }
}

#[test]
fn test_error() {
    assert_eq!(UserError.description(), "bad request");
    assert_eq!(format!("{}", UserError), "ERROR: bad request");
}
