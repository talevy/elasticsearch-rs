use rustc_serialize::json;
use url::Url;
use connection::Connection;
use index::IndexRequest;

//
// TODO(talevy): MOAR connections! load-balancing, fault-tolerance, etc.
//

#[derive(Debug, Clone, PartialEq)]
pub struct Client {
    pub connection: Connection
}

impl Client {

    pub fn new_with_url_host(host: Url) -> Client {
        Client {
            connection: Connection::new(host)
        }
    }

    pub fn new_with_str_host(host: &str) -> Client {
        Client {
            connection: Connection::new(Url::parse(host).unwrap())
        }
    }

    pub fn index(&self, index: &str, typ: &str, source: json::Object) -> IndexRequest {
        IndexRequest::new(&self.connection, index, typ, source)
    }
}

#[test]
fn index() {
    use collections::BTreeMap;
    use rustc_serialize::json::Json;
    use types::OpType;

    let client = Client::new_with_str_host("http://localhost:9200");

    let mut source: json::Object = BTreeMap::new();
    source.insert("first_field".to_string(), Json::U64(2u64));

    println!("{:?}", client.index("hello", "world", source).id("hello".to_string()).op_type(OpType::Create).execute());
}
