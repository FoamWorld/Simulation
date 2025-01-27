use std::collections::BTreeMap;

pub struct Memory {
    pool: BTreeMap<String, i32>,
}
impl Memory {
    pub fn new() -> Self {
        Memory {
            pool: BTreeMap::<String, i32>::new(),
        }
    }
    pub fn remember(&self, path: String) -> i32 {
        return self.pool[&path];
    }
    pub fn learn(&mut self, path: String, x: i32) {
        self.pool.insert(path, x);
    }
    pub fn forget(&mut self, path: String) {
        self.pool.remove(&path);
    }
}
