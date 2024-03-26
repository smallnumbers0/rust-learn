fn main() {

    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..]; //Get everything after 10th byte
    let string_burrow: &str = &string; 
    let string_literal = "1234"; //string literals are known at compile time. This string slice is immutable
    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_burrow);
    dbg!(string_literal);
    // let server = Server::new("127.0.0.1:8080");
    // server.run();
}

struct Server {
    addr: String, 
}

impl Server {
    fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    fn run(self) {

    }
}