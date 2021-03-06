pub mod aigrpc;
pub mod game;
pub mod randai;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct Act {
    from: (isize, isize),
    to: (isize, isize),
    kickto: Option<(isize, isize)>,
}

pub trait Agent {
    // プレイできる手がない場合は勝手にルール側ですすめるためNoneの必要なし
    fn action(&self, game: &game::Game) -> Act;
}
