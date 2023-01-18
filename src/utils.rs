extern crate docopt;
extern crate rustc_serialize;
extern crate gsdmm;

use std::fs::File;
use std::io::Write;
use std::io::{BufRead,BufReader};
use std::collections::HashSet;


fn lines_from_file(filename: &str) -> Vec<String>
    {
        let error_msg = format!("Could not read file {}!", filename);
        let file = File::open(filename).expect(&error_msg);
        let buf = BufReader::new(file);
        buf.lines().map(|l| l.expect("Could not parse line!")).collect()
    }

fn row_has_nan(row:&Vec<(usize, &f64)>, doc:&String) -> bool {
    for entry in row {
        if entry.1.is_nan() {
            println!("Cluster: {:?} has NaN score for document {:?}", entry, doc);
            return true
        }
    }
    return false;
}