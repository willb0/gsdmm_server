use actix_web::{App, test, HttpServer, http::header,web};

mod routes;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server running on http://localhost:8080");
    HttpServer::new(|| App::new()
        .service(
            routes::validate_body
        )
    )
    .bind(("localhost",8080))?
    .run()
    .await
}