use std::io::Read;
use url::Url;
use url::form_urlencoded::serialize;
use hyper;
use hyper::error::{HttpError, HttpResult};
use hyper::method::Method;
use hyper::method::Method::{Head, Put, Post, Get};

#[derive(Debug, Clone, PartialEq)]
pub struct Connection {
    host: Url
}


impl Connection {
    pub fn new(host: Url) -> Connection {
        Connection { host: host }
    }

    pub fn request(&self, method: Method, mut path: Vec<String>, pairs: Vec<(&str, String)>, body: Option<&[u8]>) -> HttpResult<String> {
        let mut url = self.host.clone();
        println!("{:?}", url.path_mut());
        url.path_mut().unwrap().append(&mut path);

        if !pairs.is_empty() {
            url.query = Some(serialize(pairs.iter().map(|&(n, ref v)| (n, &**v))));
        }

        let mut hyper_client = hyper::Client::new();

        let mut response = try!(match (method, body) {
            (Post, Some(bod)) => { hyper_client.post(url).body(bod).send() },
            (Post, None) => { hyper_client.post(url).send() },
            (Put, Some(bod)) => { hyper_client.put(url).body(bod).send() },
            (Put, None) => { hyper_client.put(url).send() },
            (Get, _) => { hyper_client.get(url).send() },
            _ => Err(HttpError::HttpMethodError)
        });

        let mut s = String::new();
        try!(response.read_to_string(&mut s));
        Ok(s)
    }
}
