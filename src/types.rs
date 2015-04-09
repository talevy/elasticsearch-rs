use time::Duration;
use std::error::Error;
use std::str::FromStr;
use std::fmt;
use std::string::ToString;

use self::ParseTimeoutError::*;
use self::VersionType::*;
use self::Consistency::*;
use self::OpType::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Consistency {
    One,
    Quorum,
    All
}

impl ToString for Consistency {
    fn to_string(&self) -> String {
        match *self {
            One => "one".to_string(),
            Quorum => "quorum".to_string(),
            All => "all".to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpType {
    Index,
    Create
}

impl ToString for OpType {
    fn to_string(&self) -> String {
        match *self {
            Index => "index".to_string(),
            Create => "create".to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VersionType {
    Internal,
    External,
    ExternalGte,
    Force
}

impl ToString for VersionType {
    fn to_string(&self) -> String {
        match *self {
            Internal => "internal".to_string(),
            External => "external".to_string(),
            ExternalGte => "external_gte".to_string(),
            Force => "force".to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Timeout(Duration);

impl Timeout {
    fn parse_from_str(s: &str) -> Result<Timeout, ParseTimeoutError> {
        let re = regex!(r"^(\d+)(ms|s|m|H|h|d|w){0,1}$");
        if let Some(cap) = re.captures(s) {
            let millis: i64 = FromStr::from_str(cap.at(1).unwrap()).unwrap();
            let multi_str = match cap.at(2) {
                Some(x) => x,
                None => "no units :(",
            };
            let multiplier: i64 = match multi_str {
                "ms" => 1,
                "s" => 1000,
                "m" => 60 * 1000,
                "H" | "h" => 60 * 60 * 1000,
                "d" => 24 * 60 * 60 * 1000,
                "w" => 7 * 24 * 60 * 60 * 1000,
                _ => 1,
            };

            return Ok(Timeout(Duration::milliseconds(millis * multiplier)));
        } else {
            return Err(Invalid);
        }
    }
}

impl ToString for Timeout {
    fn to_string(&self) -> String {
        let Timeout(duration) = *self;
        duration.num_milliseconds().to_string()
    }
}

impl FromStr for Timeout {
    type Err = ParseTimeoutError;
    fn from_str(s: &str) -> Result<Timeout, ParseTimeoutError> {
        Timeout::parse_from_str(s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseTimeoutError {
    Invalid
}

impl Error for ParseTimeoutError {
    fn description(&self) -> &str {
        match *self {
            Invalid => "invalid format",
        }
    }
}

impl fmt::Display for ParseTimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Invalid => write!(f, stringify!(self.description())),
        }
    }
}

