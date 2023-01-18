use actix_web::{http::header, test, web, App, HttpServer};

use gsdmm_server::models;
use gsdmm_server::routes;
use gsdmm_server::utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server running on http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .service(routes::validate_body)
            .service(routes::model_endpoint)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
