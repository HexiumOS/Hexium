use crate::fs::vfs::VfsOps;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct MemFs {
    files: BTreeMap<String, Vec<u8>>,
}

impl MemFs {
    pub fn new() -> Self {
        MemFs {
            files: BTreeMap::new(),
        }
    }
}

impl VfsOps for MemFs {
    fn create_file(&mut self, name: &str, data: &[u8]) {
        self.files.insert(String::from(name), data.to_vec());
    }

    fn read_file(&self, name: &str) -> Option<Vec<u8>> {
        self.files.get(name).map(|data| data.clone())
    }

    fn delete_file(&mut self, name: &str) {
        self.files.remove(name);
    }
}
