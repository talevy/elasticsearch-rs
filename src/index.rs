use chrono::{Duration, DateTime, UTC};
use types::{Consistency, OpType, VersionType, Timeout};
use rustc_serialize::json;
use hyper::HttpResult;
use client::Client;
use connection::Connection;
use std::convert::AsRef;
use self::IndexParam::*;

#[derive(Debug, PartialEq)]
pub enum IndexParam {
    ConsistencyParam,
    OpTypeParam,
    ParentParam,
    RefreshParam,
    RoutingParam,
    TimeoutParam,
    TimestampParam,
    TtlParam,
    VersionParam,
    VersionTypeParam
}

impl<'a> Into<&'a str> for IndexParam {
    fn into(self) -> &'a str {
        match self {
            ConsistencyParam => "consistency",
            OpTypeParam => "op_type",
            ParentParam => "parent",
            RefreshParam => "refresh",
            RoutingParam => "routing",
            TimeoutParam => "timeout",
            TimestampParam => "timestamp",
            TtlParam => "ttl",
            VersionParam => "version",
            VersionTypeParam => "version_type"
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexRequest<'a> {
    connection: &'a Connection,
    index: String,
    typ: String,
    id: Option<String>,

    consistency: Option<Consistency>,
    op_type: Option<OpType>,
    parent: Option<String>,
    refresh: Option<bool>,
    routing: Option<String>,
    timeout: Option<Timeout>,
    timestamp: Option<DateTime<UTC>>,
    ttl: Option<Duration>,
    version: Option<i64>,
    version_type: Option<VersionType>,

    source: json::Object
}


impl<'a> IndexRequest<'a> {
    pub fn new(connection: &'a Connection, index: &str, typ: &str, source: json::Object) -> IndexRequest<'a> {
        IndexRequest {
            connection: connection,
            index: index.to_string(),
            typ: typ.to_string(),
            id: None,
            consistency: None,
            op_type: Some(OpType::Create),
            parent: None,
            refresh: None,
            routing: None,
            timeout: None,
            timestamp: None,
            ttl: None,
            version: None,
            version_type: None,
            source: source
        }
    }

    field_setters!{ IndexRequest,
        (id, String),
        (consistency, Consistency),
        (op_type, OpType),
        (parent, String),
        (refresh, bool),
        (routing, String),
        (timeout, Timeout),
        (timestamp, DateTime<UTC>),
        (ttl, Duration),
        (version, i64),
        (version_type, VersionType)
    }

    pub fn execute(&self) -> HttpResult<String> {
        let mut path = urlify!(self.index, self.typ);
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(ref id) = self.id {
            path.push(id.to_string());
        }

        param_push!(params, ConsistencyParam => self.consistency, { |x| x.to_string() });
        param_push!(params, OpTypeParam => self.op_type, { |x| x.to_string() });
        param_push!(params, ParentParam => self.parent, { |x| x.to_string() });
        param_push!(params, RefreshParam => self.refresh, { |x| x.to_string() });
        param_push!(params, RoutingParam => self.routing, { |x| x.to_string() });
        param_push!(params, RoutingParam => self.routing, { |x| x.to_string() });
        param_push!(params, RoutingParam => self.routing, { |x| x.to_string() });
        param_push!(params, TimeoutParam => self.timeout, { |x| x.to_string() });
        param_push!(params, TimeoutParam => self.timeout, { |x| x.to_string() });
        param_push!(params, TimestampParam => self.timestamp, { |x| x.to_string() });
        param_push!(params, TtlParam => self.ttl, { |x| x.num_milliseconds().to_string() });
        param_push!(params, VersionParam => self.version, { |x| x.to_string() });
        param_push!(params, VersionTypeParam => self.version_type, { |x| x.to_string() });

        let body: String = json::encode(&self.source).ok().expect(":(");

        self.connection.post(path, params, body.as_bytes())
    }
}
