use std::error::Error;
use std::str::FromStr;
use std::fmt;
use std::string::ToString;
use chrono::{Duration, DateTime, UTC};

pub trait QueryParam {
    fn get_name(&self) -> &'static str;
    fn get_value(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringList(pub Vec<String>);

impl fmt::Display for StringList {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut has_entries = false;

        if self.0.is_empty() { return write!(fmt, ""); }

        for e in self.0.iter().take(self.0.len() - 1) {
            let prefix = if has_entries { "," } else { "" };
            write!(fmt, "{}{}", prefix, e);
            has_entries = true;
        };
        let prefix = if has_entries { "," } else { "" };
        return write!(fmt, "{}{}", prefix, self.0.iter().last().unwrap());
    }
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
#[derive(Debug, Clone, PartialEq)]
pub struct Fields(pub StringList);
#[derive(Debug, Clone, PartialEq)]
pub struct Preference(String);
#[derive(Debug, Clone, PartialEq)]
pub struct Realtime(bool);
#[derive(Debug, Clone, PartialEq)]
pub struct Source(bool);
#[derive(Debug, Clone, PartialEq)]
pub struct SourceExclude(pub StringList);
#[derive(Debug, Clone, PartialEq)]
pub struct SourceInclude(pub StringList);
#[derive(Debug, Clone, PartialEq)]
pub struct IgnoreUnavailable(bool);
#[derive(Debug, Clone, PartialEq)]
pub struct AllowNoIndices(bool);
#[derive(Debug, Clone, PartialEq)]
pub struct MinScore(f64);
#[derive(Debug, Clone, PartialEq)]
pub struct Local(bool);
#[derive(Debug, Clone, PartialEq)]
pub struct MasterTimeout(Timeout);


impl_query_param!(Consistency, "consistency", { |x| x.to_string() });
impl_query_param!(Fields, "fields", { |x| x.0.to_string() });
impl_query_param!(Preference, "preference", { |x| x.0.to_string() });
impl_query_param!(Realtime, "realtime", { |x| x.0.to_string() });
impl_query_param!(Source, "_source", { |x| x.0.to_string() });
impl_query_param!(SourceExclude, "_source_exclude", { |x| x.0.to_string() });
impl_query_param!(SourceInclude, "_source_include", { |x| x.0.to_string() });
impl_query_param!(OpType, "op_type", { |x| x.to_string() });
impl_query_param!(Parent, "parent", { |x| x.0.to_string() });
impl_query_param!(Refresh, "refresh", { |x| x.0.to_string() });
impl_query_param!(Routing, "routing", { |x| x.0.to_string() });
impl_query_param!(Timestamp, "timestamp", { |x| x.0.to_string() });
impl_query_param!(Timeout, "timeout", { |x| x.to_string() });
impl_query_param!(MasterTimeout, "master_timeout", { |x| x.0.to_string() });
impl_query_param!(Ttl, "ttl", { |x| x.0.num_milliseconds().to_string() });
impl_query_param!(Version, "version", { |x| x.0.to_string() });
impl_query_param!(VersionType, "version_type", { |x| x.to_string() });
impl_query_param!(IgnoreUnavailable, "ignore_unavailable", { |x| x.0.to_string() });
impl_query_param!(AllowNoIndices, "allow_no_indices", { |x| x.0.to_string() });
impl_query_param!(ExpandWildcards, "expand_wildcards", { |x| x.to_string() });
impl_query_param!(MinScore, "min_score", { |x| x.0.to_string() });
impl_query_param!(Local, "local", { |x| x.0.to_string() });

#[derive(Debug, Clone, PartialEq)]
pub enum ExpandWildcards {
    Open,
    Closed,
    None,
    All
}

impl ToString for ExpandWildcards {
    fn to_string(&self) -> String {
        match *self {
            ExpandWildcards::Open => "open".to_string(),
            ExpandWildcards::Closed => "closed".to_string(),
            ExpandWildcards::None => "none".to_string(),
            ExpandWildcards::All => "all".to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Consistency {
    One,
    Quorum,
    All
}

impl ToString for Consistency {
    fn to_string(&self) -> String {
        match *self {
            Consistency::One => "one".to_string(),
            Consistency::Quorum => "quorum".to_string(),
            Consistency::All => "all".to_string()
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
            OpType::Index => "index".to_string(),
            OpType::Create => "create".to_string()
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
            VersionType::Internal => "internal".to_string(),
            VersionType::External => "external".to_string(),
            VersionType::ExternalGte => "external_gte".to_string(),
            VersionType::Force => "force".to_string()
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
            return Err(ParseTimeoutError::Invalid);
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
            ParseTimeoutError::Invalid => "invalid format",
        }
    }
}

impl fmt::Display for ParseTimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseTimeoutError::Invalid => write!(f, stringify!(self.description())),
        }
    }
}
