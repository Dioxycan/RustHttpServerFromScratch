use super::types::{version::HttpVersion,status::{HTTP_200,HttpStatus}};
use std::collections::HashMap;
#[derive(Clone,Debug)]
pub struct HttpResponse <'a>{
   pub version:HttpVersion,
   pub status:HttpStatus<'a>,
   pub headers:Option<HashMap<String,String>>,
   pub body:Option<Vec<u8>>,
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
    pub fn add_header(& mut self,key: String,value:String){
        let mut new :HashMap<String,String>= HashMap::new();
        new.insert(key.clone(), value.clone());
        match &self.headers {
            Some(v)=>{
                new = v.clone();
                new.insert(key, value);
                self.headers=Some(new);
            }
            None=>{
                self.headers=Some(new);
            }
        }
        
        
    }
    pub fn send(&self)->Vec<u8>{
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
