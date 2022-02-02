use std::net::{TcpListener,TcpStream};
use std::io::{Read,self,Write};
use std::io::prelude::*;
use std::result::Result;
use std::fs;
use std::str;
use std::collections::HashMap;
const NOT_FOUND:&str="<h1>404 Not Found</h1>";
const NOT_FOUND_LEN:&str ="22";
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
                        res.body.insert(f);
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
                        res.body.insert(f);
                    },
                    "navigate"=>{
                        let mut url = String::new();
                        if req.uri == "/" {
                            url = format!("{}/{}",self.statics,"index.html");
                        }else {
                            url = format!("{}/{}.html",self.statics,req.uri);
                        }
                        print!("{}\r\n",url);
                        let f = fs::read(url)?; 
                        res.add_header("Content-Type".to_string(), "text/html".to_string());
                        res.add_header("Content-Length".to_string(),f.len().to_string());
                        res.body.insert(f);
                        
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

        Ok(res)
    
    }
}
struct Server{
    sock_addr:String,
    listener:TcpListener,
    router:Router,
}
impl Server{
    fn build(sock_addr:&str,router:Router)->Result<Server,io::Error>{
        let listener = TcpListener::bind(sock_addr)?;
        Ok(Server{
            sock_addr:sock_addr.to_string(),
            listener,
            router
        })
    }
    fn run(&self){
        for stream in self.listener.incoming(){
            let stream = stream.unwrap();
            self.handle_stream(stream);
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


#[derive(Debug,Clone,Copy)]
pub enum HttpMethod{
    GET,
    POST,
    CREATE,
    PUT,
    DELETE,
    Undefind
    
}

impl HttpMethod{
    fn new(s: &str)->HttpMethod{
        match s{
            "GET"=>HttpMethod::GET,
            "POST"=>HttpMethod::POST,
            "DELETE"=>HttpMethod::DELETE,
            "CREATE"=>HttpMethod::CREATE,
            "PUT"=>HttpMethod::PUT,
            _ =>HttpMethod::Undefind
        }
    }
}
#[derive(Debug,Clone)]
pub enum HttpVersion{
    V1_1,
    V2_0,
    V
}
const V1_1STR:&str = "HTTP/1.1";
const V2_0STR:&str = "HTTP/2.0";

impl HttpVersion{
    fn to_str(&self)->&str{
        match self{
            HttpVersion::V1_1=>V1_1STR,
            HttpVersion::V2_0=>V2_0STR,

            HttpVersion::V=>"HTTP/1.1"                   
        }
    }
}
impl From<&str> for HttpVersion{
    fn from(s:&str)->Self{
        match s{
            "HTTP/1.1" => HttpVersion::V1_1,
            "HTTP/2.0" => HttpVersion::V2_0,
            _ =>{
                HttpVersion::V
            }
        }
    }
}
#[non_exhaustive]
#[derive(Clone,Debug)]
pub struct HttpStatus<'a>(i32,&'a str);
pub const HTTP_200:HttpStatus =HttpStatus(200,"Ok");
pub const HTTP_404:HttpStatus =HttpStatus(404,"Not Found");

#[derive(Debug)]
pub struct HttpRequest{
    data:String,
    method:HttpMethod,
    uri:String,
    version:HttpVersion,
    headers:HashMap<String,String>,
    body:Option<usize>

}
impl From<String> for HttpRequest{
    fn from(s:String)->HttpRequest{
        let mut method = HttpMethod::Undefind;
        let mut uri =String::new();
        let mut version = HttpVersion::V;
        let mut headers = HashMap::new();
        for (i,line) in s.lines().enumerate(){
            if i == 0 {
                let word : Vec<&str>= line.split(" ").collect();
                method = HttpMethod::new(word[0]);
                uri = String::from(word[1]);
                version = HttpVersion::from(word[2]);
            }else if line == ""{
                break;
            }
            else{
                let mut index :usize=line.len();
                let mut i :usize=0;
                let bytes = line.as_bytes();
                loop{
                    if i+1 == index {
                        break;
                    }
                    if bytes[i] == b':'{
                        index = i;
                        break;
                    }
                    i+=1;
                }
                headers.insert(line[..index].to_string(),line[index+1..].to_string());
            }
        }
        let mut body = s.find("\r\n\r\n");
        body.insert(body.unwrap()+8);
        HttpRequest{
            data:s,
            method,
            uri,
            version,
            headers,
            body
        }
    }
}
#[derive(Clone,Debug)]
struct HttpResponse <'a>{
    version:HttpVersion,
    status:HttpStatus<'a>,
    headers:Option<HashMap<String,String>>,
    body:Option<Vec<u8>>,
}
impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status: HTTP_200,
            headers: None,
            body: None,
        }
    }
}impl<'a> HttpResponse<'a>{
    fn add_header(& mut self,key: String,value:String){
        let mut new :HashMap<String,String>= HashMap::new();
        new.insert(key.clone(), value.clone());
        match &self.headers {
            Some(v)=>{
                new = v.clone();
                new.insert(key, value);
                self.headers.insert(new);
            }
            None=>{
                self.headers.insert(new);
            }
        }
        
        
    }
    fn send(&self)->Vec<u8>{
        let mut bytes:Vec<u8> = String::from(self).as_bytes().to_vec();
        println!("{:#?}\r\n\r\n\r\n",String::from(self));
        match &self.body{
            Some(b)=>{
                bytes.extend(b);
            },
            None=>{}
        }
        bytes
    }
}
impl<'a> From<&HttpResponse<'a>> for String{
    fn from(res:&HttpResponse)->String{
        
        let headers = match &res.headers{
            Some(hs)=>{
                let new = hs.clone();
                let mut temp = String::new();
                for (k,v)in new{
                    temp = format!("{}{}:{}\r\n",temp,k,v);
                }
                temp
            },
            None=>String::new()
        };
        format!("{} {} {}\r\n{}\r\n",res.version.to_str(),res.status.0,res.status.1,headers)
    }
}
