use std::{collections::HashMap, io::stdin};

use newcular::{board::Board, simple::{SimpleBoard, SimpleMov}};

fn main() {
    let mut board = SimpleBoard::init();
    loop {
        if let Some(winner) = board.get_winner() {
            println!("Winner: {:?}", winner);
            break;
        }
        println!("{}", board);
        let moves_by_repr = board
            .get_moves()
            .iter()
            .map(|&m| (m.to_string(), m))
            .collect::<HashMap<String, SimpleMov>>();
        println!("Valid moves: {}", moves_by_repr.keys().map(|a|a.as_str()).collect::<Vec<&str>>().join(", "));
        println!("Enter a move: ");

        let mut line = String::new();
        if let Err(_) = stdin().read_line(&mut line) {
            println!("Could not read move.");
            continue;
        }
        match moves_by_repr.get(&line.trim().to_uppercase()) {
            Some(mov) => {board.do_move(mov);},
            None => {println!("Not a valid move.");},
        }
    }
}
