use crate::Act;
use crate::Agent;
use rand::Rng;

const HEIGHT: isize = 5;
const WIDTH: isize = 3;
const SARU_KICK: [[isize; 2]; 6] = [[1, 1], [1, -1], [-1, 0], [2, 2], [2, -2], [-2, 0]];

#[derive(Debug, Clone, PartialEq, Eq)]
enum PieceKind {
    Saru([isize; 2], Vec<[isize; 2]>),
    Usagi([isize; 2], Vec<[isize; 2]>),
    Risu([isize; 2], Vec<[isize; 2]>),
    Oyasaru([isize; 2], Vec<[isize; 2]>),
    Ball,
}
impl PieceKind {
    fn get(&self) -> Option<(&[isize; 2], &Vec<[isize; 2]>)> {
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
pub enum Player {
    Attack,
    Defence,
    Neutral,
}
impl Player {
    fn against(&self) -> Self {
        if self == &Self::Attack {
            Self::Defence
        } else {
            Self::Attack
        }
    }
}

#[derive(Debug, Clone)]
struct Piece {
    player: Player,
    piecekind: PieceKind,
}

type Board = Vec<Vec<Option<Piece>>>;
#[derive(Debug, Clone)]
pub struct Game {
    turn: Player,
    board: Board,
}

impl Game {
    pub fn setup() -> Self {
        let mut board = vec![vec![None; WIDTH as usize]; HEIGHT as usize];
        let saru_kick = SARU_KICK.iter().copied().collect();
        let saru_piece = PieceKind::Saru([-1, 1], saru_kick);
        let usagi_kick = vec![
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
        let usagi_piece = PieceKind::Usagi([-1, 1], usagi_kick);
        let risu_kick = vec![
            [1, 0],
            [2, 0],
            [-1, 0],
            [-2, 0],
            [0, 1],
            [0, 2],
            [0, -1],
            [0, -2],
        ];
        let risu_piece = PieceKind::Risu([-1, 1], risu_kick);
        board[1][1] = Some(Piece {
            player: Player::Attack,
            piecekind: saru_piece.clone(),
        });
        board[0][0] = Some(Piece {
            player: Player::Attack,
            piecekind: usagi_piece.clone(),
        });
        board[0][2] = Some(Piece {
            player: Player::Attack,
            piecekind: risu_piece.clone(),
        });
        board[3][1] = Some(Piece {
            player: Player::Defence,
            piecekind: saru_piece,
        });
        board[4][0] = Some(Piece {
            player: Player::Defence,
            piecekind: risu_piece,
        });
        board[4][2] = Some(Piece {
            player: Player::Defence,
            piecekind: usagi_piece,
        });
        board[2][1] = Some(Piece {
            player: Player::Neutral,
            piecekind: PieceKind::Ball,
        });
        Game {
            turn: Player::Attack,
            board,
        }
    }
    #[allow(clippy::needless_range_loop)]
    pub fn board_to_string(&self) -> Vec<Vec<Option<String>>> {
        let mut ret = vec![vec![None; WIDTH as usize]; HEIGHT as usize];
        for i in 0..HEIGHT as usize {
            for j in 0..WIDTH as usize {
                match &self.board[i][j] {
                    Some(piece) if piece.player == self.turn => match piece.piecekind {
                        PieceKind::Oyasaru(_, _) => ret[i][j] = Some("O_A".to_string()),
                        PieceKind::Saru(_, _) => ret[i][j] = Some("S_A".to_string()),
                        PieceKind::Risu(_, _) => ret[i][j] = Some("R_A".to_string()),
                        PieceKind::Usagi(_, _) => ret[i][j] = Some("U_A".to_string()),
                        _ => (),
                    },
                    Some(piece) => match piece.piecekind {
                        PieceKind::Ball => ret[i][j] = Some("B_N".to_string()),
                        PieceKind::Oyasaru(_, _) => ret[i][j] = Some("O_D".to_string()),
                        PieceKind::Saru(_, _) => ret[i][j] = Some("S_D".to_string()),
                        PieceKind::Risu(_, _) => ret[i][j] = Some("R_D".to_string()),
                        PieceKind::Usagi(_, _) => ret[i][j] = Some("U_D".to_string()),
                    },
                    None => (),
                }
            }
        }
        ret
    }
    #[allow(clippy::needless_range_loop)]
    pub fn board_to_tensor(&self) -> Vec<Vec<Vec<f32>>> {
        let mut ret = vec![vec![vec![0.0; WIDTH as usize]; HEIGHT as usize]; 9];
        for i in 0..HEIGHT as usize {
            for j in 0..WIDTH as usize {
                match &self.board[i][j] {
                    Some(piece) if piece.player == self.turn => match piece.piecekind {
                        PieceKind::Oyasaru(_, _) => ret[3][i][j] = 1.0,
                        PieceKind::Saru(_, _) => ret[0][i][j] = 1.0,
                        PieceKind::Risu(_, _) => ret[1][i][j] = 1.0,
                        PieceKind::Usagi(_, _) => ret[2][i][j] = 1.0,
                        _ => (),
                    },
                    Some(piece) => match piece.piecekind {
                        PieceKind::Ball => ret[8][i][j] = 1.0,
                        PieceKind::Oyasaru(_, _) => ret[7][i][j] = 1.0,
                        PieceKind::Saru(_, _) => ret[4][i][j] = 1.0,
                        PieceKind::Risu(_, _) => ret[5][i][j] = 1.0,
                        PieceKind::Usagi(_, _) => ret[6][i][j] = 1.0,
                    },
                    None => (),
                }
            }
        }
        ret
    }
    pub fn next_turn(&mut self) {
        match self.turn {
            Player::Attack => self.turn = Player::Defence,
            Player::Defence => self.turn = Player::Attack,
            _ => (),
        };
        for row in &mut self.board {
            row.reverse();
        }
        self.board.reverse();
    }
    fn piece_can_move(
        &self,
        hei: isize,
        wid: isize,
        piece: &Piece,
    ) -> (Option<Vec<Act>>, Option<Act>) {
        let mut move_legal: Vec<Act> = vec![];
        let mut ball_legal: Option<Act> = None;
        let (idou, _) = piece.piecekind.get().unwrap();
        for x in hei + idou[0]..=hei + idou[1] {
            for y in wid + idou[0]..=wid + idou[1] {
                if (0..HEIGHT).contains(&x) && (0..WIDTH).contains(&y) {
                    match &self.board[x as usize][y as usize] {
                        None => move_legal.push(Act {
                            from: (hei, wid),
                            to: (x, y),
                            kickto: None,
                        }),
                        Some(ball) if ball.piecekind == PieceKind::Ball => {
                            ball_legal = Some(Act {
                                from: (hei, wid),
                                to: (x, y),
                                kickto: None,
                            })
                        }
                        _ => (),
                    }
                }
            }
        }
        if move_legal.is_empty() {
            (None, ball_legal)
        } else {
            (Some(move_legal), ball_legal)
        }
    }
    fn piece_can_kick(
        &self,
        ball_legal: &Act,
        ball_place_kick_from: (isize, isize),
        kicked_mask: Vec<(isize, isize)>,
        piece: &Piece,
    ) -> Vec<Act> {
        let (_, kickto) = piece.piecekind.get().unwrap();
        let mut kicks_legal: Vec<Act> = vec![];
        for kick in kickto {
            let (x, y) = (
                ball_place_kick_from.0 + kick[0],
                ball_place_kick_from.1 + kick[1],
            );
            if (-1 <= x && x <= HEIGHT) && (0..WIDTH).contains(&y) {
                if (x == -1 || x == 5) || ((x, y) == ball_legal.from) {
                    kicks_legal.push(Act {
                        from: ball_legal.from,
                        to: ball_legal.to,
                        kickto: Some((x, y)),
                    });
                } else if !kicked_mask.iter().any(|&kicked| kicked == (x, y)) {
                    match &self.board[x as usize][y as usize] {
                        Some(found_piece) if found_piece.player == self.turn => {
                            let mut kicked_mask_chi = kicked_mask.clone();
                            kicked_mask_chi.push((x, y));
                            kicks_legal.append(&mut self.piece_can_kick(
                                ball_legal,
                                (x, y),
                                kicked_mask_chi,
                                found_piece,
                            ));
                        }
                        None => kicks_legal.push(Act {
                            from: ball_legal.from,
                            to: ball_legal.to,
                            kickto: Some((x, y)),
                        }),
                        _ => (),
                    }
                }
            }
        }
        kicks_legal
    }
    fn piece_legal_move(&self, hei: isize, wid: isize, piece: &Piece) -> Vec<Act> {
        let (move_legal, ball_legal) = self.piece_can_move(hei, wid, piece);
        let mut legal = match move_legal {
            Some(moves) => moves,
            None => vec![],
        };
        if let Some(ball_legal) = ball_legal {
            legal.append(&mut self.piece_can_kick(
                &ball_legal,
                ball_legal.to,
                vec![ball_legal.to],
                piece,
            ))
        }
        legal
    }
    pub fn legal_moves(&self) -> Vec<Act> {
        let mut ret: Vec<Act> = vec![];
        for (hei, row) in self.board.iter().enumerate() {
            for (wid, col) in row.iter().enumerate() {
                if let Some(piece) = col {
                    if piece.player == self.turn {
                        ret.append(&mut self.piece_legal_move(hei as isize, wid as isize, piece))
                    }
                }
            }
        }
        ret
    }
    // #[tracing::instrument(skip(self))]
    pub fn action_parse(&mut self, act: Option<Act>) -> (bool, Option<Player>) {
        // パスしかできなかった
        if act == None {
            self.next_turn();
            return (true, None);
        }
        // ただしくないact
        let act = act.unwrap();
        let legalmoves = self.legal_moves();
        if !legalmoves.iter().any(|&x| x == act) {
            return (false, None);
        }
        // 正しいactであれば移動を実行
        // もちろんキックも同時に実行　ここで勝利判定
        match act.kickto {
            Some(kick) => {
                let last_stop = kick.0;
                if last_stop == 0 - 1 {
                    return (true, Some(self.turn.against()));
                } else if last_stop == HEIGHT {
                    return (true, Some(self.turn));
                }
                let mut tmp = None;
                std::mem::swap(
                    &mut self.board[act.to.0 as usize][act.to.1 as usize], // here None
                    &mut tmp,                                              // here ball
                );
                std::mem::swap(
                    &mut self.board[kick.0 as usize][kick.1 as usize], // here ball
                    &mut tmp,                                          // here None
                );
                std::mem::swap(
                    &mut self.board[act.from.0 as usize][act.from.1 as usize], // here None
                    &mut tmp,                                                  // here piece
                );
                std::mem::swap(
                    &mut self.board[act.to.0 as usize][act.to.1 as usize], // here piece
                    &mut tmp,                                              // here None
                );
            }
            None => {
                let mut tmp = None;
                std::mem::swap(
                    &mut self.board[act.from.0 as usize][act.from.1 as usize], // here None
                    &mut tmp,                                                  // here Piece
                );
                std::mem::swap(
                    &mut self.board[act.to.0 as usize][act.to.1 as usize], // here Piece
                    &mut tmp,                                              // here None
                );
            }
        }
        // tracing::debug!("board: {:?}", self.board);
        // 親猿にグレードアップ
        // turn変更
        if act.to.0 == HEIGHT - 1 {
            if let PieceKind::Oyasaru(_, _) = self.board[act.to.0 as usize][act.to.1 as usize]
                .as_ref()
                .unwrap()
                .piecekind
            {
                self.board[act.to.0 as usize][act.to.1 as usize] = Some(Piece {
                    player: self.turn,
                    piecekind: PieceKind::Oyasaru([-2, 2], SARU_KICK.iter().copied().collect()),
                });
            }
        }
        self.next_turn();
        // 成功をreturn
        (true, None)
    }
    #[tracing::instrument]
    pub fn agent_game<T: Agent + std::fmt::Debug>(agent1: T, agent2: T) -> String {
        let mut game_ins = Self::setup();
        let turn = rand::thread_rng().gen_range(0..=1);
        let ((front, back), (frontstr, backstr)) = if turn == 0 {
            ((agent1, agent2), ("agent1", "agent2"))
        } else {
            ((agent2, agent1), ("agent2", "agent1"))
        };
        let mut now_player = (true, &front);
        loop {
            let mut winner: Option<Player>;
            loop {
                let action: Option<Act>;
                // tracing::debug!("legal moves: {:?}", game_ins.legal_moves());
                if game_ins.legal_moves().is_empty() {
                    action = None;
                } else {
                    action = Some(now_player.1.action(&game_ins));
                }
                tracing::debug!("the move is {:?}", action);
                let (success, winnertmp) = game_ins.action_parse(action);
                winner = winnertmp;
                if success {
                    break;
                }
            }
            if let Some(player) = winner {
                match player {
                    Player::Attack => {
                        if now_player.0 {
                            return format!("{} win", frontstr);
                        } else {
                            return format!("{} win", backstr);
                        }
                    }
                    Player::Defence => {
                        if now_player.0 {
                            return format!("{} win", backstr);
                        } else {
                            return format!("{} win", frontstr);
                        }
                    }
                    _ => (),
                }
            }
            now_player = if now_player.0 {
                (false, &back)
            } else {
                (true, &front)
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn piece_can_move_works() {
        let game = Game::setup();
        let input = vec![(1, 1, game.board[1][1].as_ref().unwrap())];
        let move_legal_seed = vec![(2, 0), (2, 2), (1, 0), (1, 2), (0, 1)];
        let mut move_legal_unwrap: Vec<Act> = vec![];
        for moves in move_legal_seed {
            move_legal_unwrap.push(Act {
                from: (1, 1),
                to: moves,
                kickto: None,
            })
        }
        let ball_legal_unwrap = Act {
            from: (1, 1),
            to: (2, 1),
            kickto: None,
        };
        let output = vec![(move_legal_unwrap, ball_legal_unwrap)];
        for (inp, mut ref_out) in input.into_iter().zip(output.into_iter()) {
            let (move_legal, ball_legal) = game.piece_can_move(inp.0, inp.1, inp.2);
            assert_eq!(ref_out.0.sort(), move_legal.unwrap().sort());
            assert_eq!(ref_out.1, ball_legal.unwrap());
        }
    }
    use crate::randai;
    #[test]
    fn random_battle() {
        for _ in 0..50 {
            let agent1 = randai::Randai {};
            let agent2 = randai::Randai {};
            Game::agent_game(agent1, agent2);
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
        Game::agent_game(agent1, agent2);
    }
}
