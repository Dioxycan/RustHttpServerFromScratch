
use std::net::{TcpListener,TcpStream};
use std::io::prelude::*;
use std::os::unix::prelude::JoinHandleExt;
use std::{io,time::Duration};
use std::boxed::Box;
use std::sync::{Arc,mpsc, Mutex};
use std::thread;
use crate::http;
use crate::router;
use router::router::{
Router
};
use http::{
    request::HttpRequest,
};
pub type Job = Box<dyn FnOnce(&Router) + Send + 'static>;
pub enum Work{
    NewJob(TcpStream),
    Terminate
}
pub struct Worker{
    pub id:usize,
    pub thread:thread::JoinHandle<()>,
}
impl Worker{
    pub fn new(id:usize,router:Router,receiver:Arc<Mutex<mpsc::Receiver<Work>>>)->Worker{
        let thread = thread::spawn(move || {
            let router = router;
            loop{
            let work = receiver.lock().unwrap().recv().unwrap();
            match work {
                Work::NewJob(mut stream) => {
                    println!("Worker {} got a job; executing.", id);
                    let mut buffer = [0; 1024];
                    let result=stream.read(&mut buffer);
                    handle_error(result);
                    let r = String::from_utf8(buffer.to_vec()).unwrap();
                    println!("{}",r);
                    let res = router.route(HttpRequest::from(r)).unwrap();
                    let result=stream.write(&res.send()[..]);
                    handle_error(result);

                    let result=stream.flush();
                    handle_error(result);

                }
                Work::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        }
    });
        Worker { id, thread}
    }
}
pub struct MultiThread{
    threads:Vec<Worker>,
    sender:mpsc::Sender<Work>
}
impl MultiThread{
    pub fn build<F>(size:usize,router_fn:F)->MultiThread
    where 
        F:Fn()->Router
    {
        assert!(size >0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
            let mut threads = Vec::with_capacity(size);
            for id in 0..size {
                threads.push(Worker::new(id,router_fn(), Arc::clone(&receiver)));
            }
        MultiThread{threads,sender}

    }
    pub fn execute(&self,stream: TcpStream)
    {
        self.sender.send(Work::NewJob(stream)).unwrap();
    }
}
pub struct Server{
    pub _sock_addr:String,
    pub listener:TcpListener,
    pub router:Router,
}
impl Server{
    pub fn build(sock_addr:&str,router:Router)->Result<Server,io::Error>{
        let listener = TcpListener::bind(sock_addr)?;
        Ok(Server{
            _sock_addr:sock_addr.to_string(),
            listener,
            router,
        })
    }
    pub fn run(&self,multi:MultiThread)
    {
        for stream in self.listener.incoming(){
            let stream = stream.unwrap();
            multi.execute(stream);
        }
        
    }
}

pub fn handle_stream(mut stream:TcpStream,router:Router)->Result<(),io::Error>{

    Ok(())
}
pub fn handle_error<t>(r:Result<t,io::Error>){
    match r{
        Ok(e)=>{},
        Err(e)=>{println!("{}",e)}
    }
}