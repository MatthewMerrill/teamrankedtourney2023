use std::{
    thread,
    time::{Duration, Instant},
};

use crate::EvalResult;
use crossbeam_channel::{after, Receiver};
use newcular::{
    board::{Board, Player},
    simple::{SimpleBoard, SimpleMove},
};

// type EvalFn = dyn Fn(&SimpleBoard) -> i32;

pub struct ABMax<F>
where
    F: Send,
    F: Fn(&SimpleBoard) -> i32,
{
    eval: F,
    rx: Receiver<Instant>,
    // pub alpha: EvalResult,
    // pub beta: EvalResult,
}

impl<F> ABMax<F>
where
    F: Send + 'static,
    F: Fn(&SimpleBoard) -> i32,
{
    pub fn choose_best_iterdeep(board: &SimpleBoard, eval: F) -> (SimpleMove, EvalResult) {
        let (tx, rx) = crossbeam_channel::unbounded::<(SimpleMove, EvalResult)>();
        let mut ab = ABMax {
            eval,
            rx: after(Duration::from_secs(5)),
        };
        let board = board.clone();
        thread::spawn(move || {
            let mut depth = 3;
            loop {
                match ab.choose_best(&board, depth) {
                    // todo: short-circuit for win/loss?
                    Some(x) => tx.send(x).unwrap(),
                    None => { println!("Got {depth} plies deep!"); break; },
                };
                depth += 1;
            }
        });
        rx.iter().last().unwrap()
    }

    pub fn choose_best(
        &mut self,
        board: &SimpleBoard,
        plies: u8,
    ) -> Option<(SimpleMove, EvalResult)> {
        let moves = board.get_moves();
        match board.get_player() {
            Player::PlayerOne => {
                let mut results = vec![];
                for m in moves {
                    let mut board = board.clone();
                    board.do_move(&m);
                    match self.mini(
                        &board,
                        EvalResult::FavorTwo(0),
                        EvalResult::FavorOne(0),
                        plies - 1,
                    ) {
                        // todo: should we short-circuit here for win?
                        Some(x) => results.push((m, x)),
                        None => return None,
                    }
                }
                results.iter().max_by_key(|t| t.1).copied()
            }
            Player::PlayerTwo => {
                let mut results = vec![];
                for m in moves {
                    let mut board = board.clone();
                    board.do_move(&m);
                    match self.maxi(
                        &board,
                        EvalResult::FavorTwo(0),
                        EvalResult::FavorOne(0),
                        plies - 1,
                    ) {
                        Some(x) => results.push((m, x)),
                        None => return None,
                    }
                }
                results.iter().min_by_key(|t| t.1).copied()
            }
        }
    }

    fn maxi(
        &mut self,
        board: &SimpleBoard,
        mut alpha: EvalResult,
        beta: EvalResult,
        plies: u8,
    ) -> Option<EvalResult> {
        assert!(board.get_player() == Player::PlayerOne);
        if let Ok(_) = self.rx.try_recv() {
            return None;
        }
        if let Some(p) = board.get_winner() {
            return match p {
                Player::PlayerOne => Some(EvalResult::FavorOne(0)),
                Player::PlayerTwo => Some(EvalResult::FavorTwo(0)),
            };
        }
        if plies == 0 {
            return Some(EvalResult::Evaluate((self.eval)(board)));
        }
        let mut best = EvalResult::FavorTwo(0);
        for m in board.get_moves() {
            let mut board = board.clone();
            board.do_move(&m);
            match self.mini(&board, alpha, beta, plies - 1) {
                Some(x) => best = Ord::max(best, x.level_up()),
                None => return None,
            }
            alpha = Ord::max(alpha, best);
            if best >= beta {
                break;
            }
        }
        Some(best)
    }

    fn mini(
        &mut self,
        board: &SimpleBoard,
        alpha: EvalResult,
        mut beta: EvalResult,
        plies: u8,
    ) -> Option<EvalResult> {
        assert!(board.get_player() == Player::PlayerTwo);
        if let Ok(_) = self.rx.try_recv() {
            return None;
        }
        if let Some(p) = board.get_winner() {
            return match p {
                Player::PlayerOne => Some(EvalResult::FavorOne(0)),
                Player::PlayerTwo => Some(EvalResult::FavorTwo(0)),
            };
        }
        if plies == 0 {
            return Some(EvalResult::Evaluate((self.eval)(board)));
        }
        let mut best = EvalResult::FavorOne(0);
        for m in board.get_moves() {
            let mut board = board.clone();
            board.do_move(&m);
            match self.maxi(&board, alpha, beta, plies - 1) {
                Some(x) => best = Ord::min(best, x.level_up()),
                None => return None,
            }
            beta = Ord::min(beta, best);
            if best <= alpha {
                break;
            }
        }
        Some(best)
    }
}
