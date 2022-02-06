const HEIGHT: usize = 5;
const WIDTH: usize = 3;
const SARU_KICK: [[isize; 2]; 6] = [[1, 1], [1, -1], [-1, 0], [2, 2], [2, -2], [-2, 0]];
const USAGI_KICK: [[isize; 2]; 10] = [
    [1, 0],
    [2, 0],
    [0, 1],
    [0, 2],
    [0, -1],
    [0, -2],
    [1, 1],
    [2, 2],
    [1, -1],
    [2, -2],
];
const RISU_KICK: [[isize; 2]; 8] = [
    [1, 0],
    [2, 0],
    [-1, 0],
    [-2, 0],
    [0, 1],
    [0, 2],
    [0, -1],
    [0, -2],
];

#[derive(Debug, Clone, PartialEq, Eq)]
enum PieceKind {
    Saru(Vec<isize>, Vec<Vec<isize>>),
    Usagi(Vec<isize>, Vec<Vec<isize>>),
    Risu(Vec<isize>, Vec<Vec<isize>>),
    Oyasaru(Vec<isize>, Vec<Vec<isize>>),
    Ball,
}
impl PieceKind {
    fn get(&self) -> Option<(&Vec<isize>, &Vec<Vec<isize>>)> {
        match self {
            PieceKind::Saru(idou, kick) => Some((idou, kick)),
            PieceKind::Usagi(idou, kick) => Some((idou, kick)),
            PieceKind::Risu(idou, kick) => Some((idou, kick)),
            PieceKind::Oyasaru(idou, kick) => Some((idou, kick)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Player {
    Attack,
    Defence,
    Neutral,
}

#[derive(Debug, Clone)]
struct Piece {
    player: Player,
    piecekind: PieceKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Act {
    from: isize,
    to: isize,
    kickto: Option<isize>,
}

// type Board = Vec<Vec<Option<Piece>>>;
type Board = [[Option<Piece>; WIDTH]; HEIGHT];
pub struct Game {
    turn: Player,
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        let mut board: Board = [[None; WIDTH]; HEIGHT];
        let saru_kick = vec![
            vec![1, 1],
            vec![1, -1],
            vec![-1, 0],
            vec![2, 2],
            vec![2, -2],
            vec![-2, 0],
        ];
        let saru_piece = PieceKind::Saru(vec![-1, 1], saru_kick);
        board[1][1] = Some(Piece {
            player: Player::Attack,
            piecekind: saru_piece,
        });
        Game {
            turn: Player::Attack,
            board,
        }
    }
    fn next_turn(&mut self) {
        match self.turn {
            Player::Attack => self.turn = Player::Defence,
            Player::Defence => self.turn = Player::Attack,
        };
        for row in self.board {
            row.reverse();
        }
        self.board.reverse();
    }
    fn piece_can_move(
        &self,
        hei: isize,
        wid: isize,
        piece: &Piece,
    ) -> (Option<Vec<(isize, isize)>>, Option<(isize, isize)>) {
        let mut legal: Vec<(isize, isize)> = vec![];
        let mut ball_legal: Option<(isize, isize)> = None;
        let (idou, kick) = piece.piecekind.get().unwrap();
        for x in hei + idou[0]..=hei + idou[1] {
            for y in wid + idou[0]..=wid + idou[1] {
                if (0 <= x && x <= (HEIGHT as isize - 1)) && (0 <= y && y <= WIDTH as isize) {
                    match self.board[x as usize][y as usize] {
                        None => legal.push((x, y)),
                        Some(ball) if ball.piecekind == PieceKind::Ball => {
                            ball_legal = Some((x, y))
                        }
                    }
                }
            }
        }
        if legal.len() == 0 {
            (None, ball_legal)
        } else {
            (Some(legal), ball_legal)
        }
    }
    fn piece_legal_move(&self, hei: isize, wid: isize, piece: &Piece) -> Vec<Act> {}
    fn legal_moves(&self) -> Vec<Act> {
        let mut ret: Vec<Act> = vec![];
        for (hei, row) in self.board.iter().enumerate() {
            for (wid, col) in row.iter().enumerate() {
                match col {
                    Some(piece) => (),
                    None => (),
                }
            }
        }
        ret
    }
    // fn reset();
    fn action(&self, act: Act) {
        // legal move一覧を取得
        // actがlegal moveであるか確認　→　違ったらreturn

        // 正しいactであれば移動を実行
        // もちろんキックも同時に実行　ここで勝利判定

        // 親猿にグレードアップ
        // turn変更

        // 成功をreturn
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
