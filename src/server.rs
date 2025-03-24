use std::{
    error::Error,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread::sleep,
    time::Duration,
};

use crate::ThreadPool;
use regex::Regex;

pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub fn run(&self, addr: &str, threads: usize) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(addr)?;
        let pool = ThreadPool::new(threads);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            pool.execute(|| handle_connection(stream));
        }

        Ok(())
    }
}

fn parse_request(stream: &TcpStream) -> Vec<String> {
    BufReader::new(stream)
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect()
}

fn extract_path(request: &Vec<String>) -> &str {
    let re = Regex::new("GET (/.*) HTTP/1.1").unwrap();

    request
        .get(0)
        .and_then(|x| re.captures(x))
        .and_then(|x| x.get(1))
        .map(|x| x.as_str())
        .unwrap_or("")
}

fn find_page(path: &str) -> String {
    let success = "HTTP/1.1 200 OK";
    let failure = "HTTP/1.1 404 NOT FOUND";

    let (status_line, path) = match path {
        "/" => (success, "web/index.html"),
        "/sleep" => {
            sleep(Duration::from_secs(10));
            (success, "web/index.html")
        }
        _ => (failure, "web/404.html"),
    };

    let content = fs::read_to_string(path).unwrap();
    let len = content.len();
    format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{content}")
}

fn handle_connection(mut stream: TcpStream) {
    let ip = stream.local_addr().unwrap().ip();
    let request = parse_request(&stream);
    let path = extract_path(&request);
    let response = find_page(path);

    println!("{ip} {path}");

    stream.write_all(response.as_bytes()).unwrap();
}
