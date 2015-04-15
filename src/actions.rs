use types::*;
use rustc_serialize::json;
use hyper::HttpResult;
use hyper::method::Method::{Get, Put, Post, Head, Delete};
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
        (source: Source, None),
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
        (source: Source, None)
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
