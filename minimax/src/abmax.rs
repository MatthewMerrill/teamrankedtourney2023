use std::{
    thread,
    time::{Duration, Instant}, marker::PhantomData,
};

use crate::EvalResult;
use crossbeam_channel::{after, Receiver};
use newcular::{
    board::{Board, Player, Mov},
};

// type EvalFn = dyn Fn(&SimpleBoard) -> i32;

pub struct ABMax<M, B, F>
where
    M: Mov + Send + Clone,
    B: Board<M> + Clone + Send,
    F: Send,
    F: Fn(&B) -> i32,
{
    eval: F,
    rx: Receiver<Instant>,

    _phantom_move: PhantomData<M>,
    _phantom_board: PhantomData<B>,
    // pub alpha: EvalResult,
    // pub beta: EvalResult,
}

impl<M, B, F> ABMax<M, B, F>
where
    M: Mov + Send + Clone + 'static,
    B: Board<M> + Clone + Send + 'static,
    F: Send + 'static,
    F: Fn(&B) -> i32,
{
    pub fn choose_best_iterdeep(board: &B, eval: F) -> (M, EvalResult) {
        let (tx, rx) = crossbeam_channel::unbounded::<(M, EvalResult)>();
        let mut ab = ABMax {
            eval,
            rx: after(Duration::from_secs(5)),
            _phantom_move: PhantomData {},
            _phantom_board: PhantomData {},
        };
        let board = board.clone();
        thread::spawn(move || {
            let mut depth = 3;
            loop {
                match ab.choose_best(&board, depth) {
                    // todo: short-circuit for win/loss?
                    Some(x) => {
                        tx.send(x.clone()).unwrap();
                        match x {
                            (_, EvalResult::Evaluate(_)) => {},
                            _ => { println!("Got {depth} plies deep!"); break; },
                        }
                    },
                    None => { println!("Got {depth} plies deep!"); break; },
                };
                depth += 1;
            }
        });
        rx.iter().last().unwrap()
    }

    pub fn choose_best(
        &mut self,
        board: &B,
        plies: u8,
    ) -> Option<(M, EvalResult)> {
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
                results.into_iter().max_by_key(|t| t.1) //.copied()
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
                results.into_iter().min_by_key(|t| t.1) //.copied()
            }
        }
    }

    fn maxi(
        &mut self,
        board: &B,
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
        board: &B,
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
