use std::sync::Mutex;
use actix_web::{get, web, HttpResponse, Responder};

pub struct AppStateWithCounter {
    pub counter: Mutex<i32>,
}

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