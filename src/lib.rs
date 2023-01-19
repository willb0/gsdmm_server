use std::fmt;
use crate::models::TopicModelingRequest;
use crate::models::TrainingError;


pub mod routes;
pub mod models;
pub mod utils;

impl fmt::Display for TrainingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "training the GSDMM model failed")
    }
}

pub trait ToString {
    fn to_json_string(&self) -> String;
}


impl ToString for TopicModelingRequest {
    fn to_json_string(&self) -> String{
        return serde_json::to_string(&self).expect("Could not parse object");
    }
}

impl fmt::Display for TopicModelingRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?} {} {} {}",self.documents,self.vocab,self.max_clusters,self.alpha,self.beta)
    }
}