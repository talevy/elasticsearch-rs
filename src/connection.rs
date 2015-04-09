use std::io::Read;
use url::Url;
use url::form_urlencoded::serialize;
use hyper;
use hyper::error::HttpResult;

#[derive(Debug, Clone, PartialEq)]
pub struct Connection {
    host: Url
}


impl Connection {
    pub fn new(host: Url) -> Connection {
        Connection { host: host }
    }

    pub fn post(&self, mut path: Vec<String>, pairs: Vec<(&str, String)>, body: &[u8]) -> HttpResult<String> {
        let mut url = self.host.clone();
        println!("{:?}", url.path_mut());
        url.path_mut().unwrap().append(&mut path);

        if !pairs.is_empty() {
            url.query = Some(serialize(pairs.iter().map(|&(n, ref v)| (n, &**v))));
        }

        let mut hyper_client = hyper::Client::new();
        let mut response = try!(hyper_client.post(url)
                                .body(body)
                                .send());
        let mut s = String::new();
        try!(response.read_to_string(&mut s));
        Ok(s)
    }
}
