use hyper::server::conn::AddrIncoming;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use hyper_rustls::TlsAcceptor;
use rustls::{Certificate, ServerConfig, PrivateKey};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::io::{Error, BufReader};
use std::fs::{read,File};
use std::sync::Arc;
#[tokio::main]
async fn main() {
    //let addr = "192.168.1.131:8080".parse().unwrap();
    let addr = "127.0.0.1:8080".parse().unwrap();
    let mut reader = match File::open("pem/privatekey.pem") {
        Ok(o) => BufReader::new(o),
        Err(e) => panic!("Problem opening private key file: {:?}",e),
    };
    let loader = pkcs8_private_keys(&mut reader).unwrap();
    let key = PrivateKey(loader[0].clone());
    let mut reader = match File::open("pem/cert.pem"){
        Ok(o) => BufReader::new(o),
        Err(e) => panic!("Problem opening Certificate file: {:?}",e),
    };
    let loader = certs(&mut reader).unwrap();
    let cert = loader.into_iter().map(Certificate).collect();
    let incoming = AddrIncoming::bind(&addr).unwrap();
    let serverconfig = Arc::new(ServerConfig::builder().with_safe_defaults().with_no_client_auth().with_single_cert(cert, key).unwrap());
    let acceptor = TlsAcceptor::new(serverconfig, incoming);
    let service = make_service_fn(|_| async { Ok::<_, Error>(service_fn(|req| async {serving(req)}))});
    if let Err(e) = Server::builder(acceptor).serve(service).await {
        eprintln!("Server Error: {}", e)
    }
}
fn serving(req: Request<Body>) -> Result<Response<Body>, Error> {
    let (status ,filename,_type) = match req.uri().path() {
        "/" => (200,"html/index.html","text/html"),
        "/about" => (200,"html/about.html","text/html"),
        "/terminal-script.js" => (200, "js/terminal-script.js","text/javascript"),
        "/terminal-styles.css" => (200,"css/terminal-styles.css","text/css"),
        "/favicon.ico" => (200,"img/favicon.ico","image/x-icon"),
        "/terminal" => (200,"html/terminal.html","text/html"),
        "/blog" => (200,"html/blog.html","text/html"),
        "/home-styles.css" => (200,"css/home-styles.css","text/css"),
        "/home-script.js" => (200,"js/home-script.js","text/javascript"),
        "/him.jpg" => (200,"img/him.jpg","image/jpg"),
        "/robots.txt" =>(200,"robots.txt","text/plain"),
        "/resume" => (200,"html/resume.html","text/html"),
        "/contacts" => (200, "html/contacts.html","text/html"),
        "/DDNS" => (200,"test.html","text/html"),
        _ => (404,"html/404.html","text/html"),
    };
    let contents = match read(filename) {
        Ok(i) => i,
        Err(_e) => "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>404</title></head><body><h1>404</h1><p>Page not found</p></body></html>".as_bytes().to_vec(),
    };
    let body = Body::from(contents.clone());
    Ok::<_,Error>(Response::builder().status(status).header("Content-Type", _type).header("content-length", contents.len()).body(body).unwrap())
}