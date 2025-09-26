use std::fs::{OpenOptions, File};
use std::io::{Write, Read};
use std::collections::HashMap;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
enum WalEntry {
    Set { key: String, value: String, expiry: Option<std::time::SystemTime> }
 }

pub struct Redis {
  pub  map: HashMap<String, String>,
  pub   wal_file: File,
}

impl Redis {
    pub fn open(wal_path: &str) -> Self {
       //Wal file is a file with written ahead log
        let wal_file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(wal_path)
            .unwrap();

        let mut store = Redis { map: HashMap::new(), wal_file };
        store.replay_wal(wal_path);
        store
    }

    fn replay_wal(&mut self, wal_path: &str) {
        let mut file = File::open(wal_path).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        let mut slice: &[u8] = &buf;

        while !slice.is_empty() {
            if slice.len() < 4 { break; }
            let len = u32::from_le_bytes(slice[0..4].try_into().unwrap()) as usize;
            if slice.len() < 4 + len { break; }
            let entry_bytes = &slice[4..4+len];
            let entry: WalEntry = bincode::deserialize(entry_bytes).unwrap();

            match entry {
                WalEntry::Set { key, value, .. } => { self.map.insert(key, value); }
            }

            slice = &slice[4+len..];
        }
    }

    fn append_wal(&mut self, entry: &WalEntry) {
        let bytes = bincode::serialize(entry).unwrap();
        let len = (bytes.len() as u32).to_le_bytes();

        self.wal_file.write_all(&len).unwrap();
        self.wal_file.write_all(&bytes).unwrap();
        self.wal_file.flush().unwrap();
        self.wal_file.sync_all().unwrap();
    }

    pub fn set(&mut self, key: String, value: String) {
        let entry = WalEntry::Set { key: key.clone(), value: value.clone(), expiry: None };
        self.append_wal(&entry);
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.map.get(key).cloned()
    }
    pub fn delete(&mut self, key: &str) -> bool {
        self.map.remove(key).is_some()
    }

}
