pub mod db;
pub mod ff_core;
pub mod questions;

use crate::{
    db::*,
    questions::question::{Answer, Question},
};
use std::io::prelude::*;
use std::path::Path;
use std::{fs::File, io::BufWriter};
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
#[path = "../test/test.rs"]
pub mod test;

fn main() {
    let question = Question::new(
        "Some weird question?",
        vec![
            Answer::new(10, "1"),
            Answer::new(12, "2"),
            Answer::new(13, "3"),
            Answer::new(14, "4"),
            Answer::new(15, "5"),
        ],
        1,
    );
    let _serialized = serde_json::to_string(&question).unwrap();
}
