use rustc_serialize::json;
use url::Url;
use connection::Connection;
use actions::{CountRequest, GetRequest, IndexRequest, ExistsRequest, DeleteRequest};
use indices;
use types::*;

//
// TODO(talevy): MOAR connections! load-balancing, fault-tolerance, etc.
//

#[derive(Debug, Clone, PartialEq)]
pub struct Client {
    pub connection: Connection,
    pub indices: IndicesClient
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndicesClient {
    pub connection: Connection
}

impl IndicesClient {
    pub fn new(conn: Connection) -> IndicesClient {
        IndicesClient { connection: conn }
    }

    pub fn exists(&self, indices: StringList) -> indices::ExistsRequest {
        indices::ExistsRequest::new(&self.connection, indices)
    }
}


impl Client {

    pub fn new_with_url_host(host: Url) -> Client {
        let conn = Connection::new(host);
        Client {
            connection: conn.clone(),
            indices: IndicesClient::new(conn.clone())
        }
    }

    pub fn new_with_str_host(host: &str) -> Client {
        let conn =Connection::new(Url::parse(host).unwrap());
        Client {
            connection: conn.clone(),
            indices: IndicesClient::new(conn.clone())
        }
    }

    pub fn index(&self, index: &str, typ: &str, id: Option<String>, source: json::Object) -> IndexRequest {
        IndexRequest::new(&self.connection, index.to_string(), typ.to_string(), id, source)
    }

    pub fn get(&self, index: &str, typ: &str, id: &str) -> GetRequest {
        GetRequest::new(&self.connection, index.to_string(), typ.to_string(), id.to_string())
    }

    pub fn count(&self, index: Option<String>, typ: Option<String>) -> CountRequest {
        CountRequest::new(&self.connection, index, typ)
    }

    pub fn exists(&self, index: &str, typ: &str, id: &str) -> ExistsRequest {
        ExistsRequest::new(&self.connection, index.to_string(), typ.to_string(), id.to_string())
    }

    pub fn delete(&self, index: &str, typ: &str, id: &str) -> DeleteRequest {
        DeleteRequest::new(&self.connection, index.to_string(), typ.to_string(), id.to_string())
    }
}

// #[test]
// fn index() {
//     use collections::BTreeMap;
//     use rustc_serialize::json::Json;
//     use types::OpType;

//     let client = Client::new_with_str_host("http://localhost:9200");

//     let mut source: json::Object = BTreeMap::new();
//     source.insert("first_field".to_string(), Json::U64(2u64));

//     println!("{:?}", client.index("hello", "world", None, source)
//              .op_type(OpType::Create).execute());

//     let resp = client.get("hello", "world", "jofis")
//         .parent("hello")
//         .fields(string_list!["cool"]).execute();

//     println!("{:?}", resp);
// }
