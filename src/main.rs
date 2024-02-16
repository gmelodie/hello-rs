use hello::ThreadPool;
use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    str, thread,
    time::Duration,
};

fn handle_request(mut stream: TcpStream) {
    let mut buf = vec![0; 1024];

    // read data
    stream.read(&mut buf).unwrap();
    let uri = str::from_utf8(&buf[..])
        .expect("Unable to parse HTTP request")
        .split("\n")
        .next()
        .unwrap()
        .trim();

    println!("{uri}");

    let filename: &str;
    let status: &str;
    match uri {
        "GET / HTTP/1.1" => {
            filename = "hello.html";
            status = "HTTP/1.1 200 OK";
        }
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            filename = "hello.html";
            status = "HTTP/1.1 200 OK";
        }
        _ => {
            filename = "404.html";
            status = "HTTP/1.1 404 NOT FOUND";
        }
    }
    let contents = fs::read_to_string(filename).expect("Unable to read file {filename}");
    let headers = format!("Content-Length: {}", contents.len());
    // write hello world
    stream
        .write(format!("{status}\r\n{headers}\r\n\r\n{contents}").as_bytes())
        .expect("Unable to send stream");
}

fn main() -> std::io::Result<()> {
    // accept connection
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Listening on 127.0.0.1:8080");

    let mut pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        pool.execute(|| handle_request(stream.unwrap()));
    }
    Ok(())
}
