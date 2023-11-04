use std::error::Error;
use std::io;
use async_std::fs;

const HTTP: &'static str = "HTTP/1.1";

pub enum HttpStatus {
    OK,
    NotFound,
}

impl HttpStatus {
    fn as_string(&self) -> String {
        match self {
            Self::OK => format!("{HTTP} 200 OK"),
            Self::NotFound => format!("{HTTP} 404 NOT FOUND"),
        }
    }
}

pub enum HttpMethod {
    Get,
    Post,
}

impl HttpMethod {
    fn from_str(method: &str) -> Result<HttpMethod, Box<dyn Error>> {
         match method {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            _ => Err(String::from("Invalid method").into())
         }
    }
}


struct Uri(Vec<String>);

impl Uri {
    fn from_str(raw_uri: &str) -> Uri {
        // raw_uri[1..] because raw_uri[0] is an empty &str
        let vectorized_uri: Vec<_> = raw_uri[1..]
            .split("/")
            .map(|s| s.to_owned())
            .collect();

        Uri(vectorized_uri)
    }
}


pub struct HttpRequest {
    method: Option<HttpMethod>,
    uri: Option<Uri>,
}

impl HttpRequest {
    pub fn new(request_line: &str) -> HttpRequest {
        let mut request_line = request_line.split(' ');
        let method = request_line.next().unwrap();
        let uri = request_line.next().unwrap();

        HttpRequest {
            method: Some(HttpMethod::from_str(method).unwrap()),
            uri: Some(Uri::from_str(uri)),
        }
    }

    pub fn method(&mut self) -> HttpMethod {
        self.method.take().unwrap()
    }

    pub fn uri(&mut self) -> Vec<String> {
        self.uri.take().unwrap().0
    }
}


pub struct HttpResponse {
    status: HttpStatus,
    content_length: usize,
    content: String,
}

impl HttpResponse {
    pub async fn new(status: HttpStatus, content: &str) -> io::Result<HttpResponse> {
        let content = fs::read_to_string(content).await?;
        let content_length = content.len();
        let http_response = HttpResponse {
            status,
            content_length,
            content,
        };

        Ok(http_response)
    }

    pub fn compose(self) -> String {
        let status_line = self.status.as_string();
        let content_length = self.content_length;
        let content = self.content;

        format!("{HTTP} {status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}")
    }
}

