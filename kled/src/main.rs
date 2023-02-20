use std::{collections::HashMap, io::stdin};
use minimax::MiniMax;

use newcular::{
    board::{Board, PieceKind, Player},
    simple::{SimpleBoard, SimpleMove},
};

// mod termdisplay;

fn piece_rank(kind: &PieceKind) -> i32 {
    match kind {
        B => 3,
        K => 20,
        N => 3,
        R => 5,
        P => 1,
    }
}

fn eval(b: &SimpleBoard) -> i32 {
    b.rows
        .iter()
        .flatten()
        .filter_map(|sq| match sq {
            Some((Player::PlayerOne, kind)) => Some(piece_rank(kind)),
            Some((Player::PlayerTwo, kind)) => Some(-piece_rank(kind)),
            None => None,
        })
        .sum()
}

fn main() {
    let mut board = SimpleBoard::init();
    let who_am_i = Player::PlayerOne;
    loop {
        println!("{}", board);
        if let Some(winner) = board.get_winner() {
            println!("Winner: {:?}", winner);
            break;
        }
        let moves_by_repr = board
            .get_moves()
            .iter()
            .map(|&m| (m.to_string(), m))
            .collect::<HashMap<String, SimpleMove>>();
        println!(
            "Valid moves: {}",
            moves_by_repr
                .keys()
                .map(|a| a.as_str())
                .collect::<Vec<&str>>()
                .join(", ")
        );

        if board.get_player() == who_am_i {
            let mm = MiniMax { eval: |b|{eval(b)} };
            let mov = mm.choose_best(&board, 2);
            println!("I'll play {}", mov);
            board.do_move(&mov);
        } else {
            println!("Enter a move: ");
            let mut line = String::new();
            if let Err(_) = stdin().read_line(&mut line) {
                println!("Could not read move.");
                continue;
            }
            match moves_by_repr.get(&line.trim().to_uppercase()) {
                Some(mov) => {
                    board.do_move(mov);
                }
                None => {
                    println!("Not a valid move.");
                }
            }
        }
    }
}
