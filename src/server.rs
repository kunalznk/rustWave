 
use std::{net::TcpListener, io::{Read}};

use crate::http::{Request, Response, StatusCode, ParseError};

pub trait  Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    } 
}

pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr,
        }
    }

    pub fn run (&self, mut handler: impl Handler)  {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Server is running {:?}" ,8080);
        for conn in listener.incoming() {
            let mut buf = [0; 1024];
            match conn {
                Ok(mut stream, ) => {
                
                    let request = stream.read(&mut buf);
    
                    match request {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));
    
                            let response = match Request::try_from(&buf[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };
    
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
    
                    }
             
    
                }
                Err(e) => println!("Sever cant accept more connection due to {}", e)
            }
        }

        
    }
}

/*
let conn = listener.accept();
        let mut buf = [0; 1024];
        match  conn {
            Ok((mut stream, _)) => {
                
                let request = stream.read(&mut buf);

                match request {
                    Ok(_) => {
                        println!("Received a request: {}", String::from_utf8_lossy(&buf));

                        let response = match Request::try_from(&buf[..]) {
                            Ok(request) => handler.handle_request(&request),
                            Err(e) => handler.handle_bad_request(&e),
                        };

                        if let Err(e) = response.send(&mut stream) {
                            println!("Failed to send response: {}", e);
                        }
                    }
                    Err(e) => println!("Failed to read from connection: {}", e),

                }
         

            }
            Err(e) => println!("Sever cant accept more connection due to {}", e)
        }

*/