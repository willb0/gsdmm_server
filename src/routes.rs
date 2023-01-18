use actix_web::{Error, post, web::{self,Json}, Responder, HttpResponse};
use validator::Validate;
use crate::models::TopicModelingRequest;

#[post("/validate_body")]
pub async fn validate_body(body: Json<TopicModelingRequest>) -> impl Responder {
    
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => HttpResponse::Ok().json(body),
        Err(err) => HttpResponse::BadRequest().json(err) 
    }
}
