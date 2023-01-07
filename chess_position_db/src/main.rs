extern crate sled;
extern crate bincode;
extern crate chess_position_db;

use chess_position_db::parse::*;


fn parse_db() {
    let file_name = "chess_position_db/test_dataset.pgn";
//    let file_name = "millionbase-2.5.pgn";
    let mut store = BtreeKVStore::new();
    let res = parse_database(file_name, &mut store);

    let position_stats = store.get(&START_POSITION_FEN.to_string());
    println!("from start position -> {}", position_stats);
}

fn test_sled() {
    let db: sled::Db = sled::open("my_db").unwrap();
    
    let position_stats: PositionStats = PositionStats::new('w');
    let encoded: Vec<u8> = bincode::serialize(&position_stats).unwrap();
    
    db.insert(b"yo!", encoded);
    
    let decoded: PositionStats = bincode::deserialize(&db.get(b"yo!").unwrap().unwrap()).unwrap();
    println!("{}", decoded);
}

fn main() {
    parse_db()
}
