
use server::Server;
use http::Request;
use http::Method;

mod http;
mod server;
fn main() {
   
    // let string = String::from("127.0.0.1:8080");
    // let string_slice = &string[10..]; //Get everything after 10th byte
    // let string_burrow: &str = &string; 
    // let string_literal = "1234"; //string literals are known at compile time. This string slice is immutable
    // dbg!(&string);
    // dbg!(string_slice);
    // dbg!(string_burrow);
    // dbg!(string_literal);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();

    // let number: i32 = 42;
    // let string_number = number.to_string();
    // dbg!(string_number);
    // dbg!(number);
}


/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/
