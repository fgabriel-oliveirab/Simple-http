use async_std::{
    prelude::*,
    net::TcpStream,
    io::BufReader,
    fs,
    io,
};

pub async fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let buf = BufReader::new(&mut stream);

    let request_line = buf.lines().next().await.unwrap()?;
    let (response_status, response_content);

    match &request_line[..] {
        "GET / HTTP/1.1" => (response_status, response_content) = ("HTTP/1.1 200 OK", "index.html"),
        "GET /foss HTTP/1.1" => (response_status, response_content) = ("HTTP/1.1 200 OK", "foss.html"),
        _ => (response_status, response_content) = ("HTTP/1.1 404 NOT FOUND", "404.html"),
    }

    let response_content = fs::read_to_string(response_content).await?;
    let response_content_length = response_content.len();
    let response = format!("{response_status}\r\nContent-Length: {response_content_length}\r\n\r\n{response_content}");
    
    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}
