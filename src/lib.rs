use async_std::{
    prelude::*,
    net::TcpStream,
    io::BufReader,
    fs,
    io,
};


const HTTP: &'static str = "HTTP/1.1";


enum HttpStatus {
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


enum HttpMethod {
    Get,
    Post,
}

impl HttpMethod {
    fn from_str(method: &str) -> Option<HttpMethod> {
        match method {
            "GET" => Some(Self::Get),
            "POST" => Some(Self::Post),
            _ => None,
        }
    }
}


struct Uri(Vec<String>);

impl Uri {
    fn from_str(raw_uri: &str) -> Uri {
        // [1..] because the [0] is a empty &str
        let vectorized_uri: Vec<_> = raw_uri[1..]
            .split("/")
            .map(|s| s.to_owned())
            .collect();

        Uri(vectorized_uri)
    }
}
struct HttpRequest {
    method: HttpMethod,
    uri: Uri, 
}
impl HttpRequest {
    fn new(request_line: &str) -> HttpRequest {
        let mut request_line = request_line.split(' ');
        let method = request_line.next().unwrap();
        let uri = request_line.next().unwrap();

        HttpRequest { 
            method: HttpMethod::from_str(method).unwrap(), 
            uri: Uri::from_str(uri), 
        }
    }
}


struct HttpResponse {
    status: HttpStatus,
    content_length: usize,
    content: String,
}
impl HttpResponse {
    async fn new(status: HttpStatus, content: &str) -> io::Result<HttpResponse> {
        let content = fs::read_to_string(content).await?;
        let content_length = content.len();
        let http_response = HttpResponse { 
            status,
            content_length,
            content: content, 
        };

        Ok(http_response)
    }

    fn compose(self) -> String {
        let status_line = self.status.as_string();
        let content_length = self.content_length;
        let content = self.content;


        format!("{HTTP} {status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}")
    }
}


pub async fn handle_connection(mut stream: TcpStream) -> io::Result<()> { 
    let buf = BufReader::new(&mut stream); 
    let request_line = buf.lines().next().await.unwrap()?;
    let request = HttpRequest::new(&request_line[..]);
    let (res_status, res_content);

    match request.method { 
        HttpMethod::Get=> {
            match &request.uri.0[0][..] {
                "" => (res_status, res_content) = (HttpStatus::OK, "index.html"),
                "foss" => (res_status, res_content) = (HttpStatus::OK, "foss.html"),
                _ => (res_status, res_content) = (HttpStatus::NotFound, "404.html"),
            }
        },

        HttpMethod::Post => (res_status, res_content) = (HttpStatus::NotFound, "404.html"),
    }

    let response = HttpResponse::new(res_status, res_content).await?;

    stream.write_all(response.compose().as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn path_composing_well() {
        let uri = Uri::from_str("/hello/world");
        assert_eq!(uri.0, vec!["hello".to_owned(), "world".to_owned()]);

        let uri = Uri::from_str("/");
        assert_eq!(uri.0, vec!["".to_owned()]);
    }
}

