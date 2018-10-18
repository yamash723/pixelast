use reqwest;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashMap;

pub enum MethodType {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Hash, PartialEq, Eq)]
pub enum HeaderType {
    UserToken,
    ContentLength,
}

pub struct RequestContext<'a> {
    pub uri: &'a str,
    pub method: MethodType,
    pub body: Option<&'a str>,
    pub headers: HashMap<HeaderType, &'a str>,
}

impl<'a> RequestContext<'a> {
    pub fn new(uri: &'a str, method: MethodType, body: Option<&'a str>, token: Option<&'a str>) -> Self {
        let mut headers = HashMap::new();
        if let Some(v) = token {
            headers.insert(HeaderType::UserToken, v);
        }

        Self {
            uri,
            method,
            body,
            headers,
        }
    }

    pub fn insert_header(&mut self, header_type: HeaderType, value : &'a str) {
        self.headers.insert(header_type, value);
    }
}

pub trait HttpClient {
    fn do_request(context: &RequestContext) -> Result<String, reqwest::Error>;
}

pub struct TinyHttpClient;
impl HttpClient for TinyHttpClient {
    fn do_request(context: &RequestContext) -> Result<String, reqwest::Error> {
        let client = Client::new();

        let mut req = match context.method {
            MethodType::Get => client.get(context.uri),
            MethodType::Post => client.post(context.uri),
            MethodType::Put => client.put(context.uri),
            MethodType::Delete => client.delete(context.uri),
        };

        if let Some(v) = context.body {
            req = req.body(v.to_owned());
        };

        let mut headers = HeaderMap::new();
        for (key, val) in context.headers.iter() {
            let type_name = match key {
                HeaderType::UserToken => "X-USER-TOKEN",
                HeaderType::ContentLength => "CONTENT-LENGTH",
            };

            headers.insert(type_name, HeaderValue::from_str(val).unwrap());
        }

        let res = req.headers(headers)
                     .send()?
                     .text()?;
        Ok(res)
    }
}