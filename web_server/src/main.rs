use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    println!("Shutting down.");
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (path, query_opt) = parse_path_and_query(&request_line);

    let (status_line, contents) = match path {
        "/" => {
            let contents = fs::read_to_string("hello.html").unwrap();
            ("HTTP/1.1 200 OK", contents)
        }
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            let contents = fs::read_to_string("hello.html").unwrap();
            ("HTTP/1.1 200 OK", contents)
        }
        "/square" => {
            if let Some(query) = query_opt {
                match extract_query_param(query, "num").and_then(|v| v.parse::<i32>().ok()) {
                    Some(num) => {
                        match num.checked_mul(num) {
                            Some(square) => {
                                let html = format!(
                                    "<html><body><h1>{} squared is {}</h1></body></html>",
                                    num, square
                                );
                                ("HTTP/1.1 200 OK", html)
                            }
                            None => (
                                "HTTP/1.1 400 BAD REQUEST",
                                "<html><body><h1>Number too large to square safely</h1></body></html>"
                                    .to_string(),
                            ),
                        }
                    }
                    None => (
                        "HTTP/1.1 400 BAD REQUEST",
                        "<html><body><h1>Invalid or missing number</h1></body></html>".to_string(),
                    ),
                }
            } else {
                (
                    "HTTP/1.1 400 BAD REQUEST",
                    "<html><body><h1>Missing query</h1></body></html>".to_string(),
                )
            }
        }
        _ => {
            let contents = fs::read_to_string("404.html").unwrap();
            ("HTTP/1.1 404 NOT FOUND", contents)
        }
    };
    
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}


fn parse_path_and_query(request_line: &str) -> (&str, Option<&str>) {
    let path_start = request_line.find(' ').unwrap_or(0) + 1;
    let path_end = request_line[path_start..]
        .find(' ')
        .map(|i| path_start + i)
        .unwrap_or(request_line.len());
    let full_path = &request_line[path_start..path_end];

    match full_path.find('?') {
        Some(q_idx) => (&full_path[..q_idx], Some(&full_path[q_idx + 1..])),
        None => (full_path, None),
    }
}

fn extract_query_param<'a>(query: &'a str, key: &str) -> Option<&'a str> {
    for pair in query.split('&') {
        let mut kv = pair.splitn(2, '=');
        if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
            if k == key {
                return Some(v);
            }
        }
    }
    None
}
