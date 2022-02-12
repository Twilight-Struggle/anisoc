#![allow(clippy::needless_range_loop)]

mod game;
pub mod randai;

use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Act {
    from: (isize, isize),
    to: (isize, isize),
    kickto: Option<(isize, isize)>,
}

pub trait Agent {
    // プレイできる手がない場合は勝手にルール側ですすめるためNoneの必要なし
    fn action(&self, game: &game::Game) -> Act;
}

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
    #[test]
    fn random_battle() {
        for _ in 0..50 {
            let agent1 = randai::Randai {};
            let agent2 = randai::Randai {};
            game::Game::agent_game(agent1, agent2);
        }
    }
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::EnvFilter;
    fn init_subscriber() {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("Debug"));
        let format = tracing_subscriber::fmt::format().pretty();
        tracing_subscriber::fmt()
            .with_writer(std::io::stdout)
            .with_env_filter(env_filter)
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .event_format(format)
            .init();
    }
    #[test]
    fn integ_test() {
        init_subscriber();
        let agent1 = randai::Randai {};
        let agent2 = randai::Randai {};
        game::Game::agent_game(agent1, agent2);
    }
}
