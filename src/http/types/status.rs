#[non_exhaustive]
#[derive(Clone,Debug)]
pub struct HttpStatus<'a>(pub i32,pub &'a str);
pub const HTTP_200:HttpStatus =HttpStatus(200,"Ok");
pub const HTTP_404:HttpStatus =HttpStatus(404,"Not Found");