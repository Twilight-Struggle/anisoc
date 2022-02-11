use crate::game::Game;
use crate::Act;
use crate::Agent;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Randai {}

impl Agent for Randai {
    fn action(&self, game: &Game) -> Act {
        let legalmoves = game.legal_moves();
        let mut not_lose = vec![];
        for (i, action) in legalmoves.iter().enumerate() {
            if let Some(kick) = action.kickto {
                if kick.0 == 5 {
                    return legalmoves[i];
                } else if kick.0 != -1 {
                    not_lose.push(i);
                }
            } else {
                not_lose.push(i);
            }
        }
        let ret_index = not_lose[rand::thread_rng().gen_range(0..not_lose.len())];
        legalmoves[ret_index]
    }
}
