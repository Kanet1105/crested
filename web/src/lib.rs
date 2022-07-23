#[allow(unused)]
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use actix_web::{get, post};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use actix_web::middleware;

use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

#[get("/health_check")]
async fn health_check(request: HttpRequest) -> impl Responder {
    dbg!(request);
    HttpResponse::Ok().finish()
}

fn load_rustls_config(base_path: &PathBuf) -> ServerConfig {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load key/cert files.
    let cert_path = base_path.join("cert.pem");
    let cert_file = &mut BufReader::new(File::open(cert_path).unwrap());
    let key_path = base_path.join("key.pem");
    let key_file = &mut BufReader::new(File::open(key_path).unwrap());

    // convert files to key/cert objects.
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit when no key exists.
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}

pub fn run_tls() -> Result<Server, std::io::Error> {
    // load the TLS configuration.
    let base_path = std::env::current_dir()?
        .join("certs");
    let config = load_rustls_config(&base_path);

    // the web server.
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(health_check)
    })
    .bind_rustls("127.0.0.1:8443", config)?
    .run();
    Ok(server)
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(health_check)
    })
    .bind("127.0.0.1:8080")?
    .run();
    Ok(server)
}