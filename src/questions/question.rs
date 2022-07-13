#[derive(Debug)]
pub struct Answer {
    pub answer: String,
    pub points: u8,
}
impl Answer {
    pub fn new(points: u8, answer: &str) -> Self {
        Answer {
            points,
            answer: answer.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Question {
    pub question: String,
    pub answers: Vec<Answer>,
    pub multiplayer: i8,
}

impl Question {
    pub fn new(question: &str, answers: Vec<Answer>, multiplayer: i8) -> Self {
        Question {
            question: question.to_string(),
            answers,
            multiplayer,
        }
    }
}
