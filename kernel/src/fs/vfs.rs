use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::str;

extern crate alloc;

#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    pub fn new(name: &str, data: &[u8]) -> Self {
        File {
            name: String::from(name),
            data: data.to_vec(),
        }
    }
}

#[derive(Debug)]
pub struct VFS {
    files: BTreeMap<String, File>,
}

impl VFS {
    pub fn new() -> Self {
        VFS {
            files: BTreeMap::new(),
        }
    }

    pub fn create_file(&mut self, name: &str, data: &[u8]) {
        let file = File::new(name, data);
        self.files.insert(String::from(name), file);
    }

    pub fn read_file(&self, name: &str) -> Option<&[u8]> {
        self.files.get(name).map(|file| file.data.as_slice())
    }

    pub fn delete_file(&mut self, name: &str) {
        self.files.remove(name);
    }
}
