use actix_web::{get, post, web, Responder, HttpResponse};
use actix_web::http::header::LOCATION;
use serde::Deserialize;
use fred::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use shuttle_runtime::SecretStore;
use shuttle_actix_web::ShuttleActixWeb;

struct AppState {
    redis_client: RedisClient
}

#[derive(Deserialize)]
struct CreateRequest {
    url: String
}

fn create_shortened_url() -> String {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();
    rand_string
}

#[get("/")]
async fn index() -> impl Responder {
    match std::fs::read_to_string("pages/index.html") {
        Ok(contents) => HttpResponse::Ok().content_type("text/html").body(contents),
        Err(_) => HttpResponse::InternalServerError().body("Error reading index.html"),
    }
}

#[get("/{id}")]
async fn get_url(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let url_id = path.into_inner();

    match data.redis_client.get::<Option<String>, String>(url_id).await {
        Ok(Some(route)) => {
            println!("HTTP 302 FOUND: Redirecting to {}", route);
            HttpResponse::Found()
                .insert_header((LOCATION, route))
                .finish()
        },
        Ok(None) => {
            println!("HTTP 404 NOT_FOUND: Redirecting to https://http.cat/404");
            HttpResponse::Found()
                .insert_header((LOCATION, "https://http.cat/404"))
                .finish()
        },
        Err(e) => {
            println!("HTTP 505 INTERNAL SERVER ERROR: Error getting url. {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/create")]
async fn create_url(body: web::Json<CreateRequest>, data: web::Data<AppState>) -> impl Responder { 
    let real_url = body.url.clone();
    let shortened_url = create_shortened_url();
    let expiration_seconds = 300;

    match data.redis_client.set::<(), &str, String>(&shortened_url, real_url, Some(Expiration::EX(expiration_seconds)), Some(SetOptions::NX), false).await {
        Ok(_) => {
            println!("HTTP 200 OK: Successfully created {}", &shortened_url);
            HttpResponse::Ok().body(shortened_url)
        },
        Err(e) => {
            println!("HTTP 505 INTERNAL SERVER ERROR: Error creating shortened url, {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[shuttle_runtime::main]
async fn actix_web_service(
    #[shuttle_runtime::Secrets] secret_store: SecretStore
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    let username = secret_store.get("REDIS_USERNAME").expect("REDIS_USERNAME must be set.");
    let password = secret_store.get("REDIS_PASSWORD").expect("REDIS_PASSWORD must be set.");
    let host = secret_store.get("REDIS_HOST").expect("REDIS_HOST must be set.");
    let port = secret_store.get("REDIS_PORT").expect("REDIS_PORT must be set.");
    let redis_url = format!("rediss://{}:{}@{}:{}", username, password, host, port);

    let config = RedisConfig::from_url(&redis_url).unwrap();
    let client = Builder::from_config(config).build().unwrap();

    client.init().await.map_err(|err| -> anyhow::Error {
        err.into()
    })?;

    let factory = move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(web::Data::new(AppState { redis_client: client.clone() }))
            .service(index)
            .service(get_url)
            .service(create_url);
        };

    Ok(factory.into())
}
