use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;

use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

use pgnparse::parser::*;

pub struct MoveStats {
    pub move_san: String,
    pub fen_after: String,
    pub times_played: u16,
    pub times_white_won: u16,
    pub times_black_won: u16,
}

impl MoveStats {
    fn new(move_san: String, fen_after: String, result: &str) -> MoveStats {
        MoveStats {
            move_san,
            fen_after,
            times_played: 1,
            times_white_won: if result == "1-0" { 1 } else { 0 },
            times_black_won: if result == "0-1" { 1 } else { 0 }
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
        return write!(f, "({}, times played: {}, win rate[white]: {:.3}, win rate[black]: {:.3}))",
                      self.move_san,
                      self.times_played,
                      self.times_white_won as f32 / self.times_played as f32,
                      self.times_black_won as f32 / self.times_played as f32);
    }
}

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
    fn new(side_to_play: char) -> PositionStats {
        PositionStats {
            side_to_play,
            played_moves: Vec::new(),
        }
    }

    fn start_position() -> PositionStats {
        PositionStats {
            side_to_play: 'w',
            played_moves: Vec::new(),
        }
    }
}

pub const START_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq";

pub fn parse_database(pgnfile_name: &str) -> std::io::Result<BTreeMap<String, PositionStats>> {
    let file = File::open(pgnfile_name)?;
    let reader = 
        BufReader::new(DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(file));

    let mut pgn_game = Vec::new();
    let mut n_games = 0;
    let mut parsing_game = false;

    // start position of a board
    let start_position_stats = PositionStats::start_position();

    let mut position_db: BTreeMap<String, PositionStats> = BTreeMap::new();
    position_db.insert(START_POSITION_FEN.to_string(), start_position_stats);

    let mut fen_before: String = START_POSITION_FEN.to_string();

    for line in reader.lines() {
        let read_line = line?;

        if !parsing_game && read_line.starts_with("[") {
            // new game
            pgn_game = Vec::new();
            fen_before = START_POSITION_FEN.to_string();
            parsing_game = true;
        }

        let line = read_line.trim_end().to_string();
        pgn_game.push(line.to_string());
        
        if parsing_game
            && (line.ends_with("0-1")
                || line.ends_with("1-0")
                || line.ends_with("1/2-1/2"))
        {
            // game finished
            if !pgn_game.is_empty() {
                // parse game
                println!("Parsing game number #{}", n_games);
                let result = parse_pgn_to_rust_struct(
                    pgn_game.join("\n"));
                let game_result = &result.headers["Result"].trim().to_string();

                for move_ in result.moves {
                    let mut side_to_play = 'w';
                    if fen_before.contains(" b ") {
                        side_to_play = 'b';
                    }

                    // get rid of unnecessary info like move number
                    // otherwise we cannot find transpositions
                    let fen_after_copy = move_.fen_after
                        .to_string().split(" - ").next().unwrap().to_string();

                    if !position_db.contains_key(&fen_before) {
                        position_db.insert(fen_before.clone(),
                                           PositionStats::new(
                                                side_to_play));
                    }

                    let position =
                        position_db.get_mut(&fen_before).unwrap();
                    let mut seen = false;

                    for played_move_ in
                            &mut position.played_moves {
                        if played_move_.move_san == move_.san {
                            played_move_.times_played += 1;
                            played_move_.update_times_won(game_result);
                            seen = true;
                            position.played_moves.sort_by_key(
                                |p| p.times_played);
                            break;
                        }
                    }

                    if !seen {
                        let move_stats = MoveStats::new(move_.san.to_string(),
                                                                   fen_after_copy.clone(),
                                                                   game_result);
                        position
                            .played_moves
                            .push(move_stats);
                    }

                    fen_before = fen_after_copy;
                }
                n_games += 1;
                parsing_game = false;
            }
        }
    }

    Ok(position_db)
}