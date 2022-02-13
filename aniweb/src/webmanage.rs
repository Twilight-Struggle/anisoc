use crate::aigame::{AiGame, Status};
use actix_files::NamedFile;
use actix_web::dev::Server;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use anicore::randai::Randai;
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

#[derive(Debug, Serialize)]
pub struct MakeResult {
    pub id: Uuid,
    pub board: Vec<Vec<Option<String>>>,
    pub res: String,
}

#[tracing::instrument(
    skip(data),
    fields(
        request_id = %Uuid::new_v4(),
    )
)]
#[post("/make")]
async fn make(data: web::Data<GameManeger<Randai>>) -> HttpResponse {
    let mut games = data.games.lock().unwrap();
    let opponent = Randai {};
    let new_id = Uuid::new_v4();
    let aigame = AiGame::setup(opponent);
    games.insert(new_id, aigame.clone());
    HttpResponse::Ok().json(MakeResult {
        id: new_id,
        board: aigame.board(),
        res: "made".to_string(),
    })
}

#[derive(Debug, Deserialize)]
pub struct Resetcmd {
    pub id: Uuid,
}
#[tracing::instrument(
    skip(data),
    fields(
        request_id = %Uuid::new_v4(),
    )
)]
#[post("/reset")]
async fn reset(cmd: web::Json<Resetcmd>, data: web::Data<GameManeger<Randai>>) -> HttpResponse {
    let mut games = data.games.lock().unwrap();
    match games.get(&cmd.id) {
        Some(_) => {
            let opponent = Randai {};
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

#[derive(Debug, Deserialize)]
pub struct Movcmd {
    pub id: Uuid,
    pub act: Act,
}
#[tracing::instrument(
    skip(data),
    fields(
        request_id = %Uuid::new_v4(),
    )
)]
#[post("/mov")]
async fn mov(cmd: web::Json<Movcmd>, data: web::Data<GameManeger<Randai>>) -> HttpResponse {
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
    let gamemaneger = web::Data::new(GameManeger::<Randai> {
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
