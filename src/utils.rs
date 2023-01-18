extern crate gsdmm;

use gsdmm::GSDMM;
use crate::models::TopicModelingRequest;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::models::TrainingError;



pub fn train_gsdmm(data: &TopicModelingRequest) -> Result<Vec<usize>,TrainingError> {
    let vocab = HashSet::from_iter(data.vocab.iter().cloned());
    let documents = data.documents.clone();
    let mut model = GSDMM::new(data.alpha, data.beta,data.max_clusters,100,vocab,documents);
    model.fit();
    Ok(model.labels)
}

fn lines_from_file(filename: &str) -> Vec<String> {
    let error_msg = format!("Could not read file {}!", filename);
    let file = File::open(filename).expect(&error_msg);
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line!"))
        .collect()
}

fn row_has_nan(row: &Vec<(usize, &f64)>, doc: &String) -> bool {
    for entry in row {
        if entry.1.is_nan() {
            println!("Cluster: {:?} has NaN score for document {:?}", entry, doc);
            return true;
        }
    }
    return false;
}
