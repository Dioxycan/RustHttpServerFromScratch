#[derive(Debug,Clone,Copy)]
pub enum HttpMethod{
    GET,
    POST,
    CREATE,
    PUT,
    DELETE,
    OPTIONS,
    Undefind
}
impl HttpMethod{
    pub fn new()->HttpMethod{
        HttpMethod::Undefind
    }
    pub fn to_str(&self)->&str{
        match self{
            HttpMethod::GET=>"GET",
            HttpMethod::POST=>"POST",
            HttpMethod::CREATE=>"CREATE",
            HttpMethod::DELETE=>"DELETE",
            HttpMethod::PUT=>"PUT",
            HttpMethod::OPTIONS=>"OPTIONS",
            HttpMethod::Undefind=>"GET",
        }
    }

}
impl From<&str> for HttpMethod{
    fn from(s: &str)->HttpMethod{
        match s{
            "GET"=>HttpMethod::GET,
            "POST"=>HttpMethod::POST,
            "DELETE"=>HttpMethod::DELETE,
            "CREATE"=>HttpMethod::CREATE,
            "PUT"=>HttpMethod::PUT,
            "OPTIONS"=>HttpMethod::OPTIONS,
            _ =>HttpMethod::Undefind
        }
    }
}
impl PartialEq for HttpMethod {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}