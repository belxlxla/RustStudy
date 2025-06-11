use crate::{Method, Version};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        fn process_req_line(s: &str) -> (Method, Resource, Version) {
            let mut words = s.split_whitespace();
            let method = words.next().unwrap();
            let resource = words.next().unwrap();
            let version = words.next().unwrap();
            (
                method.into(),
                Resource::Path(resource.to_string()),
                version.into(),
            )
        }

        fn process_header_line(s: &str) -> (String, String) {
            let mut header_items = s.split(":");
            let mut key = String::from("");
            let mut value = String::from("");
            if let Some(k) = header_items.next() {
                key = k.trim().to_string();
            }
            if let Some(v) = header_items.next() {
                value = v.trim().to_string()
            }
            (key, value)
        }

        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
                // 빈 행
            } else {
                parsed_msg_body = line;
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_read_http() {
        let s: String = String::from("GET /greeting HTTP/1.1\r\nHOST:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n8r\n");
        let mut headers_expected = HashMap::new();
        headers_expected.insert("HOST".into(), "3000".into());
        headers_expected.insert("Accept".into(), "*/*".into());
        headers_expected.insert("User-Agent".into(), "curl/7.64.1".into());
        let req: HttpRequest = s.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }

    #[test]
    fn test_method_into() {
        let m = Method::Get;
        assert_eq!(m, Method::Get);
        
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }
}