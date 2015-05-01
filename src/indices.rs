use types::*;
use rustc_serialize::json;
use hyper::HttpResult;
use hyper::method::Method::{Post, Head};
use connection::Connection;

// new_query_struct!{ AnalyzeRequest(index: Option<String>) {
//     fn_path => |self| {
//         if let Some(ref i) = self.index {
//             vec![i.to_string()]
//         } else {
//             vec![]
//         }
//     },
//     query_params => [
//         (source: Source, None),
//         (analyzer: Analyzer, None)
//     ],
//     method => Get
// }}

new_query_struct!{ ExistsRequest(indices: StringList) {
    fn_path => |self| {
        vec![self.indices.to_string()]
    },
    query_params => [
        (ignore_unavailable: IgnoreUnavailable, None),
        (allow_no_indices: AllowNoIndices, None),
        (expand_wildcards: ExpandWildcards, None),
        (local: Local, None)
    ],
    method => Head
}}

new_query_struct!{ CreateRequest(index: String) {
    fn_path => |self| {
        vec![self.index.to_string()]
    },
    query_params => [
        (timeout: Timeout, None),
        (master_timeout: MasterTimeout, None),
        (ignore_unavailable: IgnoreUnavailable, None),
        (allow_no_indices: AllowNoIndices, None),
        (expand_wildcards: ExpandWildcards, None)
    ],
    method => Post
}}

new_query_struct!{ CloseRequest(index: String, config_body: json::Object) {
    fn_path => |self| {
        vec![self.index.to_string()]
    },
    query_params => [
        (timeout: Timeout, None),
        (master_timeout: MasterTimeout, None)
    ],
    body => config_body,
    method => Post
}}
