fn main() {
    let get = Method::GET; //Getting enum value
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;
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

struct Server {
    addr: String, 
}

impl Server { 
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {
        println!("On {}", self.addr);
    }
}

//Enums
struct Request {
    path: String, 
    query_string: String,
    method: Method,
}

enum Method {
    GET, //0  
    DELETE, //1
    POST, //2  //if we set POST = 5, then everything after would be 6, 7, 8, ...
    PUT, //....
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/
