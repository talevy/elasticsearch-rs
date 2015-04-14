use types::*;
use rustc_serialize::json;
use hyper::HttpResult;
use hyper::method::Method::Post;
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
