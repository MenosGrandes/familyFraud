use crate::db::data_base_manipulate::*;
use crate::{
    db::*,
    questions::question::{Answer, Question, Questions},
};
use json_db::json_db_manipulator::JsonDB;
use std::fs::{self, File};

#[test]
fn get_db_connection() {
    let file_name = "db_test.json";
    match fs::remove_file(file_name) {
        Err(why) => panic!("{:?}", why),
        Ok(value) => value,
    }
    let _file = File::create(file_name);
    let mut a = JsonDB::new(&file_name);

    let questions = {
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

        let question_2 = Question::new(
            "Some weird question 2?",
            vec![
                Answer::new(10, "1"),
                Answer::new(12, "2"),
                Answer::new(13, "3"),
                Answer::new(14, "4"),
                Answer::new(15, "5"),
            ],
            1,
        );
        let questions = Questions::new(vec![question, question_2]);
        std::fs::write(file_name, serde_json::to_string_pretty(&questions).unwrap()).unwrap();
        questions
    };
    let _res : DBStatus = match a.connect() {
        Err(why) => panic!("{:?}", why),
        Ok(status) => match status {
            Some(DBStatus::Opened) => DBStatus::Opened,
            Some(DBStatus::Closed) => DBStatus::Closed,
            Some(DBStatus::Other) => DBStatus::Other,
            None => DBStatus::Other,
        },
    };
    let questions_read = match a.read() {
        Err(why) => panic!("{:?}", why),
        Ok(questions) => questions,
    };
    assert_eq!(questions, questions_read);
}
