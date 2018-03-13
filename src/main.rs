extern crate hyper;
extern crate futures;

use std::fs::File;
use std::path::Path;
use std::env;
use std::io::Read;
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Response, Request, Service};

fn main() {
    let mut args = env::args();
    let port = match args.len() {
        2 => args.nth(1).unwrap().parse().unwrap(),
        _ => 3000,
    };

    let addr = format!("127.0.0.1:{}", port).parse().unwrap();
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
        Box::new(futures::future::ok(
            Response::new().with_body(res)
        ))
    }
}

fn callback(req: &Request) -> String {
    let path = req.uri().path().to_string().replacen("/", "", 1);
    let p = Path::new(&path);

    // TODO
    let mut file = File::open(&p).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s
}