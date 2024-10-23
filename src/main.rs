use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Result, Responder, HttpResponse};
use std::env;
use std::path::PathBuf;

#[get("/")]
async fn index() -> Result<NamedFile> {
    let root = env::current_dir().unwrap();
    let abs_path = format!("{}/app/index.html", root.to_str().unwrap());
    let path: PathBuf = PathBuf::from(abs_path);
    Ok(NamedFile::open(path)?)
}

#[get("/{id}")]
async fn get_id(path: web::Path<String>) -> impl Responder { 
    let friend = path.into_inner();
    println!("{}", friend);
    HttpResponse::Ok().body(friend)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on 127.0.0.1:3000");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(get_id)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
