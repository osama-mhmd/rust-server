use std::{
  fs,
  io::{prelude::*, BufReader},
  net::{TcpListener, TcpStream},
};

fn main() {
  let listener = TcpListener::bind("localhost:8080").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection(stream);
  }
}

fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&mut stream);
  let request_line = buf_reader.lines().next().unwrap().unwrap();

  handle_request(stream, request_line);
}

fn handle_request(mut stream: TcpStream, request_line: String) {
  #[allow(unused_variables)]
  let [method, path, http_type] = request_line.split(" ").collect::<Vec<&str>>()[..] else {
    todo!("Make it please")
  };

  // so usually we are working with HTTP/1.1, if you working with another thing
  // -> you will have to customize

  #[allow(unused_variables)]
  let ok_status_line = "HTTP/1.1 200 OK";
  #[allow(unused_mut)]
  let mut status_line = ok_status_line;

  let file_path = match path {
    "/" => html_path("index"),
    _ => html_path("not-found"),
  };

  let contents = read_file(file_path.as_str());

  let length = contents.len();
  let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
  stream.write_all(response.as_bytes()).unwrap();

  println!("\t{status_line} => {path}")
}

fn read_file(file_path: &str) -> String {
  fs::read_to_string(format!("{file_path}")).unwrap()
}

fn html_path(file_name: &str) -> String {
  format!("views/{file_name}.html")
}
