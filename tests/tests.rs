use actix_web::{
    http::header::{self, ContentType},
    test, web::{self, Bytes}, App, body::to_bytes, guard::Patch,
};
use gsdmm_server::models::TopicModelingRequest;
use gsdmm_server::routes;
use serde::{Serialize,Deserialize};
use serde_json::{self, Value};
use std::{fs};

trait BodyTest {
    fn as_str(&self) -> &str;
}

impl BodyTest for Bytes {
    fn as_str(&self) -> &str {
        std::str::from_utf8(self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
struct Tests {
    tests: Vec<TopicModelingRequest>
}

#[actix_web::test]
async fn test_validate_body_ok() {
    let mut app = test::init_service(App::new().service(routes::validate_body)).await;
    let data = fs::read_to_string("tests/tests.json").expect("Unable to read file");
    let payloads: Tests = serde_json::from_str(&data).expect("Unable to parse");
    let curr_payload = &payloads.tests[0];
    println!("{}",curr_payload.to_string());
    let response = test::TestRequest::post()
        .uri("/validate_body")
        .append_header(("Content-Type", "application/json"))
        .set_payload(curr_payload.to_string())
        .send_request(&mut app)
        .await;
    let body = to_bytes(response.into_body()).await.unwrap();
    println!("{}",body.as_str());
    let json_body:TopicModelingRequest  = serde_json::from_str(body.as_str()).expect("Couldn't parse the json ");
    
    assert!(json_body == *curr_payload);
}

#[actix_web::test]
async fn test_validate_body_not_ok() {
    let mut app = test::init_service(App::new().service(routes::validate_body)).await;
    let data = fs::read_to_string("tests/tests.json").expect("Unable to read file");
    let payloads: Tests = serde_json::from_str(&data).expect("Unable to parse");
    let curr_payload = &payloads.tests[1];
    let response = test::TestRequest::post()
        .uri("/validate_body")
        .append_header(("Content-Type", "application/json"))
        .set_payload(curr_payload.to_string())
        .send_request(&mut app)
        .await;
    let body = to_bytes(response.into_body()).await.unwrap();
    let json_body:TopicModelingRequest  = serde_json::from_str(body.as_str()).expect("Couldn't parse the json ");
    assert!(json_body == *curr_payload);
}




