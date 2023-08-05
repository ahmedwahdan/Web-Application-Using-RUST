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

// HttpRequest build up
impl From<String> for HttpRequest {
    fn from(request: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_headers = HashMap::new();
        let mut parsed_body = "".to_string();

        for line in request.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_request_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, val) = process_header_line(line);
                parsed_headers.insert(key, val);
            } else if line.len() == 0 {
                // Do Nothing
            } else {
                parsed_body += line;
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_body,
        }
    }
}

// Request line processing
fn process_request_line(s: &str) -> (Method, Resource, Version) {
    // split requet line into chunks separated by whitespaces
    let mut words = s.split_whitespace();
    // extract method
    let method = words.next().unwrap();
    // extract path/URI/URL
    let path = words.next().unwrap();
    // extract version
    let version = words.next().unwrap();

    // convert and return
    (
        method.into(),
        Resource::Path(path.to_string()),
        version.into(),
    )
}

// Request header processing
fn process_header_line(header: &str) -> (String, String) {
    let mut header_items = header.split(":");
    let key = header_items.next().unwrap();
    let value = header_items.next().unwrap();

    (key.to_string(), value.to_string())
}

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    Uninitialized,
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(value: &str) -> Method {
        match value {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::Uninitialized,
        }
    }
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::GET);
    }

    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let http_req_string : String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");
        let mut expected_headers = HashMap::new();

        expected_headers.insert("Host".to_string(), " localhost".to_string());
        expected_headers.insert("Accept".into(), " */*".into());
        expected_headers.insert("User-Agent".into(), " curl/7.64.1".into());

        let req: HttpRequest = http_req_string.into();

        assert_eq!(Method::GET, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(expected_headers, req.headers);
    }
}
