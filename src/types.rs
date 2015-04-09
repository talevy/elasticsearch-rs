use std::error::Error;
use std::str::FromStr;
use std::fmt;
use std::string::ToString;
use chrono::{Duration, DateTime, UTC};

use self::ParseTimeoutError::*;
use self::VersionType::*;
use self::Consistency::*;
use self::OpType::*;

pub trait QueryParam {
    fn get_name(&self) -> &'static str;
    fn get_value(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parent(String);
#[derive(Debug, Clone, PartialEq)]
pub struct Refresh(bool);
#[derive(Debug, Clone, PartialEq)]
pub struct Routing(String);
#[derive(Debug, Clone, PartialEq)]
pub struct Timestamp(DateTime<UTC>);
#[derive(Debug, Clone, PartialEq)]
pub struct Ttl(Duration);
#[derive(Debug, Clone, PartialEq)]
pub struct Version(i64);

impl_query_param!(Consistency, "consistency", { |x| x.to_string() });
impl_query_param!(OpType, "op_type", { |x| x.to_string() });
impl_query_param!(Parent, "parent", { |x| x.0.to_string() });
impl_query_param!(Refresh, "refresh", { |x| x.0.to_string() });
impl_query_param!(Routing, "routing", { |x| x.0.to_string() });
impl_query_param!(Timestamp, "timestamp", { |x| x.0.to_string() });
impl_query_param!(Timeout, "timeout", { |x| x.to_string() });
impl_query_param!(Ttl, "ttl", { |x| x.0.num_milliseconds().to_string() });
impl_query_param!(Version, "version", { |x| x.0.to_string() });
impl_query_param!(VersionType, "version_type", { |x| x.to_string() });

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

