use actix_files::NamedFile;
use actix_web::{get, App, HttpServer, Result};
use std::env;
use std::path::PathBuf;

#[get("/")]
async fn index() -> Result<NamedFile> {
    let root = env::current_dir().unwrap();
    let abs_path = format!("{}/app/index.html", root.to_str().unwrap());
    let path: PathBuf = PathBuf::from(abs_path);
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on 127.0.0.1:3000");

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
