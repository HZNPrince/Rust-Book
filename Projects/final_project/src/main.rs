use final_project::ThreadPool;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }

    println!("Shutting Down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Set the Response (Status_line) and file according to the request we get
    let (status_line, file) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", fs::read_to_string("hello.html").unwrap()),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", fs::read_to_string("slept.html").unwrap())
        }
        _ => (
            "HTTP/1.1 404 NOT FOUND",
            fs::read_to_string("404.html").unwrap(),
        ),
    };

    let content_len = file.len();

    let response = format!("{status_line}\r\nContent-Length: {content_len}\r\n\r\n{file}");

    stream.write_all(response.as_bytes()).unwrap();
}
