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

    pub fn msg<D: fmt::Display>(self, msg: D) -> Fail {
        match self {
            Fail::Fail(None, fail) => Fail::Fail(Some(msg.to_string()), fail),
            fail => Fail::Fail(Some(msg.to_string()), Box::new(fail)),
        }
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

impl<E: 'static + error::Error> From<E> for Fail {
    fn from(err: E) -> Fail {
        Fail::Error(Box::new(err))
    }
}

pub trait ErrorMessageExt<T> {
    fn err_msg<D: fmt::Display>(self, msg: D) -> Result<T, Fail>;
}

impl<T, E: 'static + error::Error> ErrorMessageExt<T> for Result<T, E> {
    fn err_msg<D: fmt::Display>(self, msg: D) -> Result<T, Fail> {
        self.map_err(|err| Fail::Fail(Some(msg.to_string()), Box::new(err.into())))
    }
}

impl<T> ErrorMessageExt<T> for Option<T> {
    fn err_msg<D: fmt::Display>(self, msg: D) -> Result<T, Fail> {
        self.ok_or_else(|| Fail::Message(msg.to_string()))
    }
}

impl<T> ErrorMessageExt<T> for Result<T, Fail> {
    fn err_msg<D: fmt::Display>(self, msg: D) -> Result<T, Fail> {
        self.map_err(|fail| fail.msg(msg))
    }
}
