use crate::{
    db::*,
    questions::question::{Answer, Question},
};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
#[test]
fn get_db_connection() {
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
    let serialized = serde_json::to_string(&question).unwrap();
    print!("{serialized}");
    let path = Path::new("db.json");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let _res = serde_json::to_writer(&mut file, &serialized);
}
