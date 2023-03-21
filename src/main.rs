use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    env
};

fn main() {

    let path = env::current_dir().unwrap();
    println!("The current directory is: {}", path.display());

    let port = env::var("PORT")
        .expect("Variable for port not found");

    let listener = TcpListener::bind(format!("{}{}","0.0.0.0:",port))
        .expect("Failed to bind to port");

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to create TCP stream");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result
                .expect("failed to get result rom http request lines"))
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("./html/hello.html")
        .expect("Error finding html file");
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).expect("Failed to write response");
}