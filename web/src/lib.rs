#[allow(unused)]
use actix_web::{get, post};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;

#[get("/health_check")]
async fn health_check(request: HttpRequest) -> impl Responder {
    dbg!(request);
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check)
        })
        .bind(("127.0.0.1", 8080))?
        .run();
    Ok(server)
}