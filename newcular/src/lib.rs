pub mod bitboard;
pub mod board;
pub mod simple;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitboard::{BitBoard, BitBoardMove};
    use crate::board::{Board, Mov};
    use crate::simple::SimpleBoard;
    use rand::prelude::*;
    use std::fmt::Display;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    fn moves_sorted<M: Mov + Display, B: Board<M>>(b: &B) -> Vec<String> {
        let mut moves: Vec<String> = b
            .get_moves()
            .into_iter()
            .map(|mov| mov.to_string())
            .collect();
        moves.sort();
        moves
    }

    #[test]
    fn test_board_equivalence() {
        // version stability not important
        let mut rng = StdRng::seed_from_u64(42);

        for _ in 0..10000 {
            let mut simple = SimpleBoard::init();
            let mut bitboard = BitBoard::init();
            let mut moves = vec![];

            loop {
                assert_eq!(
                    simple.get_winner(),
                    bitboard.get_winner(),
                    "winners not eq after {:?}",
                    moves.join(" ")
                );

                if let Some(_) = simple.get_winner() {
                    break;
                }

                let simple_moves = moves_sorted(&simple);
                let bitboard_moves = moves_sorted(&bitboard);
                assert_eq!(
                    simple_moves,
                    bitboard_moves,
                    "movegen not eq after {:?}\n{}\n{}",
                    moves.join(" "),
                    simple.to_string(),
                    bitboard.to_string()
                );

                let simple_moves = simple.get_moves();
                let picked_move = simple_moves.choose(&mut rng).unwrap();
                moves.push(picked_move.to_string());
                simple.do_move(picked_move);
                let (from, dest) = picked_move.get_from_dest();
                bitboard.do_move(&BitBoardMove::from_from_dest(from, dest).unwrap());
            }
        }
    }
}
