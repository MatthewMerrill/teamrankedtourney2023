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


pub trait Board<Mov, Pla> {
    fn get_player(&self) -> Pla;
    fn get_moves(&self) -> Vec<Mov>;
    fn get_winner(&self) -> Option<Pla>;
    fn do_move(&mut self, mov: &Mov) -> Result<(), MoveError>;
}