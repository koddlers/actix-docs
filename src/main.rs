mod api;

use api::{counter_api, greet, index};

use actix_web::{web, App, HttpServer};
use api::AppStateWithCounter;
use std::sync::Mutex;

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
            // both of the approaches to call the `counter_api` works
            // .route("/counter", web::get().to(counter_api))
            .service(counter_api)

            .service(index)
            .service(greet)
    })
        .bind((host, port))?
        .run()
        .await
}
