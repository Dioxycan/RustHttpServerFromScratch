#[derive(Debug,Clone)]
pub enum HttpVersion{
    V1_1,
    V2_0,
    V
}
pub const V1_1STR:&str = "HTTP/1.1";
pub const V2_0STR:&str = "HTTP/2.0";

impl HttpVersion{
    pub fn to_str(&self)->&str{
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