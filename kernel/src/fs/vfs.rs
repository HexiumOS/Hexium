use alloc::vec::Vec;

extern crate alloc;

pub trait VfsOps {
    fn create_file(&mut self, name: &str, data: &[u8]);
    fn read_file(&self, name: &str) -> Option<Vec<u8>>;
    fn delete_file(&mut self, name: &str);
}
