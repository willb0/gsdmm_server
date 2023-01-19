use actix_web::{App, HttpServer};

use gsdmm_server::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server running on http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .service(routes::validate_body)
            .service(routes::model_endpoint)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
