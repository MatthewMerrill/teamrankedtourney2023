use crate::EvalResult;
use newcular::board::{Board, Mov};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

pub struct TTable<M: Mov, B: Board<M>> {
    entry_by_board: HashMap<B, TTableEntry<M>>,
}

pub struct TTableEntry<M> {
    pub plies: u8,
    pub value: Option<EvalResult>,
    pub killer_move_index: usize,
    pub moves: Vec<M>,
}

impl<M: Mov, B: Board<M>> TTable<M, B> {
    pub fn new() -> Self {
        TTable {
            entry_by_board: HashMap::new(),
        }
    }

    pub fn load(&mut self, board: B) -> &mut TTableEntry<M> {
        self.entry_by_board
            .entry(board)
            .or_insert_with_key(move |b| TTableEntry {
                plies: 0,
                value: None,
                killer_move_index: 0,
                moves: b.get_moves(),
            })
    }
    pub fn write(&mut self, board: B, plies: u8, value: EvalResult, killer_move_idx: usize) {
        let mut entry = self.load(board);
        if plies >= entry.plies {
            entry.plies = plies;
            entry.value = Some(value);
            entry.killer_move_index = killer_move_idx;
        }
    }
}
