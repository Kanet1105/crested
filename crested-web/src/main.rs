#[macro_use] extern crate rocket;
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
use rocket::http::ContentType;

#[get("/")]
async fn index() -> (ContentType, &'static str) {
    let index_html = include_str!("C:\\Projects\\crested\\crested-wasm\\build\\index.html");
    (ContentType::HTML, index_html)
}

#[get("/<file..>")]
async fn get_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("C:\\Projects\\crested\\crested-wasm\\build").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/build", routes![get_file])
}