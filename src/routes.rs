use actix_web::{post, web::{Json}, Responder, HttpResponse};
use validator::Validate;
use crate::models;
use crate::utils;

#[post("/validate_body")]
pub async fn validate_body(body: Json<models::TopicModelingRequest>) -> impl Responder {
    
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => HttpResponse::Ok().json(body),
        Err(err) => HttpResponse::BadRequest().json(err) 
    }
}
    
#[post("/model_endpoint")]
pub async fn model_endpoint(mut body: Json<models::TopicModelingRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            println!("Data validated, beginning training");
            let json_body = &mut body.0;
            let res = utils::train_gsdmm(&json_body);
            HttpResponse::Ok().json(res.unwrap())
        },
        Err(err) => HttpResponse::BadRequest().json(err)
    }
}
