use types::*;
use rustc_serialize::json;
use hyper::HttpResult;
use hyper::method::Method::{Get, Put, Post, Head, Delete};
use connection::Connection;

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
