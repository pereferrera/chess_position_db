use std::fmt;

pub struct MoveStatsDAG {
    pub move_san: String,
    pub times_played: u16,
    pub times_white_won: u16,
    pub times_black_won: u16,
    pub position_after: PositionStatsDAG,
}

impl MoveStatsDAG {
    pub fn new(move_san: String, result: &str, new_side_to_play: char) -> MoveStatsDAG {
        MoveStatsDAG {
            move_san,
            times_played: 1,
            times_white_won: if result == "1-0" { 1 } else { 0 },
            times_black_won: if result == "0-1" { 1 } else { 0 },
            position_after: PositionStatsDAG::new(new_side_to_play),
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

impl fmt::Display for MoveStatsDAG {
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

pub struct PositionStatsDAG {
    pub side_to_play: char,
    pub played_moves: Vec<MoveStatsDAG>,
}

impl PositionStatsDAG {
    pub fn new(side_to_play: char) -> PositionStatsDAG {
        PositionStatsDAG {
            side_to_play,
            played_moves: Vec::new(),
        }
    }

    pub fn start_position() -> PositionStatsDAG {
        PositionStatsDAG {
            side_to_play: 'w',
            played_moves: Vec::new(),
        }
    }

    pub fn sort_moves(&mut self) {
        self.played_moves.sort_unstable_by_key(|p| p.times_played);
    }
}

impl fmt::Display for PositionStatsDAG {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str_ = format!("Plays: {} - ", self.side_to_play);

        for move_ in &self.played_moves {
            str_ = str_ + &format!("{}, ", move_);
        }

        return write!(f, "{}", str_);
    }
}
