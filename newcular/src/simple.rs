use std::fmt::{Debug, Display};

use crate::board::*;

#[derive(Clone, Copy, Eq, Debug, PartialEq)]
pub struct SimpleMov {
    from_rc: (u8, u8),
    dest_rc: (u8, u8),
}

impl Display for SimpleMov {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SimplePla {
    PlayerOne,
    PlayerTwo,
}

impl SimplePla {
    fn other(&self) -> SimplePla {
        match self {
            Self::PlayerOne => Self::PlayerTwo,
            Self::PlayerTwo => Self::PlayerOne,
        }
    }
    fn parity(&self) -> i8 {
        match self {
            Self::PlayerOne => 1,
            Self::PlayerTwo => -1,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct SimpleBoard {
    current_player: SimplePla,
    rows: [[Option<(SimplePla, PieceKind)>; 7]; 9],
}

impl Display for SimpleBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut drawn = String::new();
        for (row_idx, row) in self.rows.iter().rev().enumerate() {
            writeln!(
                f,
                "{}. {}",
                9 - row_idx,
                row.iter()
                .enumerate()
                    .map(|(col_idx, val)| {
                        let piece_str = match val {
                        Some((SimplePla::PlayerOne, PieceKind::B)) => " B ",
                        Some((SimplePla::PlayerOne, PieceKind::K)) => " K ",
                        Some((SimplePla::PlayerOne, PieceKind::N)) => " N ",
                        Some((SimplePla::PlayerOne, PieceKind::R)) => " R ",
                        Some((SimplePla::PlayerOne, PieceKind::P)) => " P ",
                        Some((SimplePla::PlayerTwo, PieceKind::B)) => " b ",
                        Some((SimplePla::PlayerTwo, PieceKind::K)) => " k ",
                        Some((SimplePla::PlayerTwo, PieceKind::N)) => " n ",
                        Some((SimplePla::PlayerTwo, PieceKind::R)) => " r ",
                        Some((SimplePla::PlayerTwo, PieceKind::P)) => " p ",
                        None => " - ",
                        };
                        match (row_idx + col_idx) % 2 {
                            0 => ansi_term::Color::Black.on(ansi_term::Color::Cyan),
                            _ => ansi_term::Color::Black.on(ansi_term::Color::White),
                        }.paint(piece_str).to_string()
                    })
                    .collect::<Vec<String>>()
                    .join("")
            )?;
        }
        writeln!(f, "{}\n    A  B  C  D  E  F  G ", drawn)
    }
}

impl SimpleBoard {
    pub fn init() -> Self {
        SimpleBoard {
            current_player: SimplePla::PlayerOne,
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
                    'B' => Some((SimplePla::PlayerOne, PieceKind::B)),
                    'K' => Some((SimplePla::PlayerOne, PieceKind::K)),
                    'N' => Some((SimplePla::PlayerOne, PieceKind::N)),
                    'R' => Some((SimplePla::PlayerOne, PieceKind::R)),
                    'P' => Some((SimplePla::PlayerOne, PieceKind::P)),
                    'b' => Some((SimplePla::PlayerTwo, PieceKind::B)),
                    'k' => Some((SimplePla::PlayerTwo, PieceKind::K)),
                    'n' => Some((SimplePla::PlayerTwo, PieceKind::N)),
                    'r' => Some((SimplePla::PlayerTwo, PieceKind::R)),
                    'p' => Some((SimplePla::PlayerTwo, PieceKind::P)),
                    _ => None,
                })
            }),
        }
    }

    fn raycast_moves(
        &self,
        player: &SimplePla,
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

    fn get_bishop_moves(&self, player: &SimplePla, pos: (u8, u8)) -> Vec<(u8, u8)> {
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

    fn get_king_moves(&self, player: &SimplePla, pos: (u8, u8)) -> Vec<(u8, u8)> {
        vec![
            pos,
            ((pos.0 as i8 + player.parity() * 1) as u8, pos.1),
            ((pos.0 as i8 + player.parity() * 1) as u8, pos.1 - 1),
            ((pos.0 as i8 + player.parity() * 1) as u8, pos.1 + 1),
        ]
        .iter()
        .filter(|&nxt| check_pos(*nxt).is_ok())
        .map(|&a| a)
        .collect()
    }

    fn get_pawn_moves(&self, player: &SimplePla, pos: (u8, u8)) -> Vec<(u8, u8)> {
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

    fn get_rook_moves(&self, player: &SimplePla, pos: (u8, u8)) -> Vec<(u8, u8)> {
        let mut ret = vec![pos];
        ret.extend(self.raycast_moves(player, pos, (player.parity() * 1, 0)).0);
        ret.extend(self.raycast_moves(player, pos, (0, 1)).0);
        ret.extend(self.raycast_moves(player, pos, (0, -1)).0);
        if let Some(attack_move) = self.raycast_moves(player, pos, (player.parity() * -1, 0)).1 {
            ret.push(attack_move)
        }
        ret
    }

    fn get_knight_moves(&self, player: &SimplePla, pos: (u8, u8)) -> Vec<(u8, u8)> {
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

impl Board<SimpleMov, SimplePla> for SimpleBoard {
    fn get_player(&self) -> SimplePla {
        self.current_player
    }

    fn get_moves(&self) -> Vec<SimpleMov> {
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
                .map(|&nxt| SimpleMov {
                    from_rc: pos,
                    dest_rc: nxt,
                })
                .collect(),
                _ => vec![],
            })
            .flatten()
            .collect::<Vec<SimpleMov>>()
    }

    fn get_winner(&self) -> Option<SimplePla> {
        if !self.rows.iter().flatten().any(|piece| match piece {
            Some((SimplePla::PlayerOne, PieceKind::K)) => true,
            _ => false,
        }) {
            return Some(SimplePla::PlayerTwo);
        }
        if !self.rows.iter().flatten().any(|piece| match piece {
            Some((SimplePla::PlayerTwo, PieceKind::K)) => true,
            _ => false,
        }) {
            return Some(SimplePla::PlayerOne);
        }
        if self.get_moves().is_empty() {
            return Some(self.current_player.other());
        }
        None
    }

    fn do_move(&mut self, mov: &SimpleMov) -> Result<(), MoveError> {
        check_pos(mov.from_rc)?;
        check_pos(mov.dest_rc)?;
        if mov.from_rc == mov.dest_rc {
            // Explode!!
            for clear_row in [(mov.from_rc.0 as i8 - 1) as u8, mov.from_rc.0, mov.from_rc.0 + 1] {
                for clear_col in [(mov.from_rc.1 as i8 - 1) as u8, mov.from_rc.1, mov.from_rc.1 + 1] {
                    if check_pos((clear_row, clear_col)).is_ok() {
                        self.rows[clear_row as usize][clear_col as usize] = None;
                    }
                }
            }
        }
        // don't validate. w/e.
        self.rows[mov.dest_rc.0 as usize][mov.dest_rc.1 as usize] =
            self.rows[mov.from_rc.0 as usize][mov.from_rc.1 as usize];
        self.rows[mov.from_rc.0 as usize][mov.from_rc.1 as usize] = None;
        self.current_player = self.current_player.other();
        Ok(())
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
                "B1B1", "B1C3", "B1A3", "C1C1", "C1C2", "C1C3", "D1D1", "D1D2", "D1C2", "D1E2",
                "E1E1", "E1E2", "E1E3", "F1F1", "F1G3", "F1E3", "D2D2", "D2E3", "D2F4", "D2G5",
                "D2C3", "D2B4", "D2A5", "D3D3", "A4A4", "A4A5", "C4C4", "C4C5", "E4E4", "E4E5",
                "G4G4", "G4G5",
            ]
        )
    }
}
