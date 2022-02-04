use super::url::Url;
use super::method::HttpMethod;
pub struct Header{
    key:String,
    value:String
}
impl Header {
    pub fn new(k:&str,v:&str)->Header{
        Header{
            key:k.to_string(),
            value:v.to_string()
        }
    }
    pub fn to_str(&self)->String{
        if self.value.starts_with(" "){
            format!("{}:{}",self.key,self.value)
        }else{
            format!("{}: {}",self.key,self.value)
        }
    }
}
enum HttpHeader{
    Some(Header),
    ContentLength{len:usize},
    ContentType{ctype:Ctype,csub_types:CsubTypes},
    AcceptLanguage{lng:String},
    Referer{url:Url},
    SecFetchSite{site:FetchSite},
    SecFetchMode{mode:FetchMode},
    SecFetchDest{dest:FetchDest},
    SecFetchUser{user:bool},
    AccessControlAllowOrigin{origin:AllowOrigin},
    AccessControlAllowHeaders{header:AllowHeader},
    AccessControlAllowMethods{method:AllowMethods},
    AccessControlRequestHeaders{header:RequestHeader},
    AccessControlRequestMethods{method:RequestMethod},
}
enum RequestHeader{
    All,
    Some(Vec<String>)
}
enum RequestMethod{
    All,
    Some(Vec<HttpMethod>)
}
enum AllowMethods{
    All,
    Some(Vec<HttpMethod>)
}
enum AllowHeader{
    All,
    Some(Vec<String>),
}
enum AllowOrigin{
    All,
    Origin(Url),
    Null
}
enum FetchDest{
    Audio,
    AudioWorklet,
    Document,
    Embed,
    Empty,
    Font,
    Frame,
    Iframe,
    Image,
    Manifest,
    Object,
    PaintWorklet,
    Report,
    Script,
    ServiceWorker,
    SharedWorker,
    Style,
    Track,
    Video,
    Worker,
    Xslt
}
enum FetchMode{
    Cors,
    Navigate,
    NoCors,
    SameOrigin,
    WebSocket
}
enum FetchSite{
    CrossSite,
    SameOrigin,
    SameSite,
    Empty,
}
enum Ctype{
    Text,
    Application,
    Font,
    Image,
}
enum CsubTypes{
    Plain,
    Css,
    Html,
    Javascript,
    Png,

}