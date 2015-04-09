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

    pub fn id(&'a mut self, id: String) -> &'a mut IndexRequest {
        self.id = Some(id);
        self
    }

    pub fn consistency(&'a mut self, consistency: Consistency) -> &'a mut IndexRequest {
        self.consistency = Some(consistency);
        self
    }

    pub fn op_type(&'a mut self, op_type: OpType) -> &'a mut IndexRequest {
        self.op_type = Some(op_type);
        self
    }

    pub fn parent(&'a mut self, parent: String) -> &'a mut IndexRequest {
        self.parent = Some(parent);
        self
    }

    pub fn refresh(&'a mut self, refresh: bool) -> &'a mut IndexRequest {
        self.refresh = Some(refresh);
        self
    }

    pub fn routing(&'a mut self, routing: String) -> &'a mut IndexRequest {
        self.routing = Some(routing);
        self
    }

    pub fn timeout(&'a mut self, timeout: Timeout) -> &'a mut IndexRequest {
        self.timeout = Some(timeout);
        self
    }

    pub fn timestamp(&'a mut self, timestamp: DateTime<UTC>) -> &'a mut IndexRequest {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn ttl(&'a mut self, ttl: Duration) -> &'a mut IndexRequest {
        self.ttl = Some(ttl);
        self
    }

    pub fn version(&'a mut self, version: i64) -> &'a mut IndexRequest {
        self.version = Some(version);
        self
    }

    pub fn version_type(&'a mut self, version_type: VersionType) -> &'a mut IndexRequest {
        self.version_type = Some(version_type);
        self
    }

    pub fn execute(&self) -> HttpResult<String> {
        let path = urlify!(self.index, self.typ);
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(ref consistency) = self.consistency {
            params.push((ConsistencyParam.into(), consistency.to_string()));
        }

        if let Some(ref op_type) = self.op_type {
            params.push((OpTypeParam.into(), op_type.to_string()));
        }

        if let Some(ref parent) = self.parent {
            params.push((ParentParam.into(), parent.to_string()));
        }

        if let Some(ref refresh) = self.refresh {
            params.push((RefreshParam.into(), refresh.to_string()));
        }

        if let Some(ref routing) = self.routing {
            params.push((RoutingParam.into(), routing.to_string()));
        }

        if let Some(ref timeout) = self.timeout {
            params.push((TimeoutParam.into(), timeout.to_string()));
        }

        if let Some(ref timestamp) = self.timestamp {
            params.push((TimestampParam.into(), timestamp.to_string()));
        }

        if let Some(ref ttl) = self.ttl {
            params.push((TtlParam.into(), ttl.num_milliseconds().to_string()));
        }

        if let Some(ref version) = self.version {
            params.push((VersionParam.into(), version.to_string()));
        }

        if let Some(ref version_type) = self.version_type {
            params.push((VersionTypeParam.into(), version_type.to_string()));
        }

        let body: String = json::encode(&self.source).ok().expect(":(");

        self.connection.post(path, params, body.as_bytes())
    }
}
