use serde::{Deserialize, Serialize};
use std::fmt;
use std::marker::Copy;
use validator::Validate;
use crate::models::TopicModelingRequest;


pub mod routes;
pub mod models;

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