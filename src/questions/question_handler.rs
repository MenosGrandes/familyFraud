use super::question::Question;
use crate::questions::traits::traits_collection::QuestionHandlerTrait;

#[derive(Debug)]
pub struct QuestionHandler {
    questions: Vec<Question>,
}
impl QuestionHandler {
    pub fn new(quesitons: Vec<Question>) -> Self {
        QuestionHandler {
            questions: quesitons,
        }
    }
}
impl QuestionHandlerTrait for QuestionHandler {
    fn add_question(&mut self, q: Question) {
        self.questions.push(q);
    }
    fn get_question(&self) -> &Question {
        &self.questions[0]
    }
}
