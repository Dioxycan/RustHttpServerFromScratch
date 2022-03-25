
use std::io;

use std::result::Result;
use crate::http::{
    request::HttpRequest,
    response::HttpResponse,
};
use crate::server::server::{
    Server,
    MultiThread,
    Work,
    Worker,
    Job,
    handle_stream
};
use crate::router::router::{
    Router,
    Api,
    ApiMethod,
    ApiView,

};
mod client;
mod router;
mod http;
mod server;
extern crate num_cpus;
fn main() -> Result<(),io::Error>{

    fn get_view(req:&HttpRequest,res: &mut HttpResponse){
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

    fn r_fn()->Router{
        Router{
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
        }
    }
    let server = Server::build("127.0.0.1:9000", r.clone())?;
    let multi = MultiThread::build(4,r_fn);
    server.run(multi);
    Ok(())

}
