#[macro_use] extern crate log;
extern crate simplelog;
extern crate hyper;
extern crate futures;

use std::fs::File;
use std::path::Path;
use std::env;
use std::io::Read;
use simplelog::*;
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Response, Request, Service};

fn main() {
    CombinedLogger::init(vec![
        SimpleLogger::new(LevelFilter::Info, Config::default())
    ]).unwrap();
    let mut args = env::args();
    let port = match args.len() {
        2 => args.nth(1).unwrap().parse().unwrap(),
        _ => 3000,
    };

    let ip = format!("127.0.0.1:{}", port);
    info!("Start {}", ip);
    let addr = ip.parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Server)).unwrap();
    server.run().unwrap();
}

struct Server;
impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<futures::Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let res = callback(&req);
        let r = match res {
            Ok(r) => r,
            Err(e) => e,
        };
        Box::new(futures::future::ok(
            Response::new().with_body(r)
        ))
    }
}

fn callback(req: &Request) -> Result<String, String> {
    let path = req.uri().path().to_string().replacen("/", "", 1);
    info!("{}", path);
    let p = Path::new(&path);
    
    let mut file = try!(File::open(&p).map_err(|e| e.to_string()));
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    Ok(s)
}