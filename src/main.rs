use crate::ff_core::family::{Family, FamilyMember};
use crate::ff_core::game::Game;
use crate::ff_core::game_host::GameHost;
use crate::ff_core::point_manipulator::FamilyPointManipulator;
use crate::ff_core::traits::traits_collection::PointManipulator;
use crate::questions::question::{Answer, Question};
use crate::questions::question_handler::QuestionHandler;
pub mod ff_core;
pub mod questions;

#[cfg(test)]
#[path = "../test/test.rs"]
pub mod test;


fn main() {
}
