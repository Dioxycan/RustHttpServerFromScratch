use std::net::{TcpListener,TcpStream};
use std::io::prelude::*;
use std::io;
use crate::http;
use crate::router;
use router::router::{
Router
};
use http::{
    request::HttpRequest,
};

pub struct Server{
    pub _sock_addr:String,
    pub listener:TcpListener,
    pub router:Router,

}
impl<'a> Server{
    pub fn build(sock_addr:&str,router:Router)->Result<Server,io::Error>{
        let listener = TcpListener::bind(sock_addr)?;
        Ok(Server{
            _sock_addr:sock_addr.to_string(),
            listener,
            router,
        })
    }
    pub fn run(&self){
        for stream in self.listener.incoming(){
            let stream = stream.unwrap();
            self.handle_stream(stream).unwrap();
        }
        
    }
    pub fn handle_stream(&self,mut stream:TcpStream)->Result<(),io::Error>{
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;
        let r = String::from_utf8(buffer.to_vec()).unwrap();
        println!("{}",r);
        let res = self.router.route(HttpRequest::from(r))?;
        stream.write(&res.send()[..])?;
        stream.flush()?;
        Ok(())
    }

}

