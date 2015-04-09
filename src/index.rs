use types::*;
use rustc_serialize::json;
use hyper::HttpResult;
use connection::Connection;

#[derive(Debug, Clone, PartialEq)]
pub struct IndexRequest<'a> {
    connection: &'a Connection,
    // path fields
    index: String,
    typ: String,
    id: Option<String>,
    // query params
    consistency: Option<Consistency>,
    op_type: Option<OpType>,
    parent: Option<Parent>,
    refresh: Option<Refresh>,
    routing: Option<Routing>,
    timeout: Option<Timeout>,
    timestamp: Option<Timestamp>,
    ttl: Option<Ttl>,
    version: Option<Version>,
    version_type: Option<VersionType>,
    // body
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
        (parent, Parent),
        (refresh, Refresh),
        (routing, Routing),
        (timeout, Timeout),
        (timestamp, Timestamp),
        (ttl, Ttl),
        (version, Version),
        (version_type, VersionType)
    }

    pub fn execute(&self) -> HttpResult<String> {
        let mut path = vec![self.index.to_string(), self.typ.to_string()];

        if let Some(ref id) = self.id {
            path.push(id.to_string());
        }

        let params: Vec<(&str, String)> = param_pairs! {
            self.consistency,
            self.op_type,
            self.parent,
            self.refresh,
            self.routing,
            self.timeout,
            self.timestamp,
            self.ttl,
            self.version,
            self.version_type
        };

        let body: String = json::encode(&self.source).ok().expect(":(");
        self.connection.post(path, params, body.as_bytes())
    }
}
