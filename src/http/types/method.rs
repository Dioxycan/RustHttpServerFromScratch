#[derive(Debug,Clone,Copy)]
pub enum HttpMethod{
    GET,
    POST,
    CREATE,
    PUT,
    DELETE,
    OPTION,
    Undefind
    
}

impl HttpMethod{
    pub fn new(s: &str)->HttpMethod{
        match s{
            "GET"=>HttpMethod::GET,
            "POST"=>HttpMethod::POST,
            "DELETE"=>HttpMethod::DELETE,
            "CREATE"=>HttpMethod::CREATE,
            "PUT"=>HttpMethod::PUT,
            "OPTION"=>HttpMethod::OPTION,
            _ =>HttpMethod::Undefind
        }
    }
}

impl PartialEq for HttpMethod {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}