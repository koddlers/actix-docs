use actix_web::{get, App, HttpResponse, HttpServer, Responder};


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(".::Welcome to Actix Web::.")
}

#[get("/greet")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(greet)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
