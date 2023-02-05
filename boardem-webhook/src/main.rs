use std::{collections::HashMap, fmt::format};

use actix_web::{get, middleware, web, App, Error as AWError, HttpResponse, HttpServer, Responder};
use log::info;
use newcular::{
    board::Board,
    simple::{SimpleBoard, SimpleMove},
};
use serde::Serialize;
use std::io;

#[derive(Serialize)]
struct GameSummary {
    valid_moves: Vec<String>,
    render: String,
    winner: Option<i8>,
}

fn play_board_moves(moves: &Vec<String>) -> Result<SimpleBoard, usize> {
    let mut board = SimpleBoard::init();
    for (idx, mov) in moves.iter().enumerate() {
        let moves_by_repr = board
            .get_moves()
            .iter()
            .map(|&m| (m.to_string(), m))
            .collect::<HashMap<String, SimpleMove>>();

        match moves_by_repr.get(mov) {
            Some(mov) => board.do_move(mov),
            None => {
                return Err(idx);
            }
        }
    }
    Ok(board)
}

#[get("/gameType/newcular/validMoves/{moves:([A-Z0-9]+( [A-Z0-9]+)*)?}")]
async fn valid_moves(req: web::Path<(String,)>) -> impl Responder {
    let moves = req
        .0
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    match play_board_moves(&moves) {
        Ok(board) => HttpResponse::Ok().json(
            board
                .get_moves()
                .iter()
                .map(|&m| m.to_string())
                .collect::<Vec<String>>(),
        ),
        Err(e) => {
            return HttpResponse::BadRequest().body(format!("invalid move at index {}", e));
        }
    }
}

#[get("/gameType/newcular/render/{moves:([A-Z0-9]+( [A-Z0-9]+)*)?}")]
async fn render(req: web::Path<(String,)>) -> impl Responder {
    let moves = req
        .0
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    match play_board_moves(&moves) {
        Ok(board) => HttpResponse::Ok().json(board.to_string()),
        Err(e) => {
            return HttpResponse::BadRequest().body(format!("invalid move at index {}", e));
        }
    }
}

#[get("/gameType/newcular/summary/{moves:([A-Z0-9]+( [A-Z0-9]+)*)?}")]
async fn summary(req: web::Path<(String,)>) -> impl Responder {
    let moves = req
        .0
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    match play_board_moves(&moves) {
        Ok(board) => HttpResponse::Ok().json(GameSummary {
            valid_moves: board
                .get_moves()
                .iter()
                .map(|&m| m.to_string())
                .collect::<Vec<String>>(),
            render: board.to_string(),
            winner: board.get_winner().map(|player| player.ord()),
        }),
        Err(e) => {
            return HttpResponse::BadRequest().body(format!("invalid move at index {}", e));
        }
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("starting HTTP server at http://localhost:8181");

    // start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(valid_moves)
            .service(render)
            .service(summary)
    })
    .bind(("127.0.0.1", 8181))?
    .workers(2)
    .run()
    .await
}
