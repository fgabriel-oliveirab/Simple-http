use async_std::{
    prelude::*,
    net::TcpListener,
    io,
    task,
};
use simple_http::handle_connection;

#[async_std::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    while let Some(stream) = listener.incoming().next().await {
        let stream = stream?;
        task::spawn(handle_connection(stream));
    }
    Ok(())
}
