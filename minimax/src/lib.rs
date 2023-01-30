use std::cmp::Ordering;

use newcular::{
    board::{Board, MoveError, Player},
    simple::{SimpleBoard, SimpleMove},
};

type EvalFn = dyn Fn(&SimpleBoard) -> i32;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EvalResult {
    FavorOne(u8),
    Evaluate(i32),
    FavorTwo(u8),
}

impl EvalResult {
    fn level_up(&self) -> EvalResult {
        match self {
            EvalResult::FavorOne(a) => EvalResult::FavorOne(a + 1),
            EvalResult::FavorTwo(a) => EvalResult::FavorTwo(a + 1),
            f => *f,
        }
    }
}

impl Ord for EvalResult {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // b is "greater" if smaller (winning sooner)
            (EvalResult::FavorOne(a), EvalResult::FavorOne(b)) => b.cmp(a),
            (EvalResult::FavorOne(_), EvalResult::Evaluate(_)) => Ordering::Greater,
            (EvalResult::FavorOne(_), EvalResult::FavorTwo(_)) => Ordering::Greater,

            (EvalResult::Evaluate(_), EvalResult::FavorOne(_)) => Ordering::Less,
            (EvalResult::Evaluate(a), EvalResult::Evaluate(b)) => a.cmp(b),
            (EvalResult::Evaluate(_), EvalResult::FavorTwo(_)) => Ordering::Greater,

            (EvalResult::FavorTwo(_), EvalResult::FavorOne(_)) => Ordering::Less,
            (EvalResult::FavorTwo(_), EvalResult::Evaluate(_)) => Ordering::Less,
            (EvalResult::FavorTwo(a), EvalResult::FavorTwo(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd<EvalResult> for EvalResult {
    fn partial_cmp(&self, other: &EvalResult) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct MiniMax<F> where F: Fn(&SimpleBoard) -> i32 {
    pub eval: F,
}

impl <F> MiniMax<F> where F: Fn(&SimpleBoard) -> i32 {
    pub fn choose_best(&self, board: &SimpleBoard, plies: u8) -> SimpleMove {
        let moves = board.get_moves();
        *match board.get_player() {
            Player::PlayerOne => moves
                .iter()
                .max_by_key(|m| {
                    let mut board = board.clone();
                    board.do_move(m).unwrap();
                    self.mini(&board, plies)
                })
                .unwrap(),
            Player::PlayerTwo => moves
                .iter()
                .min_by_key(|m| {
                    let mut board = board.clone();
                    board.do_move(m).unwrap();
                    self.maxi(&board, plies)
                })
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
