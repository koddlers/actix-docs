use actix_web::{get, post, web, HttpResponse, Responder};
use crate::app_state::{AppState, AppStateWithCounter};

#[get("/counter")]
pub async fn counter_api(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Request number: {counter}")
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(".::Welcome to Actix Web::.")
}

#[get("/greet")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/echo")]
async fn echo(message: String) -> impl Responder {
    HttpResponse::Ok().body(message)
}

#[get("/info")]
async fn info(data: web::Data<AppState>) -> String {
    let app = &data.app_name;
    let version = &data.version;

    format!("App name: {app}, version: {version}")
}