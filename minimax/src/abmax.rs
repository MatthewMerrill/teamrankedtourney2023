use std::hash::Hash;
use std::rc::Rc;
use std::{
    marker::PhantomData,
    thread,
    time::{Duration, Instant},
};

use crate::EvalResult;
use crossbeam_channel::{after, Receiver};
use newcular::board::{Board, Mov, Player};

use crate::ttable::TTable;

// type EvalFn = dyn Fn(&SimpleBoard) -> i32;

pub struct ABMax<M, B, F>
where
    M: Mov + Send + Clone,
    B: Board<M> + Clone + Send + Hash + Eq,
    F: Fn(&B) -> i32,
{
    eval: F,
    ttable: TTable<M, B>,
    rx: Receiver<Instant>,

    _phantom_move: PhantomData<M>,
    _phantom_board: PhantomData<B>,
    // pub alpha: EvalResult,
    // pub beta: EvalResult,
}

// struct ABMaxSearch<M, B, F>
// where
//     M: Mov + Send + Clone,
//     B

impl<M, B, F> ABMax<M, B, F>
where
    M: Mov + Send + Clone,
    B: Board<M>,
    F: 'static,
    F: Send + Sync + Fn(&B) -> i32,
{
    pub fn new(eval: F) -> Self {
        Self {
            eval,
            ttable: TTable::new(),
            rx: crossbeam_channel::never(),

            _phantom_move: PhantomData {},
            _phantom_board: PhantomData {},
        }
    }

    pub fn choose_best_iterdeep(&mut self, board: &B) -> (M, EvalResult) {
        let (tx, rx) = crossbeam_channel::unbounded::<(M, EvalResult)>();
        self.rx = after(Duration::from_secs(5));
        let board = board.clone();

        // thread::spawn(|| {
        // let mut best = ;
        let mut best = None;
        let mut depth = 3;
        loop {
            match self.choose_best(&board, depth) {
                // todo: short-circuit for win/loss?
                Some(x) => {
                    // tx.send(x.clone()).unwrap();
                    best = Some(x.clone());
                    match x {
                        (_, EvalResult::Evaluate(_)) => {}
                        _ => {
                            println!("Got {depth} plies deep!");
                            break;
                        }
                    }
                }
                None => {
                    println!("Got {depth} plies deep!");
                    break;
                }
            };
            depth += 1;
        }
        // });
        // rx.iter().last().unwrap()
        best.unwrap()
    }

    pub fn choose_best(&mut self, board: &B, plies: u8) -> Option<(M, EvalResult)> {
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
        assert_eq!(board.get_player(), Player::PlayerOne);
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
        let mut entry = self.ttable.load(board.clone());
        if entry.plies > plies && entry.value.is_some() {
            return entry.value;
        }
        let mut best = (0, EvalResult::FavorTwo(0));
        let mut moves = entry.moves.clone();
        moves.swap(0usize, entry.killer_move_index);
        // let moves = board.get_moves();
        for (idx, m) in moves.into_iter().enumerate() {
            let mut board = board.clone();
            board.do_move(&m);
            match self.mini(&board, alpha, beta, plies - 1) {
                Some(x) => {
                    let here = x.level_up();
                    if here > best.1 {
                        best = (idx, here);
                    }
                }
                None => return None,
            }
            alpha = Ord::max(alpha, best.1);
            if best.1 >= beta {
                break;
            }
        }
        self.ttable.write(board.clone(), plies, best.1, best.0);
        Some(best.1)
    }

    fn mini(
        &mut self,
        board: &B,
        alpha: EvalResult,
        mut beta: EvalResult,
        plies: u8,
    ) -> Option<EvalResult> {
        assert_eq!(board.get_player(), Player::PlayerTwo);
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
        let mut entry = self.ttable.load(board.clone());
        if entry.plies > plies && entry.value.is_some() {
            return entry.value;
        }
        let mut best = (0, EvalResult::FavorOne(0));
        let mut moves = entry.moves.clone();
        moves.swap(0usize, entry.killer_move_index);
        // let moves = board.get_moves();
        for (idx, m) in moves.iter().enumerate() {
            let mut board = board.clone();
            board.do_move(&m);
            match self.maxi(&board, alpha, beta, plies - 1) {
                Some(x) => {
                    let here = x.level_up();
                    if here < best.1 {
                        best = (idx, here);
                    }
                }
                None => return None,
            }
            beta = Ord::min(beta, best.1);
            if best.1 <= alpha {
                break;
            }
        }
        self.ttable.write(board.clone(), plies, best.1, best.0);
        Some(best.1)
    }
}
