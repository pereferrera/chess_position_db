extern crate chess_position_db;

use chess_position_db::parse::*;

fn main() {
//    let file_name = "chess_position_db/test_dataset.pgn";
    let file_name = "millionbase-2.5.pgn";
    let res = parse_database(file_name);

    match res {
        Ok(position_db) => {
            println!("Created a b-tree of size {}", position_db.len());
            let position_stats = position_db.get(START_POSITION_FEN);
            // Option<&A> -> A (unwrap)
            // here we are confident the result will exist
            println!("from start position -> {}", position_stats.unwrap());
        },
        Err(e) => {        
            println!("ERROR {:?}", e);
        }
    }
}
