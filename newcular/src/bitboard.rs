use std::{fmt::{Debug, Display}, ops::Shr};

use crate::board::*;

#[derive(Clone, Copy, Eq, Debug, PartialEq)]
pub struct BitBoardMove {
    from_pos: u8,
    dest_pos: u8,
}

impl BitBoardMove {
    pub fn from_from_dest(from_pos: (u8, u8), dest_pos: (u8, u8)) -> Option<BitBoardMove> {
        Some(BitBoardMove {
            from_pos: 7 * from_pos.0 + from_pos.1,
            dest_pos: 7 * dest_pos.0 + dest_pos.1,
        })
    }
}

impl Display for BitBoardMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            (b'A' + self.from_pos % 7) as char,
            (self.from_pos / 7) + 1,
            (b'A' + self.dest_pos % 7) as char,
            (self.dest_pos / 7) + 1
        )
    }
}

impl Mov for BitBoardMove {
    fn invert(&self) -> Self {
        let from_row = self.from_pos / 7;
        let from_col = self.from_pos % 7;
        let dest_row = self.dest_pos / 7;
        let dest_col = self.dest_pos % 7;
        BitBoardMove {
            from_pos: 7 * (8 - from_row) + from_col,
            dest_pos: 7 * (8 - dest_row) + dest_col,
        }
    }
    fn get_from_dest(&self) -> ((u8, u8), (u8, u8)) {
        (
            (self.from_pos / 7, self.from_pos % 7),
            (self.dest_pos / 7, self.dest_pos % 7),
        )
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct BitBoard {
    current_player: Player,
    pub piece_mask: u64,
    pub player_one_mask: u64,
    pub bishop_mask: u64,
    pub king_mask: u64,
    pub knight_mask: u64,
    pub rook_mask: u64,
    pub pawn_mask: u64,
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let drawn = String::new();

        for row_idx in 0..9 {
            writeln!(
                f,
                "{}. {}",
                9 - row_idx,
                (0..7)
                    .map(|col_idx| {
                        let piece_str = match self.get_piece(8 - row_idx, col_idx) {
                            Some((Player::PlayerOne, PieceKind::B)) => " B ",
                            Some((Player::PlayerOne, PieceKind::K)) => " K ",
                            Some((Player::PlayerOne, PieceKind::N)) => " N ",
                            Some((Player::PlayerOne, PieceKind::R)) => " R ",
                            Some((Player::PlayerOne, PieceKind::P)) => " P ",
                            Some((Player::PlayerTwo, PieceKind::B)) => " b ",
                            Some((Player::PlayerTwo, PieceKind::K)) => " k ",
                            Some((Player::PlayerTwo, PieceKind::N)) => " n ",
                            Some((Player::PlayerTwo, PieceKind::R)) => " r ",
                            Some((Player::PlayerTwo, PieceKind::P)) => " p ",
                            None => " - ",
                        };
                        match (row_idx + col_idx) % 2 {
                            0 => ansi_term::Color::Black.on(ansi_term::Color::Cyan),
                            _ => ansi_term::Color::Black.on(ansi_term::Color::White),
                        }
                        .paint(piece_str)
                        .to_string()
                    })
                    .collect::<Vec<String>>()
                    .join("")
            )?;
        }
        writeln!(f, "{}\n    A  B  C  D  E  F  G ", drawn)
    }
}

#[inline]
fn nort(x: u64) -> u64 {
    (x & 0x00ffffffffffffff) << 7
}

// #[inline]
// fn sout(x: u64) -> u64 {
//     (x & 0x7FFFFFFFFFFFFFFF) >> 7
// }

#[inline]
fn east(x: u64) -> u64 {
    (x & 0b0011111101111110111111011111101111110111111011111101111110111111) << 1
}

#[inline]
fn west(x: u64) -> u64 {
    (x & 0b0111111011111101111110111111011111101111110111111011111101111110) >> 1
}

#[inline]
fn do_move_up(board: &mut u64, from_mask: u64, dest_mask: u64, up_shift: u8) {
    *board = ((*board & from_mask) << up_shift) | (*board & !from_mask & !dest_mask);
}

#[inline]
fn do_move_down(board: &mut u64, from_mask: u64, dest_mask: u64, down_shift: u8) {
    *board = ((*board & from_mask) >> down_shift) | (*board & !from_mask & !dest_mask);
}

/*
static U64 FULL_BOARD = (1ULL << 56) - 1;
static U64 FlipVert(U64 state) {
  const U64 k1 = 0x01fc07f01fc07fULL;
  const U64 k2 = 0x0003fff0003fffULL;
  U64 x = state;
  x = ((x >>  7) & k1) | ((x & k1) <<  7);
  x &= FULL_BOARD;
  x = ((x >> 14) & k2) | ((x & k2) << 14);
  x &= FULL_BOARD;
  x = ( x >> 28)       | ( x       << 28);
  x &= FULL_BOARD;
  return x;
}
 */

#[inline]
fn flip_vertical(x: u64) -> u64 {
    let k1 = 0b0000000011111110000000111111100000000000000111111100000001111111u64;
    let k2 = 0b0000000000000001111111111111100000000000000000000011111111111111u64;
    let k3 = 0b0000000000000000000000000000011111110000000000000000000000000000u64;
    let x = (((x >> 7) & k1) | ((x & k1) << 7) | (x & k3)) & 0x7FFFFFFFFFFFFFFF;
    let x = (((x >> 14) & k2) | ((x & k2) << 14) | (x & k3)) & 0x7FFFFFFFFFFFFFFF;
    ((x >> 35) | (x << 35) | (x & k3)) & 0x7FFFFFFFFFFFFFFF
}

#[inline]
fn project_fwd(state: &BitBoard, pos: u64, mask: u64, delta: u64) -> u64 {
    let mut positions = 0u64;
    // See https://go.mattmerr.com/bitboardhex
    positions |= (((positions & !state.piece_mask) | pos) & mask) << delta;
    positions |= (((positions & !state.piece_mask) | pos) & mask) << delta;
    positions |= (((positions & !state.piece_mask) | pos) & mask) << delta;
    positions |= (((positions & !state.piece_mask) | pos) & mask) << delta;
    positions |= (((positions & !state.piece_mask) | pos) & mask) << delta;
    positions |= (((positions & !state.piece_mask) | pos) & mask) << delta;
    positions |= (((positions & !state.piece_mask) | pos) & mask) << delta;
    positions |= (((positions & !state.piece_mask) | pos) & mask) << delta;
    positions &= !(state.piece_mask & state.player_one_mask);

    positions
}

#[inline]
fn project_bwd(state: &BitBoard, pos: u64, mask: u64, delta: u64) -> u64 {
    let mut positions = 0u64;
    // See https://go.mattmerr.com/bitboardhex
    positions |= (((positions & !state.piece_mask) | pos) & mask) >> delta;
    positions |= (((positions & !state.piece_mask) | pos) & mask) >> delta;
    positions |= (((positions & !state.piece_mask) | pos) & mask) >> delta;
    positions |= (((positions & !state.piece_mask) | pos) & mask) >> delta;
    positions |= (((positions & !state.piece_mask) | pos) & mask) >> delta;
    positions |= (((positions & !state.piece_mask) | pos) & mask) >> delta;
    positions |= (((positions & !state.piece_mask) | pos) & mask) >> delta;
    positions |= (((positions & !state.piece_mask) | pos) & mask) >> delta;
    positions &= !(state.piece_mask & state.player_one_mask);

    positions
}

impl BitBoard {
    pub fn init() -> Self {
        BitBoard {
            current_player: Player::PlayerOne,
            piece_mask: 0b0011111000010000001000101010100000001010101000100000010000111110,
            player_one_mask: 0b0000000000000000000000000000000000001010101000100000010000111110,
            bishop_mask: 0b0000000000010000001000000000000000000000000000100000010000000000,
            king_mask: 0b0000100000000000000000000000000000000000000000000000000000001000,
            knight_mask: 0b0010001000000000000000000000000000000000000000000000000000100010,
            rook_mask: 0b0001010000000000000000000000000000000000000000000000000000010100,
            pawn_mask: 0b0000000000000000000000101010100000001010101000100000000000000000,
        }
    }

    #[inline]
    fn board_array_representation(&self, board: u64) -> [[u8;7];9] {
        let mut ret: [[u8;7];9] = [[0;7]; 9];
        for row_idx in 0..9 {
            for col_idx in 0..7 {
                ret[row_idx][col_idx] = ((board >> (7 * row_idx + col_idx)) & 1) as u8;
            }
        }
        ret
    }

    pub fn array_representation(&self) -> [[[u8;7];9];7] {
        [
            self.board_array_representation(self.piece_mask),
            self.board_array_representation(self.player_one_mask),
            self.board_array_representation(self.bishop_mask),
            self.board_array_representation(self.king_mask),
            self.board_array_representation(self.knight_mask),
            self.board_array_representation(self.pawn_mask),
            self.board_array_representation(self.rook_mask),
        ]
    }

    // tbh having code for either scenario seems like it would be faster
    // fn invert(&mut self) -> Self {
    //     self.piece_mask = flip_vertical(self.piece_mask);
    //     self.piece_mask = flip_vertical(self.piece_mask);
    //     self.player_one_mask = self.piece_mask;
    // }

    // fn raycast_moves(
    //     &self,
    //     player: &Player,
    //     pos: (u8, u8),
    //     del: (i8, i8),
    // ) -> (Vec<(u8, u8)>, Option<(u8, u8)>) {
    //     let mut cur = pos;
    //     let mut ret = vec![];
    //     while let Ok(nxt) = check_pos(((cur.0 as i8 + del.0) as u8, (cur.1 as i8 + del.1) as u8)) {
    //         cur = nxt;
    //         match self.rows[cur.0 as usize][cur.1 as usize] {
    //             Some((other_player, _)) if *player == other_player => return (ret, None),
    //             Some((_, _)) => {
    //                 ret.push(cur);
    //                 return (ret, Some(cur));
    //             }
    //             None => ret.push(cur),
    //         };
    //     }
    //     (ret, None)
    // }

    #[inline]
    fn get_bishop_moves(&self, pos: u64) -> u64 {
        // See https://go.mattmerr.com/bitboardhex
        let mut positions = 0u64;
        // Attacks
        positions |= project_bwd(self, pos, 0x7efdfbf7efdfbf00u64, 8);
        positions |= project_bwd(self, pos, 0x3f7efdfbf7efdf80u64, 6);
        positions &= self.piece_mask & !self.player_one_mask;

        positions |= project_fwd(self, pos, 0x007efdfbf7efdfbfu64, 8);
        positions |= project_fwd(self, pos, 0x00fdfbf7efdfbf7eu64, 6);

        positions | pos
    }

    fn get_king_moves(&self, pos: u64) -> u64 {
        let mut ret = 0u64;
        let nort = nort(pos);
        ret |= nort & !self.player_one_mask;
        ret |= (west(nort) | east(nort)) & !(self.piece_mask & self.player_one_mask);
        ret | pos
    }

    fn get_pawn_moves(&self, pos: u64) -> u64 {
        let mut ret = pos;
        let nort = nort(pos);
        ret |= nort & !self.piece_mask;
        ret |= (west(nort) | east(nort)) & (self.piece_mask & !self.player_one_mask);
        ret
    }

    fn get_rook_moves(&self, pos: u64) -> u64 {
        let mut positions = 0u64;

        // Attacks
        positions |= project_fwd(self, pos, 0x3f7efdfbf7efdfbfu64, 1);
        positions |= project_bwd(self, pos, 0x7efdfbf7efdfbf7eu64, 1);
        positions |= project_bwd(self, pos, 0x7fffffffffffff80u64, 7);
        positions &= self.piece_mask & !self.player_one_mask;

        positions |= project_fwd(self, pos, 0x00ffffffffffffffu64, 7);
        positions | pos
    }

    fn get_knight_moves(&self, pos: u64) -> u64 {
        let mut positions = 0u64;

        // See https://go.mattmerr.com/bitboardhex
        // Backwards attacks
        positions |= (pos & 0x7cf9f3e7cf9f3e00) >> 1 * 7 + 2;
        positions |= (pos & 0x1f3e7cf9f3e7cf80) >> 1 * 7 - 2;
        positions |= (pos & 0x7efdfbf7efdf8000) >> 2 * 7 + 1;
        positions |= (pos & 0x3f7efdfbf7efc000) >> 2 * 7 - 1;
        positions &= self.piece_mask & !self.player_one_mask;

        // Forward moves
        positions |= (pos & 0x003e7cf9f3e7cf9f) << 1 * 7 + 2;
        positions |= (pos & 0x00f9f3e7cf9f3e7c) << 1 * 7 - 2;
        positions |= (pos & 0x0000fdfbf7efdfbf) << 2 * 7 + 1;
        positions |= (pos & 0x0001fbf7efdfbf7e) << 2 * 7 - 1;
        positions &= !(self.piece_mask & self.player_one_mask);

        positions | pos
    }
}

impl Board<BitBoardMove> for BitBoard {
    fn get_piece(&self, row_idx: u8, col_idx: u8) -> Option<(Player, PieceKind)> {
        let hot_bit = 1u64 << (row_idx * 7 + col_idx);
        if (self.piece_mask & hot_bit) == 0 {
            return None;
        }
        Some((
            match self.player_one_mask & hot_bit {
                0 => Player::PlayerTwo,
                _ => Player::PlayerOne,
            },
            match 1 {
                _ if (self.bishop_mask & hot_bit) != 0 => PieceKind::B,
                _ if (self.king_mask & hot_bit) != 0 => PieceKind::K,
                _ if (self.knight_mask & hot_bit) != 0 => PieceKind::N,
                _ if (self.pawn_mask & hot_bit) != 0 => PieceKind::P,
                _ if (self.rook_mask & hot_bit) != 0 => PieceKind::R,
                _ => unimplemented!(),
            },
        ))
    }
    fn get_player(&self) -> Player {
        self.current_player
    }

    fn get_moves(&self) -> Vec<BitBoardMove> {
        if self.current_player != Player::PlayerOne {
            let inv = self.invert();
            return inv.get_moves().into_iter().map(|m| m.invert()).collect();
        }
        let mut unconsidered = self.piece_mask & self.player_one_mask;
        let mut ret = vec![];
        while unconsidered > 0 {
            let from_pos = unconsidered.trailing_zeros() as u8;
            let from_mask = 1u64 << from_pos;

            let mut dest_positions = match 1 {
                _ if (self.bishop_mask & from_mask) != 0 => self.get_bishop_moves(from_mask),
                _ if (self.king_mask & from_mask) != 0 => self.get_king_moves(from_mask),
                _ if (self.knight_mask & from_mask) != 0 => self.get_knight_moves(from_mask),
                _ if (self.pawn_mask & from_mask) != 0 => self.get_pawn_moves(from_mask),
                _ if (self.rook_mask & from_mask) != 0 => self.get_rook_moves(from_mask),
                _ => unimplemented!(),
            };

            while dest_positions > 0 {
                let dest_pos = dest_positions.trailing_zeros() as u8;
                ret.push(BitBoardMove { from_pos, dest_pos });
                dest_positions ^= 1u64 << dest_pos;
            }

            unconsidered ^= from_mask;
        }
        ret
    }

    fn get_winner(&self) -> Option<Player> {
        if self.king_mask == 0 {
            return Some(self.current_player);
        }
        if (self.piece_mask & self.player_one_mask & self.king_mask) == 0 {
            return Some(Player::PlayerTwo);
        }
        if (self.piece_mask & !self.player_one_mask & self.king_mask) == 0 {
            return Some(Player::PlayerOne);
        }
        if self.get_moves().is_empty() {
            return Some(self.current_player.other());
        }
        None
    }

    fn do_move(&mut self, mov: &BitBoardMove) {
        if mov.from_pos == mov.dest_pos {
            // Explode!!
            let mut mask = 1u64 << mov.from_pos;
            mask |= (mask & 0x00ffffffffffffffu64) << 7;
            mask |= (mask & 0x7fffffffffffff80u64) >> 7;
            mask |= (mask & 0x3f7efdfbf7efdfbfu64) << 1;
            mask |= (mask & 0x7efdfbf7efdfbf7eu64) >> 1;
            self.piece_mask &= !mask;
            self.player_one_mask &= !mask;
            self.bishop_mask &= !mask;
            self.king_mask &= !mask;
            self.knight_mask &= !mask;
            self.pawn_mask &= !mask;
            self.rook_mask &= !mask;
        } else if mov.from_pos < mov.dest_pos {
            let from_mask = 1u64 << mov.from_pos;
            let dest_mask = 1u64 << mov.dest_pos;
            let up_shift = mov.dest_pos - mov.from_pos;
            do_move_up(&mut self.piece_mask, from_mask, dest_mask, up_shift);
            do_move_up(&mut self.player_one_mask, from_mask, dest_mask, up_shift);
            do_move_up(&mut self.bishop_mask, from_mask, dest_mask, up_shift);
            do_move_up(&mut self.king_mask, from_mask, dest_mask, up_shift);
            do_move_up(&mut self.knight_mask, from_mask, dest_mask, up_shift);
            do_move_up(&mut self.pawn_mask, from_mask, dest_mask, up_shift);
            do_move_up(&mut self.rook_mask, from_mask, dest_mask, up_shift);
        } else {
            let from_mask = 1u64 << mov.from_pos;
            let dest_mask = 1u64 << mov.dest_pos;
            let down_shift = mov.from_pos - mov.dest_pos;
            do_move_down(&mut self.piece_mask, from_mask, dest_mask, down_shift);
            do_move_down(&mut self.player_one_mask, from_mask, dest_mask, down_shift);
            do_move_down(&mut self.bishop_mask, from_mask, dest_mask, down_shift);
            do_move_down(&mut self.king_mask, from_mask, dest_mask, down_shift);
            do_move_down(&mut self.knight_mask, from_mask, dest_mask, down_shift);
            do_move_down(&mut self.pawn_mask, from_mask, dest_mask, down_shift);
            do_move_down(&mut self.rook_mask, from_mask, dest_mask, down_shift);
        }
        self.current_player = self.current_player.other();
    }

    fn invert(&self) -> BitBoard {
        return BitBoard {
            current_player: self.current_player.other(),
            piece_mask: flip_vertical(self.piece_mask),
            player_one_mask: flip_vertical(self.piece_mask ^ self.player_one_mask),
            bishop_mask: flip_vertical(self.bishop_mask),
            king_mask: flip_vertical(self.king_mask),
            knight_mask: flip_vertical(self.knight_mask),
            rook_mask: flip_vertical(self.rook_mask),
            pawn_mask: flip_vertical(self.pawn_mask),
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn opening_board_moves() {
        let board = BitBoard::init();
        let mut moves = board
            .get_moves()
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<String>>();
        moves.sort();
        let mut expected = vec![
            "B1B1", "B1C3", "B1A3", "C1C1", "C1C2", "C1C3", "D1D1", "D1C2", "D1E2", "E1E1", "E1E2",
            "E1E3", "F1F1", "F1G3", "F1E3", "D2D2", "D2E3", "D2F4", "D2G5", "D2C3", "D2B4", "D2A5",
            "D3D3", "A4A4", "A4A5", "C4C4", "C4C5", "E4E4", "E4E5", "G4G4", "G4G5",
        ];
        expected.sort();
        assert_eq!(moves, expected)
    }

    #[test]
    fn test_flip_vertical() {
        assert_eq!(
            format!("{:x}", flip_vertical(0xffffffffffffffffu64)),
            format!("{:x}", 0x7fffffffffffffffu64),
        );
    }
}
