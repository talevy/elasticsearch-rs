use types::*;
use rustc_serialize::json;
use hyper::HttpResult;
use hyper::method::Method::{Get, Post, Head, Delete};
use connection::Connection;

new_query_struct!{ IndexRequest(index: String, typ: String, id: Option<String>,
                                source: json::Object) {
    fn_path => |self| {
        let &IndexRequest { ref index, ref typ, ref id, .. } = self;
        let mut path: Vec<String> = vec![index.to_string(), typ.to_string()];
        if let &Some(ref inner_id) = id {
            path.push(inner_id.to_string());
        }
        path
    },
    query_params => [
        (consistency: Consistency, None),
        (op_type: OpType, Some(OpType::Create)),
        (parent: Parent, None),
        (refresh: Refresh, None),
        (routing: Routing, None),
        (timeout: Timeout, None),
        (timestamp: Timestamp, None),
        (ttl: Ttl, None),
        (version: Version, None),
        (version_type: VersionType, None)
    ],
    body => source,
    method => Post
}}

new_query_struct!{ UpdateRequest(index: String, typ: String, id: String,
                                update_body: json::Object) {
    fn_path => |self| {
        vec![self.index.to_string(), self.typ.to_string(), self.id.to_string()]
    },
    query_params => [
        (consistency: Consistency, None),
        (fields: Fields, None),
        (lang: Lang, None),
        (parent: Parent, None),
        (refresh: Refresh, None),
        (retry_on_conflict: RetryOnConflict, None),
        (routing: Routing, None),
        (script: Script, None),
        (script_id: ScriptId, None),
        (scripted_upsert: ScriptedUpsert, None),
        (timeout: Timeout, None),
        (timestamp: Timestamp, None),
        (ttl: Ttl, None),
        (version: Version, None),
        (version_type: VersionType, None)
    ],
    body => update_body,
    method => Post
}}

new_query_struct!{ GetRequest(index: String, typ: String, id: String) {
    fn_path => |self| {
        vec![self.index.to_string(), self.typ.to_string(), self.id.to_string()]
    },
    query_params => [
        (fields: Fields, None),
        (parent: Parent, None),
        (preference: Preference, None),
        (realtime: Realtime, None),
        (refresh: Refresh, None),
        (routing: Routing, None),
        (source: _Source, None),
        (source_exclude: SourceExclude, None),
        (source_include: SourceInclude, None),
        (version: Version, None),
        (version_type: VersionType, None)
    ],
    method => Get
}}

new_query_struct!{ CountRequest(index: Option<String>, typ: Option<String>) {
    fn_path => |self| {
        let mut path: Vec<String> = Vec::new();
        if let Some(ref index) = self.index { path.push(index.to_string()); }
        if let Some(ref typ) = self.typ { path.push(typ.to_string()); }
        path.push("_count".to_string());
        path
    },
    query_params => [
        (ignore_unavailable: IgnoreUnavailable, None),
        (allow_no_indices: AllowNoIndices, None),
        (expand_wildcards: ExpandWildcards, None),
        (min_score: MinScore, None),
        (preference: Preference, None),
        (routing: Routing, None),
        (source: _Source, None)
    ],
    method => Get
}}

new_query_struct!{ ExistsRequest(index: String, typ: String, id: String) {
    fn_path => |self| {
        vec![self.index.to_string(), self.typ.to_string(), self.id.to_string()]
    },
    query_params => [
        (parent: Parent, None),
        (preference: Preference, None),
        (realtime: Realtime, None),
        (refresh: Refresh, None),
        (routing: Routing, None)
    ],
    method => Head
}}

new_query_struct!{ DeleteRequest(index: String, typ: String, id: String) {
    fn_path => |self| {
        vec![self.index.to_string(), self.typ.to_string(), self.id.to_string()]
    },
    query_params => [
        (consistency: Consistency, None),
        (parent: Parent, None),
        (refresh: Refresh, None),
        (routing: Routing, None),
        (timeout: Timeout, None),
        (version: Version, None),
        (version_type: VersionType, None)
    ],
    method => Delete
}}


#[derive(Debug, Clone, PartialEq)]
pub struct BulkRequest<'a> {
    connection: &'a Connection,
    payload: BulkPayload,
    consistency: Option<Consistency>,
    index: Option<Index>,
    typ: Option<Type>,
    refresh: Option<Refresh>,
    routing: Option<Routing>,
    timeout: Option<Timeout>,
    version: Option<Version>,
    version_type: Option<VersionType>
}

impl<'a> BulkRequest<'a> {
    pub fn new(connection: &'a Connection, payload: BulkPayload) -> BulkRequest<'a> {
        BulkRequest {
            connection: connection,
            payload: payload,
            consistency: None,
            index: None,
            typ: None,
            refresh: None,
            routing: None,
            timeout: None,
            version: None,
            version_type: None
        }
    }

    pub fn get(self) -> BulkRequest<'a> { self }

    field_setter!{ BulkRequest , (consistency, Consistency) }
    field_setter!{ BulkRequest , (index, Index) }
    field_setter!{ BulkRequest , (typ, Type) }
    field_setter!{ BulkRequest , (refresh, Refresh) }
    field_setter!{ BulkRequest , (routing, Routing) }
    field_setter!{ BulkRequest , (timeout, Timeout) }
    field_setter!{ BulkRequest , (version, Version) }
    field_setter!{ BulkRequest , (version_type, VersionType) }

    pub fn get_path(&self) -> Vec<String>  { vec!["_bulk".to_string()] }

    pub fn execute(&self) -> HttpResult<String> {
        let params: Vec<(&str, String)> = param_pairs! {
            self.consistency,
            self.index,
            self.typ,
            self.refresh,
            self.routing,
            self.timeout,
            self.version,
            self.version_type
        };
        let bod: String = self.payload.to_string();
        self.connection.request(Post, self.get_path(), params, Some(bod.as_bytes()))
    }
}
