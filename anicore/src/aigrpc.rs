use crate::game::Game;
use crate::Act;
use crate::Agent;

use animalai::ai_player_client::AiPlayerClient;
use animalai::BoardReq;

pub mod animalai {
    tonic::include_proto!("animalai");
}

#[derive(Debug, Clone)]
pub struct AIgRPC {}

type Board = Vec<Vec<Vec<f32>>>;
fn into_1darray(board: Board) -> Vec<f32> {
    let i = board.len();
    let x = board[0].len();
    let y = board[0][0].len();
    let all_len = i * x * y;
    let mut ret = vec![0.0; all_len];
    #[allow(clippy::needless_range_loop)]
    for index in 0..all_len {
        let new_i = index / (x * y);
        let new_x = (index % (x * y)) / y;
        let new_y = (index % (x * y)) % y;
        ret[index] = board[new_i][new_x][new_y];
    }
    ret
}

fn num_legalmoves_from_act(acts: &[Act]) -> Vec<u64> {
    let mut ret: Vec<u64> = vec![];
    // パスではないことが保証されている
    // if acts.is_empty() {
    //     let from_num = 15;
    //     let to_num = 15;
    //     let kickto_num = 15;
    //     let action_num = from_num * 288 + to_num * 18 + kickto_num;
    //     ret.push(action_num);
    //     ret
    // } else {
    for act in acts {
        let from_num = act.from.0 * 3 + act.from.1;
        let to_num = act.to.0 * 3 + act.to.1;
        let kickto_num = match act.kickto {
            None => 15,
            Some(kick_to) => {
                if kick_to.0 == -1 {
                    16
                } else if kick_to.0 == 5 {
                    17
                } else {
                    kick_to.0 * 3 + kick_to.1
                }
            }
        };
        let action_num = from_num as u64 * 288 + to_num as u64 * 18 + kickto_num as u64;
        if !ret.contains(&action_num) {
            ret.push(action_num);
        }
    }
    ret
    // }
}

fn act_from_num(action_num: u64, acts: &[Act]) -> Act {
    let from_num = action_num / 288;
    let to_num = (action_num % 288) / 18;
    let kickto_num = (action_num % 288) % 18;

    let from = ((from_num / 3) as isize, (from_num % 3) as isize);
    let to = ((to_num / 3) as isize, (to_num % 3) as isize);
    let kickto = if kickto_num == 15 {
        None
    } else if kickto_num == 16 {
        acts.iter()
            .find(|&x| x.kickto.unwrap_or((0, 0)).0 == -1)
            .unwrap()
            .kickto
    } else if kickto_num == 17 {
        acts.iter()
            .find(|&x| x.kickto.unwrap_or((0, 0)).0 == 5)
            .unwrap()
            .kickto
    } else {
        Some(((kickto_num / 3) as isize, (kickto_num % 3) as isize))
    };
    Act { from, to, kickto }
}

impl Agent for AIgRPC {
    fn action(&self, game: &Game) -> Act {
        let legalmoves = game.legal_moves();
        let cells = into_1darray(game.board_to_tensor());
        let legal_moves = num_legalmoves_from_act(&legalmoves);
        // tracing::info!("legal_moves: {:?}", &legal_moves);
        let rt = tokio::runtime::Runtime::new().unwrap();
        let response = rt.block_on(async move {
            let mut client = AiPlayerClient::connect("http://anipy:50051").await.unwrap();
            let request = tonic::Request::new(BoardReq { cells, legal_moves });
            client.think_action(request).await.unwrap()
        });

        let action_num = response.into_inner().action;
        // tracing::info!("action_num: {:?}", &action_num);
        act_from_num(action_num, &legalmoves)
    }
}
