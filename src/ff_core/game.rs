use super::game_host::GameHost;
use crate::ff_core::family::Family;

#[derive(Debug)]
pub struct Game {
    pub families: [Family; 2],
    pub game_host: GameHost,
}
impl Game {
    pub fn new(families: [Family; 2], game_host: GameHost) -> Self {
        Game {
            families,
            game_host,
        }
    }
    pub fn get_family(&mut self, index: usize) -> &mut Family {
        &mut self.families[index]
    }
    pub fn get_game_host(&mut self) -> &mut GameHost {
        &mut self.game_host
    }
}
