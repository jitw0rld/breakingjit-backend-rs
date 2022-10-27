use actix_web::{middleware, web, App, HttpRequest, HttpServer, HttpResponse};
use rand::prelude::IteratorRandom;
use std::fs;

async fn get_image(req: HttpRequest) -> HttpResponse {
    let mut rng = rand::thread_rng();
    let files = fs::read_dir("images").unwrap();
    let file = files.choose(&mut rng).unwrap().unwrap();

    let file_stream = actix_files::NamedFile::open_async(file.path()).await.unwrap();

    file_stream.into_response(&req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/breakingbad").to(get_image))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}