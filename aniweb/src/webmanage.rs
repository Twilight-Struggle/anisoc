use actix_files::NamedFile;
use actix_web::dev::Server;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use anicore::game;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::Mutex;
use uuid::Uuid;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

struct GameManeger {
    games: Mutex<HashMap<Uuid, game::Game>>,
}

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("target/public/index.html")?)
}

#[derive(Debug, Serialize)]
pub struct MakeResult {
    pub id: Uuid,
    pub board: Vec<Vec<String>>,
    pub res: String,
}

// make
// IDとGameインスタンスを作成　GameManagerに登録してID、ボード状態、現在の状態(MakeResult)を返却

// reset in ID
// GameManagerから該当IDのものを抜き出し、新しいGameインスタンスを変わりに入れる　MakeResultを返却

// mov
// GameManegerから該当ID抜き出し→Gameインスタンスのactionメソッド叩く→結果を返却
