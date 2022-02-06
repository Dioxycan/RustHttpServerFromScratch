
use std::io;

use std::result::Result;
use crate::http::{
    request::HttpRequest,
    response::HttpResponse,
};
use crate::server::server::{
    Server
};
use crate::router::router::{
    Router,
    Api,
    ApiMethod,
    ApiView,

};
mod router;
mod http;
mod server;

fn main() -> Result<(),io::Error>{

    fn get_view(req:&HttpRequest,res: &mut HttpResponse){
        println!("its working lol");
        println!("{:#?}",req);
        let body = r#"{
            "glossary": {
                "title": "example glossary",
                "GlossDiv": {
                    "title": "S",
                    "GlossList": {
                        "GlossEntry": {
                            "ID": "SGML",
                            "SortAs": "SGML",
                            "GlossTerm": "Standard Generalized Markup Language",
                            "Acronym": "SGML",
                            "Abbrev": "ISO 8879:1986",
                            "GlossDef": {
                                "para": "A meta-markup language, used to create markup languages such as DocBook.",
                                "GlossSeeAlso": ["GML", "XML"]
                            },
                            "GlossSee": "markup"
                        }
                    }
                }
            }
        }"#;
        res.add_header("Content-Length".to_string(), body.len().to_string());
        res.add_header("Content-Type".to_string(), "application/json".to_string());

        res.body=Some(body.as_bytes().to_vec());
    }
    //let a:Box::<View<'a>> = Box::new(get_view);
    let r = Router{
        statics:"dist".to_string(),
        api:Some(vec!(Api{
            url:String::from("/api"),
            views:vec!(ApiView{
                api_methods:ApiMethod{
                    CREATE:None,
                    DELETE:None,
                    PUT:None,
                    OPTION:None,
                    POST:None,
                    GET:Some(get_view)
                },
                url:String::from("/")
            })

        })),
    };
  
    let server = Server::build("127.0.0.1:9000", r)?;
    server.run();
    Ok(())

}