use serde::{Serialize, Deserialize};
use validator::Validate;



#[derive(Serialize, Deserialize, Validate,PartialEq,PartialOrd)]
pub struct TopicModelingRequest {
    pub documents: Vec<String>,
    pub vocab: Vec<String>,
    pub max_clusters: u32,
    pub alpha: f32,
    pub beta: f32,
}

