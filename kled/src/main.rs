use std::{collections::HashMap, io::stdin};
use minimax::{EvalResult, abmax::ABMax, minimax::MiniMax};

use newcular::{
    board::{Board, PieceKind, Player},
    simple::{SimpleBoard, SimpleMove},
};

// fn piece_rank(kind: &PieceKind) -> i32 {
//     match kind {
//         PieceKind::B => 5,
//         PieceKind::K => 50,
//         PieceKind::N => 3,
//         PieceKind::R => 5,
//         PieceKind::P => 1,
//     }
// }

fn eval(b: &SimpleBoard) -> i32 {
    b.eval
    // b.rows
    //     .iter()
    //     .flatten()
    //     .filter_map(|sq| match sq {
    //         Some((Player::PlayerOne, kind)) => Some(piece_rank(kind)),
    //         Some((Player::PlayerTwo, kind)) => Some(-piece_rank(kind)),
    //         None => None,
    //     })
    //     .sum()
}

fn main() {
    let mut board = SimpleBoard::init();
    let mut who_am_i = Player::PlayerOne;
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
            // let mut mm = ABMax {
            //     eval: |b|{eval(b)},
            //     // alpha: EvalResult::FavorTwo(0),
            //     // beta: EvalResult::FavorOne(0),
            //  };
            let (mov, evaluation) = ABMax::choose_best_iterdeep(&board, eval);
            println!("I'll play {}. {}", mov, match evaluation {
                EvalResult::FavorOne(plies) if Player::PlayerOne == who_am_i => format!("I'll win in {plies}!"),
                EvalResult::FavorTwo(plies) if Player::PlayerOne == who_am_i => format!("I'll lose in {plies}!"),
                
                EvalResult::FavorOne(plies) if Player::PlayerTwo == who_am_i => format!("I'll lose in {plies}!"),
                EvalResult::FavorTwo(plies) if Player::PlayerTwo == who_am_i => format!("I'll win in {plies}!"),

                EvalResult::Evaluate(favor) if favor == 0 => format!("I'm feeling indifferent."),
                EvalResult::Evaluate(favor) if Player::PlayerOne == who_am_i && favor > 0 => format!("I'm feeling good!"),
                EvalResult::Evaluate(favor) if Player::PlayerTwo == who_am_i && favor < 0 => format!("I'm feeling good!"),
                _ => format!("I'm feeling bad!"),
            });
            board.do_move(&mov);
            who_am_i = who_am_i.other();
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
