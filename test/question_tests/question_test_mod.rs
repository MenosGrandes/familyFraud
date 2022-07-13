use crate::questions::{
    question::{Answer, Question},
    question_handler::QuestionHandler,
    traits::traits_collection::QuestionHandlerTrait,
};

#[test]
fn add_question() {
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
    let question_handler = QuestionHandler::new(vec![question]);
    let question = question_handler.get_question();
    assert_eq!(question.answers.len(), 5);
    assert_eq!(question.answers[0].answer, "1");
    assert_eq!(question.answers[1].answer, "2");
    assert_eq!(question.answers[2].answer, "3");
    assert_eq!(question.answers[3].answer, "4");
    assert_eq!(question.answers[4].answer, "5");

    assert_eq!(question.answers[0].points, 10);
    assert_eq!(question.answers[1].points, 12);
    assert_eq!(question.answers[2].points, 13);
    assert_eq!(question.answers[3].points, 14);
    assert_eq!(question.answers[4].points, 15);

    assert_eq!(question.question, "Some weird question?");
}
