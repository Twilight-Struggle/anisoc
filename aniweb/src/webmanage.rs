use crate::aigame::{AiGame, Status};
use actix_files::NamedFile;
use actix_web::dev::Server;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use anicore::aigrpc::AIgRPC;
// use anicore::randai::Randai;
use anicore::{Act, Agent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::Mutex;
use uuid::Uuid;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

struct GameManeger<T: Agent> {
    games: Mutex<HashMap<Uuid, AiGame<T>>>,
}

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("target/public/index.html")?)
}

#[derive(Debug, Serialize, Deserialize)]
struct MakeResult {
    id: Uuid,
    board: Vec<Vec<Option<String>>>,
    res: String,
}

#[tracing::instrument(
    skip(data),
    fields(
        request_id = %Uuid::new_v4(),
    )
)]
#[post("/make")]
async fn make(data: web::Data<GameManeger<AIgRPC>>) -> HttpResponse {
    let mut games = data.games.lock().unwrap();
    let opponent = AIgRPC {};
    let new_id = Uuid::new_v4();
    let aigame = AiGame::setup(opponent);
    games.insert(new_id, aigame.clone());
    HttpResponse::Ok().json(MakeResult {
        id: new_id,
        board: aigame.board(),
        res: "made".to_string(),
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct Resetcmd {
    id: Uuid,
}
#[tracing::instrument(
    skip(data),
    fields(
        request_id = %Uuid::new_v4(),
    )
)]
#[post("/reset")]
async fn reset(cmd: web::Json<Resetcmd>, data: web::Data<GameManeger<AIgRPC>>) -> HttpResponse {
    let mut games = data.games.lock().unwrap();
    match games.get(&cmd.id) {
        Some(_) => {
            let opponent = AIgRPC {};
            let aigame = AiGame::setup(opponent);
            let board = aigame.board();
            games.insert(cmd.id, aigame);
            HttpResponse::Ok().json(MakeResult {
                id: cmd.id,
                board,
                res: "reseted".to_string(),
            })
        }
        None => HttpResponse::BadRequest().finish(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Movcmd {
    id: Uuid,
    act: Act,
}
#[tracing::instrument(
    skip(data),
    fields(
        request_id = %Uuid::new_v4(),
    )
)]
#[post("/mov")]
async fn mov(cmd: web::Json<Movcmd>, data: web::Data<GameManeger<AIgRPC>>) -> HttpResponse {
    let mut games = data.games.lock().unwrap();
    match games.get_mut(&cmd.id) {
        Some(aigame) => {
            let (status, board) = aigame.action(cmd.act);
            match status {
                Status::GameEnd(stri)
                | Status::InvalidAction(stri)
                | Status::YouWin(stri)
                | Status::Youlose(stri)
                | Status::GameContinue(stri) => HttpResponse::Ok().json(MakeResult {
                    id: cmd.id,
                    board,
                    res: stri,
                }),
            }
        }
        None => HttpResponse::BadRequest().finish(),
    }
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let gamemaneger = web::Data::new(GameManeger::<AIgRPC> {
        games: Mutex::new(HashMap::new()),
    });
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .service(index)
            .app_data(gamemaneger.clone())
            .service(make)
            .service(reset)
            .service(mov)
            .service(actix_files::Files::new("", "target/public"))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, http::header, test};
    use serde_json;

    #[actix_rt::test]
    async fn test_health_ok() {
        let resp = health_check().await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn make_works() {
        let gamemaneger = web::Data::new(GameManeger::<AIgRPC> {
            games: Mutex::new(HashMap::new()),
        });
        let mut app = test::init_service(
            App::new()
                .app_data(gamemaneger.clone())
                .service(make)
                .service(reset)
                .service(mov),
        )
        .await;
        let res = test::TestRequest::post()
            .uri("/make")
            .send_request(&mut app)
            .await;

        assert!(res.status().is_success());
        let result: MakeResult = test::read_body_json(res).await;
        assert_eq!(result.res, *"made");
        assert_eq!(result.board.len(), 5);
        assert_eq!(result.board[0].len(), 3);
        println!("{:?}", result.board);
    }

    #[actix_rt::test]
    async fn reset_works() {
        let gamemaneger = web::Data::new(GameManeger::<AIgRPC> {
            games: Mutex::new(HashMap::new()),
        });
        let mut app = test::init_service(
            App::new()
                .app_data(gamemaneger.clone())
                .service(make)
                .service(reset)
                .service(mov),
        )
        .await;

        let res = test::TestRequest::post()
            .uri("/make")
            .send_request(&mut app)
            .await;
        assert!(res.status().is_success());
        let result: MakeResult = test::read_body_json(res).await;

        let generated_uuid = result.id;
        let payload = serde_json::to_string(&Resetcmd { id: generated_uuid }).unwrap();
        let res = test::TestRequest::post()
            .uri("/reset")
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .send_request(&mut app)
            .await;
        assert!(res.status().is_success());
        let result: MakeResult = test::read_body_json(res).await;
        assert_eq!(result.res, *"reseted");
        assert_eq!(result.id, generated_uuid);
        assert_eq!(result.board.len(), 5);
        assert_eq!(result.board[0].len(), 3);
        println!("{:?}", result.board);
    }

    #[actix_rt::test]
    async fn reset_dont_works() {
        let gamemaneger = web::Data::new(GameManeger::<AIgRPC> {
            games: Mutex::new(HashMap::new()),
        });
        let mut app = test::init_service(
            App::new()
                .app_data(gamemaneger.clone())
                .service(make)
                .service(reset)
                .service(mov),
        )
        .await;

        let payload = serde_json::to_string(&Resetcmd { id: Uuid::new_v4() }).unwrap();
        let res = test::TestRequest::post()
            .uri("/reset")
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .send_request(&mut app)
            .await;
        assert!(!res.status().is_success());
    }

    // テストのためにpubにしないよう上書き
    #[derive(Debug, Serialize, Deserialize)]
    struct Act {
        from: (isize, isize),
        to: (isize, isize),
        kickto: Option<(isize, isize)>,
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct Movcmd {
        id: Uuid,
        act: Act,
    }
    #[actix_rt::test]
    async fn mov_works() {
        let gamemaneger = web::Data::new(GameManeger::<AIgRPC> {
            games: Mutex::new(HashMap::new()),
        });
        let mut app = test::init_service(
            App::new()
                .app_data(gamemaneger.clone())
                .service(make)
                .service(reset)
                .service(mov),
        )
        .await;

        let res = test::TestRequest::post()
            .uri("/make")
            .send_request(&mut app)
            .await;
        assert!(res.status().is_success());
        let result: MakeResult = test::read_body_json(res).await;

        let generated_uuid = result.id;
        let payload = serde_json::to_string(&Movcmd {
            id: generated_uuid,
            act: Act {
                from: (0, 0),
                to: (1, 1),
                kickto: None,
            },
        })
        .unwrap();
        let res = test::TestRequest::post()
            .uri("/mov")
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .send_request(&mut app)
            .await;
        assert!(res.status().is_success());
        let result: MakeResult = test::read_body_json(res).await;
        assert_eq!(result.id, generated_uuid);
        assert_eq!(result.board.len(), 5);
        assert_eq!(result.board[0].len(), 3);
        println!("{:?}", result.board);
    }

    #[actix_rt::test]
    async fn mov_dont_works() {
        let gamemaneger = web::Data::new(GameManeger::<AIgRPC> {
            games: Mutex::new(HashMap::new()),
        });
        let mut app = test::init_service(
            App::new()
                .app_data(gamemaneger.clone())
                .service(make)
                .service(reset)
                .service(mov),
        )
        .await;

        let payload = serde_json::to_string(&Movcmd {
            id: Uuid::new_v4(),
            act: Act {
                from: (0, 0),
                to: (1, 1),
                kickto: None,
            },
        })
        .unwrap();
        let res = test::TestRequest::post()
            .uri("/mov")
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .send_request(&mut app)
            .await;
        assert!(!res.status().is_success());
    }
}
