extern crate sled;
extern crate bincode;
extern crate chess_position_db;

use chess_position_db::parse::*;


fn parse_db() {
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

fn main() {
    let db: sled::Db = sled::open("my_db").unwrap();
    
    let position_stats: PositionStats = PositionStats::new('w');
    let encoded: Vec<u8> = bincode::serialize(&position_stats).unwrap();
    
    db.insert(b"yo!", encoded);
    
    let decoded: PositionStats = bincode::deserialize(&db.get(b"yo!").unwrap().unwrap()).unwrap();
    println!("{}", decoded);
}
