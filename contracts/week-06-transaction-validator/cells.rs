use crate::transaction::Hash;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Script {
    pub code_hash: Hash,
    pub hash_type: u8,
    pub args: Vec<u8>,
}

impl Script {
    pub fn new(
        code_hash: Hash,
        hash_type: u8,
        args: Vec<u8>,
    ) -> Self {
        Self {
            code_hash,
            hash_type,
            args,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub capacity: u64,
    pub lock: Script,
    pub type_script: Option<Script>,
    pub data: Vec<u8>,
}

impl Cell {
    pub fn new(
        capacity: u64,
        lock: Script,
        type_script: Option<Script>,
        data: Vec<u8>,
    ) -> Self {
        Self {
            capacity,
            lock,
            type_script,
            data,
        }
    }
}
