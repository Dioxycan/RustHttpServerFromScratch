use std::net::{TcpListener,TcpStream};
use std::io::{Read,self,Write};
use std::io::prelude::*;
use std::result::Result;
use std::fs;
use std::str;
use crate::http::{
    request::HttpRequest,
    response::HttpResponse,
    types::status::HTTP_404,
    types::method::HttpMethod,
};

const NOT_FOUND:&str="<h1>404 Not Found</h1>";
const NOT_FOUND_LEN:&str ="22";
mod http;
fn main() -> Result<(),io::Error>{
    
    let r = Router{
        statics:"dist".to_string()
    };
    let server = Server::build("127.0.0.1:9000", r)?;
    server.run();
    Ok(())

}

struct Router{
    statics:String
}
impl<'a> Router {
    fn route(&self,req:HttpRequest)->Result<HttpResponse<'a>,io::Error>{
        let mut res = HttpResponse::default();
        match req.headers.get("Sec-Fetch-Dest") {
            Some(s)=>{
                if s.trim() != "empty"{
                    match req.headers.get("Sec-Fetch-Mode"){
                        Some(v)=>{
                            println!("{}",v);
                            print!("{}\r\n",req.uri);
                            match v.as_str().trim(){
                                "no-cors"=>{
                                    let mut url = String::new();
                                    if req.uri.starts_with("/."){
                                        url = format!("{}/{}",self.statics,&req.uri[2..]);
                                    }else{
                                        url = format!("{}/{}",self.statics,&req.uri[1..]);
                                    }
                                    
                                    let mut types =String::new();
                                    match req.headers.get("Sec-Fetch-Mode"){
                                        Some(vv)=>{
                                            match vv[..].trim(){
                                                "image"=>{
                                                    types= format!("{}/{}","image","ico");
                                                },
                                                "style"=>{
                                                    types= format!("{}/{}","text","css");
                                                }
                                                _=>{}
                                            }
                                        }
                                        None=>{}
                                    }
                                    let f= fs::read(url)?;
                                    res.add_header("Content-Type".to_string(),types);
                                    res.add_header("Content-Length".to_string(),f.len().to_string());
                                    res.body=Some(f);
                                },
                                "cors"=>{
                                    let mut url = String::new();
                                    if req.uri.starts_with("/."){
                                        url = format!("{}/{}",self.statics,&req.uri[2..]);
                                    }else{
                                        url = format!("{}/{}",self.statics,&req.uri[1..]);
                                    }
                                    print!("{}\r\n",url);
                                    let f= fs::read(url)?;
                                    res.add_header("Content-Type".to_string(), "text/javascript".to_string());
                                    res.add_header("Content-Length".to_string(),f.len().to_string());
                                    res.body=Some(f);
                                },
                                "navigate"=>{
                                    let mut url = String::new();
                                        url = format!("{}/{}",self.statics,"index.html");
                                        print!("{}\r\n",url);
                                        let f = fs::read(url)?; 
                                        res.add_header("Content-Type".to_string(), "text/html".to_string());
                                        res.add_header("Content-Length".to_string(),f.len().to_string());
                                        res.body=Some(f);
            
                                    
                                },
                                _=>{
                                    res.status = HTTP_404;
                                    res.add_header("Content-Type".to_string(), "text/html".to_string());
                                    res.add_header("Content-Length".to_string(), NOT_FOUND_LEN.to_string());
                                    res.body = Some(NOT_FOUND.as_bytes().to_vec());
                                }
            
                            }
                        }
                            None=>{}
                    }
                }else{
                    
                }
            },
            None=>{}
        }
        println!("dfdfd");
        Ok(res)
    
    }

}
struct Server{
    _sock_addr:String,
    listener:TcpListener,
    router:Router,
}
impl Server{
    fn build(sock_addr:&str,router:Router)->Result<Server,io::Error>{
        let listener = TcpListener::bind(sock_addr)?;
        Ok(Server{
            _sock_addr:sock_addr.to_string(),
            listener,
            router
        })
    }
    fn run(&self){
        for stream in self.listener.incoming(){
            let stream = stream.unwrap();
            self.handle_stream(stream).unwrap();
        }
        
    }
    fn handle_stream(&self,mut stream:TcpStream)->Result<(),io::Error>{
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




