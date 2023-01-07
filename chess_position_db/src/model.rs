use std::fmt;
use serde::{Serialize, Deserialize};


pub const START_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq";


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MoveStats {
    pub move_san: String,
    pub fen_after: String,
    pub times_played: u16,
    pub times_white_won: u16,
    pub times_black_won: u16,
}

impl MoveStats {
    pub fn new(move_san: String, fen_after: String, result: &str) -> MoveStats {
        MoveStats {
            move_san,
            fen_after,
            times_played: 1,
            times_white_won: if result == "1-0" { 1 } else { 0 },
            times_black_won: if result == "0-1" { 1 } else { 0 },
        }
    }

    pub fn update_times_won(&mut self, result: &str) {
        match result {
            "1-0" => self.times_white_won += 1,
            "0-1" => self.times_black_won += 1,
            _ => (),
        }
    }
}

impl fmt::Display for MoveStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "({}, times played: {}, win rate[white]: {:.3}, win rate[black]: {:.3}))",
            self.move_san,
            self.times_played,
            self.times_white_won as f32 / self.times_played as f32,
            self.times_black_won as f32 / self.times_played as f32
        );
    }
}

impl Clone for MoveStats {
    fn clone(&self) -> MoveStats {
        MoveStats { move_san: self.move_san.to_owned(),
                    fen_after: self.fen_after.to_owned(),
                    times_played: self.times_played,
                    times_white_won: self.times_white_won,
                    times_black_won: self.times_black_won }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PositionStats {
    pub side_to_play: char,
    pub played_moves: Vec<MoveStats>,
}

impl fmt::Display for PositionStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str_ = format!("Plays: {} - ", self.side_to_play);

        for move_ in &self.played_moves {
            str_ = str_ + &format!("{}, ", move_);
        }

        return write!(f, "{}", str_);
    }
}

impl PositionStats {
    pub fn new(side_to_play: char) -> PositionStats {
        PositionStats {
            side_to_play,
            played_moves: Vec::new(),
        }
    }

    pub fn start_position() -> PositionStats {
        PositionStats {
            side_to_play: 'w',
            played_moves: Vec::new(),
        }
    }
}

impl Clone for PositionStats {
    fn clone(&self) -> PositionStats {
        PositionStats { side_to_play: self.side_to_play,
                        played_moves: self.played_moves.clone() }
    }
}

