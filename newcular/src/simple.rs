use std::fmt::{Debug, Display};

use crate::board::*;

fn piece_rank(kind: &PieceKind) -> i32 {
    match kind {
        PieceKind::B => 5,
        PieceKind::K => 50,
        PieceKind::N => 3,
        PieceKind::R => 5,
        PieceKind::P => 1,
    }
}

#[derive(Clone, Copy, Eq, Debug, PartialEq)]
pub struct SimpleMove {
    from_rc: (u8, u8),
    dest_rc: (u8, u8),
}

impl Mov for SimpleMove {
    fn invert(&self) -> Self {
        SimpleMove {
            from_rc: (9 - self.from_rc.0, self.from_rc.1),
            dest_rc: (9 - self.dest_rc.0, self.dest_rc.1),
        }
    }

    fn get_from_dest(&self) -> ((u8, u8), (u8, u8)) {
        (self.from_rc, self.dest_rc)
    }
    
}

impl Display for SimpleMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            (b'A' + self.from_rc.1) as char,
            self.from_rc.0 + 1,
            (b'A' + self.dest_rc.1) as char,
            self.dest_rc.0 + 1
        )
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct SimpleBoard {
    current_player: Player,
    pub rows: [[Option<(Player, PieceKind)>; 7]; 9],
    pub eval: i32,
}

impl Display for SimpleBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (row_idx, row) in self.rows.iter().rev().enumerate() {
            writeln!(
                f,
                "{}. {}",
                9 - row_idx,
                row.iter()
                    .enumerate()
                    .map(|(col_idx, val)| {
                        let piece_str = match val {
                                Some((Player::PlayerOne, PieceKind::B)) => " B ",
                                Some((Player::PlayerOne, PieceKind::K)) => " K ",
                                Some((Player::PlayerOne, PieceKind::N)) => " N ",
                                Some((Player::PlayerOne, PieceKind::R)) => " R ",
                                Some((Player::PlayerOne, PieceKind::P)) => " P ",
                                Some((Player::PlayerTwo, PieceKind::B)) => " ♗ ",
                                Some((Player::PlayerTwo, PieceKind::K)) => " ♔ ",
                                Some((Player::PlayerTwo, PieceKind::N)) => " ♘ ",
                                Some((Player::PlayerTwo, PieceKind::R)) => " ♖ ",
                                Some((Player::PlayerTwo, PieceKind::P)) => " ♙ ",
                                None => "   ",
                        };
                        match (row_idx + col_idx) % 2 {
                            0 => ansi_term::Color::White.on(ansi_term::Color::RGB(64, 64, 64)).bold(),
                            _ => ansi_term::Color::White.on(ansi_term::Color::RGB(32, 32, 32)).bold(),
                        }
                        
                        .paint(piece_str)
                        
                        .to_string()
                    })
                    .collect::<Vec<String>>()
                    .join("")
            )?;
        }
        writeln!(f, "\n    A  B  C  D  E  F  G ")
    }
}

impl SimpleBoard {
    pub fn init() -> Self {
        SimpleBoard {
            current_player: Player::PlayerOne,
            rows: [
                ['-', 'N', 'R', 'K', 'R', 'N', '-'],
                ['-', '-', '-', 'B', '-', '-', '-'],
                ['-', '-', '-', 'B', '-', '-', '-'],
                ['P', '-', 'P', '-', 'P', '-', 'P'],
                ['-', '-', '-', '-', '-', '-', '-'],
                ['p', '-', 'p', '-', 'p', '-', 'p'],
                ['-', '-', '-', 'b', '-', '-', '-'],
                ['-', '-', '-', 'b', '-', '-', '-'],
                ['-', 'n', 'r', 'k', 'r', 'n', '-'],
            ]
            .map(|row| {
                row.map(|ch| match ch {
                    'B' => Some((Player::PlayerOne, PieceKind::B)),
                    'K' => Some((Player::PlayerOne, PieceKind::K)),
                    'N' => Some((Player::PlayerOne, PieceKind::N)),
                    'R' => Some((Player::PlayerOne, PieceKind::R)),
                    'P' => Some((Player::PlayerOne, PieceKind::P)),
                    'b' => Some((Player::PlayerTwo, PieceKind::B)),
                    'k' => Some((Player::PlayerTwo, PieceKind::K)),
                    'n' => Some((Player::PlayerTwo, PieceKind::N)),
                    'r' => Some((Player::PlayerTwo, PieceKind::R)),
                    'p' => Some((Player::PlayerTwo, PieceKind::P)),
                    _ => None,
                })
            }),
            eval: 0,
        }
    }

    fn raycast_moves(
        &self,
        player: &Player,
        pos: (u8, u8),
        del: (i8, i8),
    ) -> (Vec<(u8, u8)>, Option<(u8, u8)>) {
        let mut cur = pos;
        let mut ret = vec![];
        while let Ok(nxt) = check_pos(((cur.0 as i8 + del.0) as u8, (cur.1 as i8 + del.1) as u8)) {
            cur = nxt;
            match self.rows[cur.0 as usize][cur.1 as usize] {
                Some((other_player, _)) if *player == other_player => return (ret, None),
                Some((_, _)) => {
                    ret.push(cur);
                    return (ret, Some(cur));
                }
                None => ret.push(cur),
            };
        }
        (ret, None)
    }

    fn get_bishop_moves(&self, player: &Player, pos: (u8, u8)) -> Vec<(u8, u8)> {
        let mut ret = vec![pos];
        ret.extend(self.raycast_moves(player, pos, (player.parity() * 1, 1)).0);
        ret.extend(self.raycast_moves(player, pos, (player.parity() * 1, -1)).0);
        if let Some(attack_move) = self.raycast_moves(player, pos, (player.parity() * -1, 1)).1 {
            ret.push(attack_move)
        }
        if let Some(attack_move) = self
            .raycast_moves(player, pos, (player.parity() * -1, -1))
            .1
        {
            ret.push(attack_move)
        }
        ret
    }

    fn get_king_moves(&self, player: &Player, pos: (u8, u8)) -> Vec<(u8, u8)> {
        let fwd: Vec<(u8, u8)> = [
            ((pos.0 as i8 + player.parity() * 1) as u8, pos.1),
            (
                (pos.0 as i8 + player.parity() * 1) as u8,
                ((pos.1 as i8) - 1) as u8,
            ),
            ((pos.0 as i8 + player.parity() * 1) as u8, pos.1 + 1),
        ]
        .iter()
        .filter(|&nxt| check_pos(*nxt).is_ok())
        .filter(|&nxt| match self.rows[nxt.0 as usize][nxt.1 as usize] {
            Some((other_player, _)) if *player == other_player => false,
            _ => true,
        })
        .map(|&a| a)
        .collect();
        let mut ret = vec![pos];
        ret.extend(fwd);
        ret
    }

    fn get_pawn_moves(&self, player: &Player, pos: (u8, u8)) -> Vec<(u8, u8)> {
        let mut ret = vec![pos];
        if let Ok(nxt) = check_pos((((pos.0 as i8) + player.parity()) as u8, pos.1)) {
            if self.rows[nxt.0 as usize][nxt.1 as usize].is_none() {
                ret.push(nxt);
            }
        }
        let atk: Vec<(u8, u8)> = [(1, 1), (1, -1)]
            .iter()
            .map(|del| {
                (
                    (pos.0 as i8 + player.parity() * del.0) as u8,
                    (pos.1 as i8 + del.1) as u8,
                )
            })
            .filter(|nxt| check_pos(*nxt).is_ok())
            .filter(|nxt| match self.rows[nxt.0 as usize][nxt.1 as usize] {
                Some((other_player, _)) if other_player != *player => true,
                _ => false,
            })
            .collect();
        ret.extend(atk);
        ret
    }

    fn get_rook_moves(&self, player: &Player, pos: (u8, u8)) -> Vec<(u8, u8)> {
        let mut ret = vec![pos];
        ret.extend(self.raycast_moves(player, pos, (player.parity() * 1, 0)).0);
        ret.extend(self.raycast_moves(player, pos, (0, 1)).0);
        ret.extend(self.raycast_moves(player, pos, (0, -1)).0);
        if let Some(attack_move) = self.raycast_moves(player, pos, (player.parity() * -1, 0)).1 {
            ret.push(attack_move)
        }
        ret
    }

    fn get_knight_moves(&self, player: &Player, pos: (u8, u8)) -> Vec<(u8, u8)> {
        let fwd: Vec<(u8, u8)> = [(2, 1), (2, -1), (1, 2), (1, -2)]
            .iter()
            .map(|del| {
                (
                    (pos.0 as i8 + player.parity() * del.0) as u8,
                    (pos.1 as i8 + del.1) as u8,
                )
            })
            .filter(|nxt| check_pos(*nxt).is_ok())
            .filter(|nxt| match self.rows[nxt.0 as usize][nxt.1 as usize] {
                Some((other_player, _)) if other_player == *player => false,
                _ => true,
            })
            .collect();

        let bwd: Vec<(u8, u8)> = [(-2, 1), (-2, -1), (-1, 2), (-1, -2)]
            .iter()
            .map(|del| {
                (
                    (pos.0 as i8 + player.parity() * del.0) as u8,
                    (pos.1 as i8 + del.1) as u8,
                )
            })
            .filter(|nxt| check_pos(*nxt).is_ok())
            .filter(|nxt| match self.rows[nxt.0 as usize][nxt.1 as usize] {
                Some((other_player, _)) if other_player != *player => true,
                _ => false,
            })
            .collect();

        let mut ret = vec![pos];
        ret.extend(fwd);
        ret.extend(bwd);
        ret
    }
}

fn check_pos(pos: (u8, u8)) -> Result<(u8, u8), MoveError> {
    if pos.0 >= 9 || pos.1 >= 7 {
        return Err(MoveError::InvalidPosition);
    }
    Ok(pos)
}

impl Board<SimpleMove> for SimpleBoard {

    fn get_piece(&self, row: u8, col: u8) -> Option<(Player, PieceKind)> {
        self.rows[row as usize][col as usize]
    }

    fn get_player(&self) -> Player {
        self.current_player
    }

    

    fn get_moves(&self) -> Vec<SimpleMove> {
        self.rows
            .iter()
            .flatten()
            .enumerate()
            .filter(|(_, &piece)| match piece {
                Some((piece_player, _)) if piece_player == self.current_player => true,
                _ => false,
            })
            .map(|(pos, piece)| (((pos / 7) as u8, (pos % 7) as u8), piece))
            .map(|(pos, piece)| match piece {
                Some((player, kind)) => match kind {
                    PieceKind::B => self.get_bishop_moves(player, pos),
                    PieceKind::K => self.get_king_moves(player, pos),
                    PieceKind::N => self.get_knight_moves(player, pos),
                    PieceKind::P => self.get_pawn_moves(player, pos),
                    PieceKind::R => self.get_rook_moves(player, pos),
                    // _ => vec![],
                }
                .iter()
                .map(|&nxt| SimpleMove {
                    from_rc: pos,
                    dest_rc: nxt,
                })
                .collect(),
                _ => vec![],
            })
            .flatten()
            .collect::<Vec<SimpleMove>>()
    }

    fn get_winner(&self) -> Option<Player> {
        if self.rows.iter().flatten().all(|piece| match piece {
            Some((_, PieceKind::K)) => false,
            _ => true,
        }) {
            return Some(self.current_player);
        }

        if !self.rows.iter().flatten().any(|piece| match piece {
            Some((Player::PlayerOne, PieceKind::K)) => true,
            _ => false,
        }) {
            return Some(Player::PlayerTwo);
        }
        if !self.rows.iter().flatten().any(|piece| match piece {
            Some((Player::PlayerTwo, PieceKind::K)) => true,
            _ => false,
        }) {
            return Some(Player::PlayerOne);
        }
        if self.get_moves().is_empty() {
            return Some(self.current_player.other());
        }
        None
    }

    fn do_move(&mut self, mov: &SimpleMove) {
        check_pos(mov.from_rc).unwrap();
        check_pos(mov.dest_rc).unwrap();
        if mov.from_rc == mov.dest_rc {
            // Explode!!
            for clear_row in [
                (mov.from_rc.0 as i8 - 1) as u8,
                mov.from_rc.0,
                mov.from_rc.0 + 1,
            ] {
                for clear_col in [
                    (mov.from_rc.1 as i8 - 1) as u8,
                    mov.from_rc.1,
                    mov.from_rc.1 + 1,
                ] {
                    if check_pos((clear_row, clear_col)).is_ok() {
                        if let Some((old_player, old_kind)) = self.rows[clear_row as usize][clear_col as usize] {
                            self.eval -= (old_player.parity() as i32) * (piece_rank(&old_kind) as i32);
                        }
                        self.rows[clear_row as usize][clear_col as usize] = None;
                    }
                }
            }
            self.current_player = self.current_player.other();
            return;
        }
        // don't validate. w/e.
        if let Some((old_player, old_kind)) = self.rows[mov.dest_rc.0 as usize][mov.dest_rc.1 as usize] {
            self.eval -= (old_player.parity() as i32) * (piece_rank(&old_kind) as i32);
        } 
        self.rows[mov.dest_rc.0 as usize][mov.dest_rc.1 as usize] =
            self.rows[mov.from_rc.0 as usize][mov.from_rc.1 as usize];
        self.rows[mov.from_rc.0 as usize][mov.from_rc.1 as usize] = None;
        self.current_player = self.current_player.other();
    }

    fn invert(&self) -> Self {
        todo!()
    }
}

mod test {
    use super::*;

    #[test]
    fn opening_board_moves() {
        let board = SimpleBoard::init();
        assert_eq!(
            board
                .get_moves()
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<String>>(),
            vec![
                "B1B1", "B1C3", "B1A3", "C1C1", "C1C2", "C1C3", "D1D1", "D1C2", "D1E2", "E1E1",
                "E1E2", "E1E3", "F1F1", "F1G3", "F1E3", "D2D2", "D2E3", "D2F4", "D2G5", "D2C3",
                "D2B4", "D2A5", "D3D3", "A4A4", "A4A5", "C4C4", "C4C5", "E4E4", "E4E5", "G4G4",
                "G4G5",
            ]
        )
    }
}
