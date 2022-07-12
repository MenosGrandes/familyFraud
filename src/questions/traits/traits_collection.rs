use crate::questions::question::Question;
pub trait QuestionHandlerTrait {
    fn add_question(&mut self, q : Question);
    fn get_question(&self) -> &Question;
}
