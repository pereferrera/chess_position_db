extern crate sled;
extern crate bincode;
extern crate chess_position_db;

use chess_position_db::parse::*;
use chess_position_db::model::*;
use chess_position_db::store::*;


fn parse_db() {
    let file_name = "chess_position_db/test_dataset.pgn";
//    let file_name = "millionbase-2.5.pgn";

    let mut store = MixedStore::new(&"parsed_db".to_string());
    let res = parse_database(file_name, &mut store);

    let position_stats = store.get(&START_POSITION_FEN.to_string());
    println!("from start position -> {}", position_stats);
}

fn main() {
    parse_db()
}
