pub mod game;
pub mod randai;

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn random_battle() {
        for _ in 0..50 {
            let agent1 = randai::Randai {};
            let agent2 = randai::Randai {};
            game::Game::random_game(agent1, agent2);
        }
    }
}
