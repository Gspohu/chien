use std::str::FromStr;
use std::string::ToString;

static ENCODE_TABLE: &'static [u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

// inverse of above
const DECODE_TABLE: [u8; 256] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x3E, 0xFF, 0x3E, 0xFF, 0x3F,
    0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
    0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0xFF, 0xFF, 0xFF, 0xFF, 0x3F,
    0xFF, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
    0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30, 0x31, 0x32, 0x33, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidTokenError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token(u64, u64);

impl Token {
    pub fn new(mut p1: u64, mut p2: u64) -> Token {
        p1 &= 0x0FFF_FFFF_FFFF_FFFF;
        p2 &= 0x0FFF_FFFF_FFFF_FFFF;
        Token(p1, p2)
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        let mut bs = vec![b'A'; 20];

        let mut p1 = self.0;
        for i in (0..10).rev() {
            bs[i] = ENCODE_TABLE[(p1 % 64) as usize];
            p1 /= 64;
        }

        let mut p2 = self.1;
        for i in (10..20).rev() {
            bs[i] = ENCODE_TABLE[(p2 % 64) as usize];
            p2 /= 64;
        }

        String::from_utf8(bs).ok().unwrap()
    }
}

impl FromStr for Token {
    type Err = InvalidTokenError;

    fn from_str(s: &str) -> Result<Token, InvalidTokenError> {
        if s.len() != 20 {
            return Err(InvalidTokenError);
        }

        let mut first = 0u64;
        for c in s[0..10].as_bytes() {
            first *= 64;
            let decoded = DECODE_TABLE[*c as usize] as u64;
            if decoded == 0xFF {
                return Err(InvalidTokenError);
            }
            first += decoded;
        }

        let mut second = 0u64;
        for c in s[10..20].as_bytes() {
            second *= 64;
            let decoded = DECODE_TABLE[*c as usize] as u64;
            if decoded == 0xFF {
                return Err(InvalidTokenError);
            }
            second += decoded;
        }

        Ok(Token(first, second))
    }
}

#[test]
fn test_token_new() {
    assert_eq!(
        Token::new(0x4ef5ae907669bfb5,
                   0xfecd6f623bea35d9),
        Token::new(0x0ef5ae907669bfb5,
                   0x0ecd6f623bea35d9)
    );
}

#[test]
fn test_token_idempotent() {
    assert_eq!(
        Token::from_str(
            Token::new(
                0x124a75107f35bd12,
                0x3a743bd129087d84,
            ).to_string().as_ref()
        ).ok().unwrap(),
        (
            Token::new(
                0x124a75107f35bd12,
                0x3a743bd129087d84
            )
        )
    );
    assert_eq!(
        Token::from_str(
            Token::new(
                0x4ef5ae907669bfb5,
                0xfecd6f623bea35d9,
            ).to_string().as_ref()
        ).ok().unwrap(),
        (
            Token::new(
                0x4ef5ae907669bfb5,
                0xfecd6f623bea35d9,
            )
        )
    );
}

#[test]
fn test_token_invalid() {
    assert_eq!(
        Token::from_str(
            "012345678901234567890"
        ),
        Err(InvalidTokenError)
    );
    assert_eq!(
        Token::from_str(
            "0123456789012345678!"
        ),
        Err(InvalidTokenError)
    );
    assert_eq!(
        Token::from_str(
            "!1234567890123456789"
        ),
        Err(InvalidTokenError)
    );
}
