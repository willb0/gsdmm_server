use serde::{Serialize, Deserialize};
use validator::Validate;



#[derive(Serialize, Deserialize, Validate,PartialEq,PartialOrd,Debug)]
pub struct TopicModelingRequest {
    pub documents: Vec<Vec<String>>,
    pub vocab: Vec<String>,
    pub max_clusters: usize,
    pub alpha: f64,
    pub beta: f64,
}

#[derive(Debug,Clone)]
pub struct TrainingError;

