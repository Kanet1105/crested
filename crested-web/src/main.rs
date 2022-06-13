use actix_web::{get, middleware, App, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web::http::StatusCode;

#[get("/")]
async fn index(req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("./static/webcam.html")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(index)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
