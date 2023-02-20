use crate::EvalResult;
use newcular::{
    board::{Board, Player},
    simple::{SimpleBoard, SimpleMove},
};

type EvalFn = dyn Fn(&SimpleBoard) -> i32;

pub struct MiniMax<F> where F: Fn(&SimpleBoard) -> i32 {
    pub eval: F,
}

impl <F> MiniMax<F> where F: Fn(&SimpleBoard) -> i32 {
    pub fn choose_best(&self, board: &SimpleBoard, plies: u8) -> (SimpleMove, EvalResult) {
        let moves = board.get_moves();
        match board.get_player() {
            Player::PlayerOne => moves
                .iter()
                .map(|&m| {
                    let mut board = board.clone();
                    board.do_move(&m);
                    (m, self.mini(&board, plies))
                })
                .max_by_key(|(_, eval)|*eval)
                .unwrap(),
            Player::PlayerTwo => moves
                .iter()
                .map(|&m| {
                    let mut board = board.clone();
                    board.do_move(&m);
                    (m, self.maxi(&board, plies))
                })
                .min_by_key(|(_, eval)|*eval)
                .unwrap(),
        }
    }
    fn maxi(&self, board: &SimpleBoard, plies: u8) -> EvalResult {
        if let Some(p) = board.get_winner() {
            return match p {
                Player::PlayerOne => EvalResult::FavorOne(0),
                Player::PlayerTwo => EvalResult::FavorTwo(0),
            };
        }
        if plies == 0 {
            return EvalResult::Evaluate((self.eval)(board));
        }
        board
            .get_moves()
            .iter()
            .map(|m| {
                let mut board = board.clone();
                board.do_move(m);
                return self.mini(&board, plies - 1).level_up();
            })
            .max()
            .unwrap()
    }

    fn mini(&self, board: &SimpleBoard, plies: u8) -> EvalResult {
        if let Some(p) = board.get_winner() {
            return match p {
                Player::PlayerOne => EvalResult::FavorOne(0),
                Player::PlayerTwo => EvalResult::FavorTwo(0),
            };
        }
        if plies == 0 {
            return EvalResult::Evaluate((self.eval)(board));
        }
        board
            .get_moves()
            .iter()
            .map(|m| {
                let mut board = board.clone();
                board.do_move(m);
                return self.maxi(&board, plies - 1).level_up();
            })
            .min()
            .unwrap()
    }
}
