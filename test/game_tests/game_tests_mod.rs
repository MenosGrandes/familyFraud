use crate::ff_core::family::{Family, FamilyMember};
use crate::ff_core::game::Game;
use crate::ff_core::game_host::GameHost;
use crate::ff_core::point_manipulator::FamilyPointManipulator;
use crate::ff_core::traits::traits_collection::PointManipulator;
use crate::questions::question::{Answer, Question};
use crate::questions::question_handler::QuestionHandler;
#[test]
fn game_test() {
    let mut family_1 = Family::with_name("Niewiadowie");
    family_1.add_member(FamilyMember::new("Grzegorz"), 0);
    family_1.add_member(FamilyMember::new("Wacek"), 1);
    family_1.add_member(FamilyMember::new("Placek"), 2);
    family_1.add_member(FamilyMember::new("Jacek"), 3);
    family_1.add_member(FamilyMember::new("Anna"), 4);

    let mut family_2 = Family::with_name("Bakowscy");
    family_2.add_member(FamilyMember::new("Ania"), 0);
    family_2.add_member(FamilyMember::new("Baba"), 1);
    family_2.add_member(FamilyMember::new("Blabl"), 2);
    family_2.add_member(FamilyMember::new("Random"), 3);
    family_2.add_member(FamilyMember::new("Jaroslaw"), 4);

    let question = Question::new(
        "Co to jest to i to?",
        vec![
            Answer::new(10, "1"),
            Answer::new(10, "2"),
            Answer::new(10, "3"),
            Answer::new(10, "4"),
            Answer::new(10, "5"),
        ],
        1,
    );

    let question_handler = QuestionHandler::new(vec![question]);
    let game_host = GameHost::new("ZdzislawKrecina", question_handler);
    let mut game = Game::new([family_1, family_2], game_host);
    let point_manipulator = FamilyPointManipulator;
    let current_family: &mut Family = game.get_family(1);
    point_manipulator.add_points(current_family, 100);

    let current_family: &mut Family = game.get_family(0);
    point_manipulator.add_points(current_family, 100);

    let current_family: &mut Family = game.get_family(1);
    point_manipulator.add_points(current_family, 100);
    point_manipulator.add_points(current_family, 100);

    let _points = current_family.get_points();

    let game_host = game.get_game_host();
    let q = game_host.get_question();
    let _answer = &q.question;
}
