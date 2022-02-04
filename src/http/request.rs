use super::types::{version::HttpVersion,method::HttpMethod};
use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpRequest{
    pub method:HttpMethod,
    pub uri:String,
    pub version:HttpVersion,
    pub headers:HashMap<String,String>,
    pub body:Option<usize>

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
                method = HttpMethod::from(word[0]);
                uri = String::from(word[1]);
                version = HttpVersion::from(word[2]);
            }else if line == ""{
                break;
            }else{
                let mut index :usize=line.len();
                let mut i :usize=0;
                let bytes = line.as_bytes();
                while i<index{
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
        body=Some(body.unwrap()+8);
        HttpRequest{
            method,
            uri,
            version,
            headers,
            body
        }
    }
}