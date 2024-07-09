use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::{Validate, ValidationError};

#[derive(Serialize)]
struct MyData {
    message: String,
}

async fn index() -> impl Responder {
    info!("Handling GET request for /");
    HttpResponse::Ok().json(MyData {
        message: String::from("Hello, API!"),
    })
}

#[derive(Deserialize, Validate)]
struct Info {
    #[validate(length(min = 1, message = "name cannot be empty"))]
    name: String,
}

#[derive(Deserialize)]
struct GreetQuery {
    name: String,
}

#[derive(Error, Debug)]
enum MyError {
    #[error("Invalid input: {0}")]
    ValidationError(String),
}

impl actix_web::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MyError::ValidationError(msg) => HttpResponse::BadRequest().json(MyData {
                message: msg.clone(),
            }),
        }
    }
}

async fn greet_post(info: web::Json<Info>) -> Result<impl Responder, MyError> {
    info!("Handling POST request for /greet");
    info.validate()
        .map_err(|e| MyError::ValidationError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(MyData {
        message: format!("Hello, {}!", info.name),
    }))
}

async fn greet_get(query: web::Query<GreetQuery>) -> impl Responder {
    info!("Handling GET request for /greet with query params");
    HttpResponse::Ok().json(MyData {
        message: format!("Hello, {}!", query.name),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(|| async {
                HttpResponse::NotFound().json(MyData {
                    message: String::from("The requested resource was not found."),
                })
            }))
            .route("/", web::get().to(index))
            .route("/greet", web::post().to(greet_post))
            .route("/greet", web::get().to(greet_get))
    })
    .bind("127.0.0.1:7878")?
    .run()
    .await
}
