mod api;

use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

use api::{counter_api, greet, index, info, AppState, AppStateWithCounter};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = 8080;
    println!("Server running on - http://{}:{}", host, port);

    let app_state = web::Data::new(AppState {
        app_name: String::from("Actix Web"),
        version: String::from("v0.0.1"),
    });

    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            // both of the approaches to call the `counter_api` works
            // .route("/counter", web::get().to(counter_api))
            .service(counter_api)
            .app_data(app_state.clone())
            .service(info)
            .service(index)
            .service(greet)
    })
        .bind((host, port))?
        .run()
        .await
}
