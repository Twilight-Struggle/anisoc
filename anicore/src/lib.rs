const HEIGHT: usize = 5;
const WIDTH: usize = 3;

enum Piece {
    Saru,
    Usagi,
    Risu,
    Oyasaru,
    Ball,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Player {
    Front,
    Back,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Act {
    from: usize,
    to: usize,
    kickto: usize,
}

pub struct Game {
    turn: Player,
    board: Vec<Vec<Option<Piece>>>,
}

impl Game {
    pub fn new() -> Self {
        let mut board = vec![vec![None; WIDTH]; HEIGHT];
        board[1][1] = Some(Piece::Saru);
        Game {
            turn: Player::Front,
            board,
        }
    }
    // fn reset();
    fn action();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
