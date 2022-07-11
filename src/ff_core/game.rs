use super::family::FamilyMember;
use super::host::GameHost;
use crate::ff_core::family::Family;

#[derive(Debug)]
struct Game {
    families: [Family; 2],
    host: GameHost,
}
impl Game {
    fn new() {}
}
