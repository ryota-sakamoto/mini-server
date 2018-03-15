extern crate clap;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate simplelog;

use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::env;
use simplelog::*;
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Request, Response, Service};
use clap::{App, Arg};

fn main() {
    init_log();
    let app = init_clap();
    let matches = app.get_matches();
    let port = match matches.value_of("port") {
        Some(s) => s.parse().unwrap(),
        None => 3000,
    };
    let current_dir = env::current_dir().unwrap();
    let current_path = current_dir.to_str().unwrap();
    let root_path = match matches.value_of("root") {
        Some(r) => r.to_string(),
        None => current_path.to_string(),
    };

    let ip = format!("127.0.0.1:{}", port);
    info!("Start {}", ip);
    let addr = ip.parse().unwrap();
    let server = Http::new()
        .bind(&addr, move || {
            Ok(Server {
                root_path: root_path.to_string(),
            })
        })
        .unwrap();
    server.run().unwrap();
}

fn init_log() {
    CombinedLogger::init(vec![
        SimpleLogger::new(LevelFilter::Info, Config::default()),
    ]).unwrap();
}

fn init_clap<'a, 'b>() -> App<'a, 'b> {
    App::new("mini-server")
        .arg(
            Arg::with_name("port")
                .help("port")
                .short("p")
                .long("port")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("root")
                .help("document root")
                .short("r")
                .long("root")
                .takes_value(true),
        )
}

struct Server {
    root_path: String,
}

impl Server {
    fn callback(&self, req: &Request) -> Result<String, String> {
        let path = req.uri().path().to_string().replacen("/", "", 1);
        let absolute_path = format!("{}/{}", self.root_path, path);
        let p = Path::new(&absolute_path);
        info!("{}", absolute_path);

        let mut file = try!(File::open(&p).map_err(|e| e.to_string()));
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        Ok(s)
    }
}

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<futures::Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let res = self.callback(&req);
        let r = match res {
            Ok(r) => r,
            Err(e) => e,
        };

        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(r.len() as u64))
                .with_header(ContentType::plaintext())
                .with_body(r),
        ))
    }
}
