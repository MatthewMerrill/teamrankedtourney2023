use minimax::{abmax::ABMax, minimax::MiniMax, EvalResult};
use std::{collections::HashMap, fmt::Display, io::{stdin, self, Write}};

use newcular::{
    bitboard::BitBoard,
    board::{Board, Mov, Player},
    simple::SimpleBoard,
};

mod termdisplay;

// fn piece_rank(kind: &PieceKind) -> i32 {
//     match kind {
//         PieceKind::B => 5,
//         PieceKind::K => 50,
//         PieceKind::N => 3,
//         PieceKind::R => 5,
//         PieceKind::P => 1,
//     }
// }

fn eval_bitboard(b: &BitBoard) -> i32 {
    0
    // b.eval
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
    play(BitBoard::init(), eval_bitboard);
}

fn play<M, B, F>(mut board: B, eval: F)
where
    M: Mov + Copy + Clone + Display + Send + 'static,
    B: Board<M> + Display + Send + Clone + 'static,
    F: Send + Clone + 'static + Fn(&B) -> i32,
{
    let mut who_am_i = Player::PlayerOne;
    let mut term = termdisplay::TermDisplay {
        prev_state: board.clone(),
        cur_state: board.clone(),
        move_history: vec![],
    };
    loop {
        term.display_all();
        if let Some(winner) = board.get_winner() {
            println!("Winner: {:?}", winner);
            break;
        }
        let moves_by_repr = board
            .get_moves()
            .iter()
            .map(|&m| (m.to_string(), m))
            .collect::<HashMap<String, M>>();
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
            print!("Thinking...");
            io::stdout().flush();
            let (mov, evaluation) = ABMax::<M, B, _>::choose_best_iterdeep(&board, eval.clone());
            println!(
                "I'll play {}. {}",
                mov,
                match evaluation {
                    EvalResult::FavorOne(plies) if Player::PlayerOne == who_am_i =>
                        format!("I'll win in {plies}!"),
                    EvalResult::FavorTwo(plies) if Player::PlayerOne == who_am_i =>
                        format!("I'll lose in {plies}!"),

                    EvalResult::FavorOne(plies) if Player::PlayerTwo == who_am_i =>
                        format!("I'll lose in {plies}!"),
                    EvalResult::FavorTwo(plies) if Player::PlayerTwo == who_am_i =>
                        format!("I'll win in {plies}!"),

                    EvalResult::Evaluate(favor) if favor == 0 =>
                        format!("I'm feeling indifferent."),
                    EvalResult::Evaluate(favor) if Player::PlayerOne == who_am_i && favor > 0 =>
                        format!("I'm feeling good!"),
                    EvalResult::Evaluate(favor) if Player::PlayerTwo == who_am_i && favor < 0 =>
                        format!("I'm feeling good!"),
                    _ => format!("I'm feeling bad!"),
                }
            );
            board.do_move(&mov);
            term.prev_state = term.cur_state.clone();
            term.cur_state = board.clone();
            term.move_history.push(mov);
            // who_am_i = who_am_i.other();
        } else {
            loop {
                println!("Enter a move: ");
                let mut line = String::new();
                if let Err(_) = stdin().read_line(&mut line) {
                    println!("Could not read move.");
                    continue;
                }
                match moves_by_repr.get(&line.trim().to_uppercase()) {
                    Some(&mov) => {
                        board.do_move(&mov);
                        term.prev_state = term.cur_state.clone();
                        term.cur_state = board.clone();
                        term.move_history.push(mov);
                        break;
                    }
                    None => {
                        println!("Not a valid move.");
                    }
                }
            }
        }
    }
}
