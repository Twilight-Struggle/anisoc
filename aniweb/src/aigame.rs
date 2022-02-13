use anicore::game;
use anicore::Act;
use anicore::Agent;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct AiGame<T: Agent> {
    game_ins: game::Game,
    ai_agent: T,
    web_turn: game::Player,
    game_end: bool,
}

impl<T: Agent> AiGame<T> {
    pub fn setup(ai_agent: T) -> Self {
        let mut game_ins = game::Game::setup();
        let web_turn = if rand::thread_rng().gen_range(0..=1) == 0 {
            // web_playerが先手なら
            game::Player::Attack
        } else {
            game_ins.action_parse(Some(ai_agent.action(&game_ins)));
            game::Player::Defence
        };
        AiGame {
            game_ins,
            ai_agent,
            web_turn,
            game_end: false,
        }
    }
    pub fn action(&mut self, action_in: Act) -> (String, Vec<Vec<Option<String>>>) {
        if self.game_end {
            return ("Game End".to_string(), vec![vec![]]);
        }
        let action: Option<Act>;
        // tracing::debug!("legal moves: {:?}", game_ins.legal_moves());
        if self.game_ins.legal_moves().is_empty() {
            action = None;
        } else {
            action = Some(action_in);
        }
        let (success, winner) = self.game_ins.action_parse(action);
        if !success {
            return (
                "Invalid Action".to_string(),
                self.game_ins.board_to_string(),
            );
        }
        if let Some(player) = winner {
            self.game_end = true;
            if player == self.web_turn {
                return ("You win!".to_string(), self.game_ins.board_to_string());
            } else {
                return ("You Lose!".to_string(), self.game_ins.board_to_string());
            }
        }
        // こっからAI_agent
        let action: Option<Act>;
        if self.game_ins.legal_moves().is_empty() {
            action = None;
        } else {
            action = Some(self.ai_agent.action(&self.game_ins));
        }
        let (_, winner) = self.game_ins.action_parse(action); //　絶対成功
        if let Some(player) = winner {
            self.game_end = true;
            self.game_ins.next_turn();
            if player == self.web_turn {
                return ("You win!".to_string(), self.game_ins.board_to_string());
            } else {
                return ("You lose!".to_string(), self.game_ins.board_to_string());
            }
        }
        (
            "Game continues".to_string(),
            self.game_ins.board_to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anicore::randai::Randai;
    #[test]
    fn ai_game_works() {
        for _ in 0..50 {
            let opponent = Randai {};
            let mut aigame = AiGame::setup(opponent);
            let tester = Randai {};
            loop {
                let act = tester.action(&aigame.game_ins);
                let (status, _) = aigame.action(act);
                println!("{}", status);
                if status != *"Game continues" {
                    break;
                }
            }
        }
    }
}
