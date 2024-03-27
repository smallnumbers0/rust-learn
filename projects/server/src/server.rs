use std::net::{TcpListener, TcpStream};
use crate::http::request;
use crate::http::Request; //crate means root of entire crate
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Read;

pub struct Server {
    addr: String, 
}

fn arr(a: &[u8]) { //a reference to an array is a slice

}

impl Server { 
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(self) {
        println!("On {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Request Recieved: {:?}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("Failed {}", e),
                            }
                        

                        },
                        Err(e) => println!("Failed {}", e),
                    }
                },
                // _=> {} //default error case
                Err(e) => println!("{}", e),
                
            }    
        }
    }
}
