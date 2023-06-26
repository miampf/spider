use std::{error::Error, fmt};

#[derive(Debug)]
pub struct SpiderError {
    msg: String
}

impl Error for SpiderError {}

impl fmt::Display for SpiderError {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "An error occured: {}", self.msg)
   } 
}

impl From<ratelimit::Error> for SpiderError {
    fn from(value: ratelimit::Error) -> Self {
       Self { msg: value.to_string() } 
    }
}

impl From<url::ParseError> for SpiderError {
    fn from(value: url::ParseError) -> Self {
        Self { msg: value.to_string() }
    }
}

impl From<reqwest::Error> for SpiderError {
    fn from(value: reqwest::Error) -> Self {
        Self { msg: value.to_string() }
    }
}
