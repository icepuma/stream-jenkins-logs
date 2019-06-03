use std::error::Error;
use std::fmt;
use std::io;
use std::num;

#[derive(Debug)]
pub enum Errors {
  IO(io::Error),
  Reqwest(reqwest::Error),
  ParseInt(num::ParseIntError),
  ToStr(reqwest::header::ToStrError),
  UrlParserError(url::ParseError),
}

impl fmt::Display for Errors {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Errors::IO(ref err) => write!(f, "IO error: {}", err),
      Errors::Reqwest(ref err) => write!(f, "Reqwest error: {}", err),
      Errors::ParseInt(ref err) => write!(f, "Parse int error: {}", err),
      Errors::ToStr(ref err) => write!(f, "ToStr error: {}", err),
      Errors::UrlParserError(ref err) => write!(f, "Url ParseError error: {}", err),
    }
  }
}

impl Error for Errors {
  fn description(&self) -> &str {
    match *self {
      Errors::IO(ref err) => err.description(),
      Errors::Reqwest(ref err) => err.description(),
      Errors::ParseInt(ref err) => err.description(),
      Errors::ToStr(ref err) => err.description(),
      Errors::UrlParserError(ref err) => err.description(),
    }
  }

  fn cause(&self) -> Option<&Error> {
    match *self {
      Errors::IO(ref err) => Some(err),
      Errors::Reqwest(ref err) => Some(err),
      Errors::ParseInt(ref err) => Some(err),
      Errors::ToStr(ref err) => Some(err),
      Errors::UrlParserError(ref err) => Some(err),
    }
  }
}

impl From<reqwest::Error> for Errors {
  fn from(err: reqwest::Error) -> Errors {
    Errors::Reqwest(err)
  }
}

impl From<io::Error> for Errors {
  fn from(err: io::Error) -> Errors {
    Errors::IO(err)
  }
}

impl From<num::ParseIntError> for Errors {
  fn from(err: num::ParseIntError) -> Errors {
    Errors::ParseInt(err)
  }
}

impl From<reqwest::header::ToStrError> for Errors {
  fn from(err: reqwest::header::ToStrError) -> Errors {
    Errors::ToStr(err)
  }
}

impl From<url::ParseError> for Errors {
  fn from(err: url::ParseError) -> Errors {
    Errors::UrlParserError(err)
  }
}
