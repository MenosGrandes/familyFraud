use crate::questions::question::Question;
use crate::questions::question_handler::QuestionHandler;
use crate::questions::traits::traits_collection::QuestionHandlerTrait;
#[derive(Debug)]
pub struct GameHost {
    name: String,
    question_handler: QuestionHandler,
}
impl GameHost {
    pub fn new(name: &str, question_handler: QuestionHandler) -> Self {
        GameHost {
            name: name.to_string(),
            question_handler
        }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_question(&self) -> &Question {
        self.question_handler.get_question()
    }
}
