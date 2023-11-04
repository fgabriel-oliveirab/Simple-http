use async_std::{
    prelude::*,
    net::TcpStream,
    io::BufReader,
    io,
};

mod http;

pub async fn handle_connection(mut stream: TcpStream) -> io::Result<()> { 
    let buf = BufReader::new(&mut stream); 
    let request_line = buf.lines().next().await.unwrap()?;
    let mut request = http::HttpRequest::new(&request_line[..]);
    let (res_status, res_content);

    match request.method() { 
        http::HttpMethod::Get=> {
            match &request.uri()[0][..] {
                "" => (res_status, res_content) = (http::HttpStatus::OK, "index.html"),
                "foss" => (res_status, res_content) = (http::HttpStatus::OK, "foss.html"),
                _ => (res_status, res_content) = (http::HttpStatus::NotFound, "404.html"),
            }
        },

        http::HttpMethod::Post => (res_status, res_content) = (http::HttpStatus::NotFound, "404.html"),
    }

    let response = http::HttpResponse::new(res_status, res_content).await?;

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

