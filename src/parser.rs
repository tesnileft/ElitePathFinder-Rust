use std::fs;
use std::io::BufReader;
use std::io::prelude::*;

fn parselogfile(path: String) {
    let log = fs::read_to_string(path).expect("Log read succesfully");
    let loglines = log.lines();
    for line in loglines {}
}
