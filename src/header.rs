///! HTTP header used in docker api
///!
use base64::{self, STANDARD};
use hyperx::header::Header;
use hyperx::Error;
use hyperx::Result;
use serde::{Deserialize, Serialize};
use std::fmt;

/// The http header represent `X-Registry-Auth`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XRegistryAuth {
    /// Header value string.
    /// This body is sent/recv with enc/dec-ed base64 implicitly
    body: String,
}

impl fmt::Display for XRegistryAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            base64::encode_config(self.body.as_bytes(), STANDARD)
        )
    }
}

impl XRegistryAuth {
    pub fn new(body: String) -> Self {
        Self { body }
    }
}

impl Header for XRegistryAuth {
    fn header_name() -> &'static str {
        "X-Registry-Auth"
    }

    fn parse_header<'a, T>(raw: &'a T) -> Result<Self>
    where
        T: hyperx::header::RawLike<'a>,
    {
        if raw.len() != 1 {
            return Err(Error::Header);
        }

        base64::decode_config(raw.one().unwrap(), STANDARD)
            .map_err(|_| Error::Header)
            .map(|vec| Self::new(String::from_utf8_lossy(&vec).to_string()))
    }

    fn fmt_header(&self, f: &mut hyperx::header::Formatter) -> fmt::Result {
        f.fmt_line(self)
    }
}
