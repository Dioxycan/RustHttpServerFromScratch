use crate::http;
use std::io;
use std::fs;
use http::{
    request::HttpRequest,
    response::HttpResponse,
    types::status::{HTTP_200,HTTP_404},
    types::method::HttpMethod,
    
};
const NOT_FOUND:&str="<h1>404 Not Found</h1>";
const API_NOT_FOUND:&str="<h1>API Not Found</h1>";
#[derive(Clone)]
pub struct Router{
    pub statics:String,
    pub api:Option<Vec<Api>>
}
impl<'a> Router {
    pub fn route(&self,req:HttpRequest)->Result<HttpResponse<'a>,io::Error>{
        let mut res = HttpResponse::default();
        match req.headers.get("Sec-Fetch-Dest") {
            Some(s)=>{
                if s.trim() != "empty"{
                    self.route_static(req,&mut res)?;
                }else{
                    self.route_api(&req,&mut res)?;
                }
            },
            None=>{}
        }
        Ok(res)
    
    }
    pub fn route_static(&self,req:HttpRequest,res:&mut HttpResponse)->Result<(),io::Error>{
        match req.headers.get("Sec-Fetch-Mode"){
            Some(v)=>{
                println!("{}",v);
                print!("{}\r\n",req.uri);
                match v.as_str().trim(){
                    "no-cors"=>{
                        let mut url = String::new();
                        let mut f:Vec<u8>=vec!(); 
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
                                        
                                    },
                                    "style"=>{
                                        let f= fs::read(url)?;
                                    }
                                    _=>{}
                                }
                            }
                            None=>{}
                        }
                        res.add_header("Content-Type".to_string(),types);
                        res.add_header("Content-Length".to_string(),f.len().to_string());
                        res.body=Some(f);
                    },
                    "cors"=>{
                        let mut url = String::new();
                        println!("{}",req.uri);
                        if req.uri.starts_with("/."){
                            url = format!("{}/{}",self.statics,&req.uri[2..]);
                        }else{
                            url = format!("{}/{}",self.statics,&req.uri[1..]);
                        }
                        res.add_header("Access-Control-Allow-Origin".to_string(),"*".to_string());
                        res.add_header("Access-Control-Allow-Headers".to_string(),"*".to_string());
                        print!("{}\r\n",url);
                        let f= fs::read(url)?;
                        res.add_header("Content-Type".to_string(), "text/javascript".to_string());
                        res.add_header("Content-Length".to_string(),f.len().to_string());
                        res.body=Some(f);
                    },
                    "navigate"=>{
                        let mut url = String::new();
                        if(req.uri =="/"){
                            url = format!("{}/index.html",self.statics);
                        }else{
                            url = format!("{}{}.html",self.statics,req.uri);
                            print!("{}\r\n",url);
                          
                        }
                        let f = fs::read(url)?; 
                        res.add_header("Access-Control-Allow-Origin".to_string(),"*".to_string());
                        res.add_header("Access-Control-Allow-Headers".to_string(),"*".to_string());
                        res.add_header("Content-Type".to_string(), "text/html".to_string());
                        res.add_header("Content-Length".to_string(),f.len().to_string());
                        res.body=Some(f);

                        
                    },
                    _=>{
                        res.status = HTTP_404;
                        res.add_header("Content-Type".to_string(), "text/html".to_string());
                        res.add_header("Content-Length".to_string(), NOT_FOUND.len().to_string());
                        res.body = Some(NOT_FOUND.as_bytes().to_vec());
                    }

                }
            }
                None=>{}
        }
        Ok(())
    }
    pub fn route_api(&self,req:&HttpRequest,res:&mut HttpResponse)->Result<(),io::Error>{
   
        match &self.api{
            Some(api)=>{

                for app in api{
                    if req.uri.starts_with(app.url.as_str()){

                        for view in &app.views{
                            if req.uri.starts_with(format!("{}{}",app.url,view.url).as_str()){
                                res.add_header("Access-Control-Allow-Origin".to_string(),"*".to_string());
                                res.add_header("Access-Control-Allow-Headers".to_string(),"*".to_string());
                                res.status = HTTP_200;
                                match req.method {
                                    HttpMethod::GET=>{
                                        match &view.api_methods.GET{
                                            Some(f)=>{
                                                f(req,res);
                                            },
                                            None=>{
                                                res.body = Some("API_NOT_FOUND GET".as_bytes().to_vec());
                                                
                                            }
                                        }
                                    },
                                    HttpMethod::POST=>{
                                        match &view.api_methods.POST{
                                            Some(f)=>{f(req,res);},
                                            None=>{res.body = Some("API_NOT_FOUND POST".as_bytes().to_vec());}
                                        }
                                    },
                                    HttpMethod::CREATE=>{
                                        match &view.api_methods.CREATE{
                                            Some(f)=>{f(req,res);},
                                            None=>{res.body = Some("API_NOT_FOUND CREATE".as_bytes().to_vec());}
                                        }
                                    },
                                    HttpMethod::PUT=>{
                                        match &view.api_methods.PUT{
                                            Some(f)=>{f(req,res);},
                                            None=>{res.body = Some("API_NOT_FOUND PUT".as_bytes().to_vec());}
                                        }
                                    },
                                    HttpMethod::DELETE=>{
                                        match &view.api_methods.DELETE{
                                            Some(f)=>{f(req,res);},
                                            None=>{res.body = Some("API_NOT_FOUND DELETE".as_bytes().to_vec());}
                                        }
                                    },
                                    HttpMethod::OPTIONS=>{
                                        match &view.api_methods.OPTION{
                                            Some(f)=>{f(req,res);},
                                            None=>{res.body = Some("API_NOT_FOUND OPTION".as_bytes().to_vec());}
                                        }
                                    },
                                    HttpMethod::Undefind=>{
                                        res.body = Some("API_NOT_FOUND".as_bytes().to_vec());
                                    }

                                }
                            }
                        }
                    }     
                }
            },
            None=>{
                res.status = HTTP_200;
                res.add_header("Content-Length".to_string(), API_NOT_FOUND.len().to_string());
                res.add_header("Content-Type".to_string(),"text/html".to_string());
                res.body = Some(API_NOT_FOUND.as_bytes().to_vec());
            }
        }
        Ok(())
    }
}


pub type View = fn(req:&HttpRequest,res:&mut HttpResponse);
#[derive(Clone)]

pub struct ApiMethod{
    pub GET:Option<View>,
    pub POST:Option<View>,
    pub CREATE:Option<View>,
    pub PUT:Option<View>,
    pub DELETE:Option<View>,   
    pub OPTION:Option<View>,
}
#[derive(Clone)]

pub struct ApiView{
    pub url:String,
    pub api_methods:ApiMethod,
}
#[derive(Clone)]

pub struct Api{
    pub url:String,
    pub views:Vec<ApiView>,
}
