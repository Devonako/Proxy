use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use hyper::{Request, Body, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_client(stream).await;
        });
    }
}

async fn handle_client(mut client_stream: TcpStream) {
    // 1. Read client request (using hyper for HTTP parsing)
    let mut buf = [0; 1024];
    let n = client_stream.read(&mut buf).await.unwrap(); 
    let request = Request::from_parts(
        // ... parse HTTP request from buf ...
    );

    // 2. Forward request to destination server
    let mut server_stream = TcpStream::connect(request.uri().authority().unwrap().as_str()).await.unwrap(); 
    // ... send the request to the server ...

    // 3. Receive server response
    let mut server_response = Vec::new();
    server_stream.read_to_end(&mut server_response).await.unwrap();

    // 4. Relay response to client
    client_stream.write_all(&server_response).await.unwrap(); 
}