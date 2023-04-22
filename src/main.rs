use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    env,
    str,
};
use rust_embed::RustEmbed;


#[derive(RustEmbed)]
#[folder = "html"]
#[prefix = "html/"]
struct Asset;
fn main() {

    let path = env::current_dir().unwrap();
    println!("The current directory is: {}", path.display());

    let default_port = "8000".to_string();
    let port = env::var("PORT")
        .unwrap_or(default_port);
        // .expect("Variable for port not found");

    let listener = TcpListener::bind(format!("{}{}","0.0.0.0:",port))
        .expect("Failed to bind to port");

    println!("Successfully connected to port: {}", port);

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

    let hello_html =
        Asset::get("html/hello.html")
        .expect("Failed to access the html file");

    // let path = PathBuf::from("./html/hello.html");

    // let contents = fs::read_to_string(&path);

    let contents = std::str::from_utf8(hello_html.data.as_ref());
    match contents {
        Ok(contents) => {
            println!("The content was successfully loaded");
            let length = contents.len();

            let response =
                format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        
            let final_response = stream.write_all(response.as_bytes());
        
            match final_response {
                Ok(_) => println!("Successfully wrote response"),
                Err(e) => println!("Failed to write the response: {e:?}"),
            }
        },
        Err(e) => println!("Failed to interpret the UTF-8 string: {e:?}"),
    }
        // .expect("Error finding html file, path procided is: ")

    // .expect("Failed to write response");
}