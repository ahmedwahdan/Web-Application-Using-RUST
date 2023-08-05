use core::fmt;
use std::{collections::HashMap, string};

use crate::httprequest::Version;



#[derive(Debug)]
pub enum StatusCode {
    OK = 200,
}

#[derive(Debug)]
pub enum ContentType{
    TextHtml,
    Json,
}


// structs
#[derive(Debug)]
struct HttpResponse{
    version : Version,
    status_code : StatusCode,
    content_type : ContentType,
    header_line_1 : HashMap<String,String>,
    header_line_2 : HashMap<String,String>,
    body : String,
}

// // from 
// impl From<Version> for &str {
//     fn from(version: Version) -> Self {
//         match version {
//             Version::V1_1 => "HTTP/1.1",
//             _ => panic!(),
//         }
//     }
// }


// impl From<StatusCode> for &str {
//     fn from(code: StatusCode) -> Self {
//         match code {
//             StatusCode::OK => "200 OK",
//             _ => panic!(),
//         }
//     }
// }

impl ToString for Version {
    fn to_string(&self) -> String {
        match self {
            Version::V1_1 => "HTTP/1.1".to_string(),
            _ => panic!(),
        }
    }
}

impl ToString for StatusCode {
    fn to_string(&self) -> String {
        match self {
            StatusCode::OK => "200 OK".to_string(),
            _ => panic!(),
        }
    }
}

// methods
// impl From<HttpResponse> for String {
//     fn from(response: HttpResponse) -> Self {
//      let mut message : String = "".to_string();
//      message.push_str(&response.version.to_string());
//      message.push(' '); // space
//      message.push_str(&response.status_code.to_string());
//      message.push_str(r#"\r\n"#); // new line
//      message   
//     }
// }


impl ToString for HttpResponse {
    fn to_string(&self) -> String {
     let mut message : String = "".to_string();
     message.push_str(&self.version.to_string());
     message.push(' '); // space
     message.push_str(&self.status_code.to_string());
     message.push_str("\r\n"); // new line
     // add header_line_1
     for line in &self.header_line_1{
        message.push_str(line.0);
        message.push_str(" : ");
        message.push_str(line.1);
        message.push_str("\r\n"); // new line
     }

     if self.body.len() != 0 {
        message.push_str("Content-Lenght: ");
        message.push_str(&self.body.len().to_string());
        message.push_str("\r\n\r\n"); // new line & empty line
        message.push_str(&self.body);
     }



     message   
    }
}
// tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_response_into() {


        let mut _header_line_1 = HashMap::new();
        _header_line_1.insert("Bla bla".to_string(), "yup yup".to_string());
        _header_line_1.insert("Bla blaz".to_string(), "yup yupz".to_string());

        let response: HttpResponse = HttpResponse {
            version : Version::V1_1,
            status_code: StatusCode::OK,
            content_type : ContentType::TextHtml,
            header_line_1 : _header_line_1,
            header_line_2 : HashMap::new(),
            body : "Hello World".to_string(),

        };
        
        println!("{}", response.to_string());
    }
}
