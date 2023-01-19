extern crate gsdmm;

use gsdmm::GSDMM;
use crate::models::TopicModelingRequest;
use std::collections::HashSet;
use std::iter::FromIterator;
use crate::models::TrainingError;



pub fn train_gsdmm(data: &TopicModelingRequest) -> Result<Vec<usize>,TrainingError> {
    let vocab = HashSet::from_iter(data.vocab.iter().cloned());
    let documents = data.documents.clone();
    let mut model = GSDMM::new(data.alpha, data.beta,data.max_clusters,100,vocab,documents);
    model.fit();
    Ok(model.labels)
}

/*
fn raw_text_to_tokens(documents: Vec<String>) -> Vec<Vec<String>>{ 
    let mut tokens: Vec<Vec<String>> = Vec::new();
    for document in &mut documents.into_iter() {
        tokens.push(Vec::new());
        let last = tokens.last().expect("Couldnt get the last vec");
        let string:String = "hello".to_owned();
        last.push(string);
    }
    return tokens;
}
*/

