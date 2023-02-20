// mod board;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MoveError {
    NoSuchPiece,
    NotYourPiece,
    InvalidPosition,
    InvalidMove,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PieceKind {
    B, K, N, R, P,
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

pub trait Mov {
    fn invert(&self) -> Self;
}

pub trait Board<M: Mov> {
    fn get_player(&self) -> Player;
    fn get_moves(&self) -> Vec<M>;
    fn get_winner(&self) -> Option<Player>;
    fn do_move(&mut self, mov: &Mov);
}