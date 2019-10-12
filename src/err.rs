use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Fail {
    Error(Box<dyn error::Error>),
    Fail(Option<String>, Box<Fail>),
    Message(String),
}

impl Fail {
    pub fn new<D: fmt::Display>(msg: D) -> Fail {
        Fail::Message(msg.to_string())
    }
}

impl fmt::Display for Fail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fail::Error(e) => e.fmt(f),
            Fail::Fail(None, fail) => fail.fmt(f),
            Fail::Fail(Some(msg), fail) => write!(f, "{}: {}", msg, fail),
            Fail::Message(msg) => write!(f, "{}", msg),
        }
    }
}