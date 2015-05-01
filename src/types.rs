use std::error::Error;
use std::str::FromStr;
use std::fmt;
use std::string::ToString;
use chrono::{Duration, DateTime, UTC};
use rustc_serialize::json;

pub trait QueryParam {
    fn get_name(&self) -> &'static str;
    fn get_value(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct StringList(pub Vec<String>);

impl fmt::Display for StringList {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut has_entries = false;

        if self.0.is_empty() { return write!(fmt, ""); }

        for e in self.0.iter().take(self.0.len() - 1) {
            let prefix = if has_entries { "," } else { "" };
            let _ = write!(fmt, "{}{}", prefix, e);
            has_entries = true;
        };
        let prefix = if has_entries { "," } else { "" };
        return write!(fmt, "{}{}", prefix, self.0.iter().last().unwrap());
    }
}

impl_as_ref!{ pub struct Parent(String) }

impl From<&'static str> for Parent {
    fn from(a: &'static str) -> Self {
        Parent(a.to_string())
    }
}

impl_as_ref!{ pub struct AllowNoIndices(bool) }
impl_as_ref!{ pub struct Analyzer(String) }
impl_as_ref!{ pub struct Fields(StringList) }
impl_as_ref!{ pub struct IgnoreUnavailable(bool) }
impl_as_ref!{ pub struct Index(String) }
impl_as_ref!{ pub struct Lang(String) }
impl_as_ref!{ pub struct Local(bool) }
impl_as_ref!{ pub struct MasterTimeout(Timeout) }
impl_as_ref!{ pub struct MinScore(f64) }
impl_as_ref!{ pub struct Preference(String) }
impl_as_ref!{ pub struct Realtime(bool) }
impl_as_ref!{ pub struct Refresh(bool) }
impl_as_ref!{ pub struct RetryOnConflict(usize) }
impl_as_ref!{ pub struct Routing(String) }
impl_as_ref!{ pub struct Script(String) }
impl_as_ref!{ pub struct ScriptId(String) }
impl_as_ref!{ pub struct ScriptedUpsert(bool) }
impl_as_ref!{ pub struct Source(String) }
impl_as_ref!{ pub struct SourceExclude(StringList) }
impl_as_ref!{ pub struct SourceInclude(StringList) }
impl_as_ref!{ pub struct Timestamp(DateTime<UTC>) }
impl_as_ref!{ pub struct Ttl(Duration) }
impl_as_ref!{ pub struct Type(String) }
impl_as_ref!{ pub struct Version(i64) }
impl_as_ref!{ pub struct _Source(bool) }

impl_query_param!(AllowNoIndices, "allow_no_indices", { |x| x.0.to_string() });
impl_query_param!(Analyzer, "analyzer", { |x| x.0.to_string() });
impl_query_param!(Consistency, "consistency", { |x| x.to_string() });
impl_query_param!(ExpandWildcards, "expand_wildcards", { |x| x.to_string() });
impl_query_param!(Fields, "fields", { |x| x.0.to_string() });
impl_query_param!(IgnoreUnavailable, "ignore_unavailable", { |x| x.0.to_string() });
impl_query_param!(Index, "index", { |x| x.0.to_string() });
impl_query_param!(Lang, "lang", { |x| x.0.to_string() });
impl_query_param!(Local, "local", { |x| x.0.to_string() });
impl_query_param!(MasterTimeout, "master_timeout", { |x| x.0.to_string() });
impl_query_param!(MinScore, "min_score", { |x| x.0.to_string() });
impl_query_param!(OpType, "op_type", { |x| x.to_string() });
impl_query_param!(Parent, "parent", { |x| x.0.to_string() });
impl_query_param!(Preference, "preference", { |x| x.0.to_string() });
impl_query_param!(Realtime, "realtime", { |x| x.0.to_string() });
impl_query_param!(Refresh, "refresh", { |x| x.0.to_string() });
impl_query_param!(RetryOnConflict, "retry_on_conflict", { |x| x.0.to_string() });
impl_query_param!(Routing, "routing", { |x| x.0.to_string() });
impl_query_param!(Script, "script", { |x| x.0.to_string() });
impl_query_param!(ScriptId, "script_id", { |x| x.0.to_string() });
impl_query_param!(ScriptedUpsert, "scripted_upsert", { |x| x.0.to_string() });
impl_query_param!(Source, "source", { |x| x.0.to_string() });
impl_query_param!(SourceExclude, "_source_exclude", { |x| x.0.to_string() });
impl_query_param!(SourceInclude, "_source_include", { |x| x.0.to_string() });
impl_query_param!(Timeout, "timeout", { |x| x.to_string() });
impl_query_param!(Timestamp, "timestamp", { |x| x.0.to_string() });
impl_query_param!(Ttl, "ttl", { |x| x.0.num_milliseconds().to_string() });
impl_query_param!(Type, "type", { |x| x.0.to_string() });
impl_query_param!(Version, "version", { |x| x.0.to_string() });
impl_query_param!(VersionType, "version_type", { |x| x.to_string() });
impl_query_param!(_Source, "_source", { |x| x.0.to_string() });

#[derive(Debug, Clone, PartialEq)]
#[derive(RustcDecodable, RustcEncodable)]
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
#[derive(RustcDecodable, RustcEncodable)]
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
#[derive(RustcDecodable, RustcEncodable)]
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
#[derive(RustcDecodable, RustcEncodable)]
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

// https://github.com/elastic/elasticsearch/blob/master/src/main/java/org/elasticsearch/action/bulk/BulkRequest.java#L248
#[derive(Debug, Clone, PartialEq)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BulkActionMetadata {
    _index: String,
    _type: String,
    _id: Option<String>,
    _routing: Option<String>,
    _parent: Option<String>,
    _timestamp: Option<String>,
    _ttl: Option<String>,
    op_type: Option<OpType>,
    _version: Option<i64>,
    _version_type: Option<VersionType>,
    _retry_on_conflict: Option<bool>
}


#[derive(Debug, Clone, PartialEq)]
#[derive(RustcDecodable, RustcEncodable)]
pub enum BulkAction {
    Index(BulkActionMetadata, String),
    Delete(BulkActionMetadata, String),
    Update(BulkActionMetadata, String)
}

impl ToString for BulkAction {
    fn to_string(&self) -> String {
        match *self {
            BulkAction::Index(ref a, ref b) => format!("{}\n{}\n", json::encode(a).unwrap(), b.to_string()),
            BulkAction::Delete(ref a, ref b) => format!("{}\n{}\n", json::encode(a).unwrap(), b.to_string()),
            BulkAction::Update(ref a, ref b) => format!("{}\n{}\n", json::encode(a).unwrap(), b.to_string())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BulkPayload {
    actions: Vec<BulkAction>
}


impl ToString for BulkPayload {
    fn to_string(&self) -> String {
        let mut res = String::new();
        for i in self.actions.iter() {
            res.push_str(i.to_string().as_str());
        }
        res
    }
}

#[test]
fn test() {
    use std::default::Default;
    let a = BulkPayload {
        actions: vec![
            BulkAction::Update(BulkActionMetadata {
                _index: "_index".to_string(),
                _type: "_type".to_string(),
                _id: Some("_id".to_string()),
                _routing: Some("_routing".to_string()),
                _parent: Default::default(),
                _timestamp: Default::default(),
                _ttl: Default::default(),
                op_type: Default::default(),
                _version: Default::default(),
                _version_type: Default::default(),
                _retry_on_conflict: Default::default()
            }, "{...}".to_string())
        ]
    };
    println!("{}", a.to_string());
}
