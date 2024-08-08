use std::sync::Mutex;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};


struct AppStateWithCounter {
    counter: Mutex<i32>,
}

async fn counter_api(data: web::Data<AppStateWithCounter>) -> String {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = 8080;
    println!("Server running on - http://{}:{}", host, port);

    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/counter", web::get().to(counter_api))
            .service(index)
            .service(greet)
    })
        .bind((host, port))?
        .run()
        .await
}
