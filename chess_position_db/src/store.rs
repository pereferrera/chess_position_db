extern crate sled;

use model::*;

use std::collections::BTreeMap;


pub trait ChessPositionsStore {
    fn insert(&mut self, key: &String, value: PositionStats);
    fn contains_key(&self, key: &String) -> bool;
    fn get(&mut self, key: &String) -> PositionStats;
}

pub struct BtreeKVStore {
    data: BTreeMap<String, PositionStats>
}

impl BtreeKVStore {
    pub fn new() -> BtreeKVStore {
        BtreeKVStore { data: BTreeMap::new() }
    }
}

impl ChessPositionsStore for BtreeKVStore {
    fn insert(&mut self, key: &String, value: PositionStats) {
        self.data.insert(key.to_string(), value);
    }

    fn contains_key(&self, key: &String) -> bool {
        self.data.contains_key(key)
    }

    fn get(&mut self, key: &String) -> PositionStats {
        self.data.get_mut(key).unwrap().to_owned()
    }
}

pub struct SledStore {
    db_file: String,
    db: sled::Db
}

impl SledStore {
    pub fn new(db_file: &String) -> SledStore {
        SledStore { db_file: db_file.to_string(),
                    db: sled::open(db_file).unwrap() }
    }
}

impl ChessPositionsStore for SledStore {
    fn insert(&mut self, key: &String, value: PositionStats) {
        let encoded: Vec<u8> = bincode::serialize(&value).unwrap();
        self.db.insert(key, encoded);
    }

    fn contains_key(&self, key: &String) -> bool {
        self.db.contains_key(key).unwrap()
    }

    fn get(&mut self, key: &String) -> PositionStats {
        let position: PositionStats = bincode::deserialize(&self.db.get(key).unwrap().unwrap()).unwrap();
        return position
    }
}

pub struct MixedStore {
    memory_store: BtreeKVStore,
    disk_store: SledStore
}

impl MixedStore {
    pub fn new(db_file: &String) -> MixedStore {
        MixedStore { memory_store: BtreeKVStore::new(),
                     disk_store: SledStore::new(db_file) }
    }
}


const MAX_DATA_IN_MEMORY: u32 = 1024*1024;


impl ChessPositionsStore for MixedStore {

    fn insert(&mut self, key: &String, value: PositionStats) {
        self.memory_store.insert(key, value);
        if self.memory_store.data.len() > MAX_DATA_IN_MEMORY as usize {
            println!("Max data in memory reached, flushing all to disk...");
            for (key, value) in self.memory_store.data.iter() {
                self.disk_store.insert(key, value.to_owned());
            }
            println!("... Flush finished");
            self.memory_store.data = BTreeMap::new()
        }
    }

    fn contains_key(&self, key: &String) -> bool {
        let contained = self.memory_store.contains_key(key);
        if !contained {
            return self.disk_store.contains_key(key);
        }
        return true
    }

    fn get(&mut self, key: &String) -> PositionStats {
        if self.memory_store.contains_key(key) {
            return self.memory_store.get(key);
        }
        return self.disk_store.get(key);
    }
}
