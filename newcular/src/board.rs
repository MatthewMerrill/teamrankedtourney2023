// mod board;

use std::fmt::Display;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MoveError {
    NoSuchPiece,
    NotYourPiece,
    InvalidPosition,
    InvalidMove,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PieceKind {
    B,
    K,
    N,
    R,
    P,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Player {
    PlayerOne,
    PlayerTwo,
}

impl Player {
    pub fn other(&self) -> Player {
        match self {
            Self::PlayerOne => Self::PlayerTwo,
            Self::PlayerTwo => Self::PlayerOne,
        }
    }
    pub fn parity(&self) -> i8 {
        match self {
            Self::PlayerOne => 1,
            Self::PlayerTwo => -1,
        }
    }
    pub fn ord(&self) -> i8 {
        match self {
            Self::PlayerOne => 1,
            Self::PlayerTwo => 2,
        }
    }
}

pub trait Mov: Display {
    fn invert(&self) -> Self;
    fn get_from_dest(&self) -> ((u8, u8), (u8, u8));
}

pub trait Board<M: Mov>: Display + Clone + Send + Sync + Hash + Eq {
    fn get_piece(&self, row: u8, col: u8) -> Option<(Player, PieceKind)>;
    fn get_player(&self) -> Player;
    fn get_moves(&self) -> Vec<M>;
    fn get_winner(&self) -> Option<Player>;
    fn do_move(&mut self, mov: &M);
    fn invert(&self) -> Self;
}
