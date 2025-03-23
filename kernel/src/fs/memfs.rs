use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::fs::vfs::FileSystem;
use crate::fs::vfs::FileType;
use crate::fs::vfs::VNode;


pub struct MemFS {
    files: BTreeMap<String, Vec<u8>>, // Store file data in memory
    mounted: bool,
}

impl MemFS {
    pub fn new() -> Self {
        MemFS {
            files: BTreeMap::new(),
            mounted: false,
        }
    }
}

impl FileSystem for MemFS {
    fn mount(&mut self, _path: &str) -> Result<(), ()> {
        self.mounted = true;
        Ok(())
    }

    fn unmount(&mut self) -> Result<(), String> {
        self.mounted = false;
        self.files.clear();
        Ok(())
    }

    fn create(&mut self, path: &str, file_type: FileType) -> Result<VNode, String> {
        if self.files.contains_key(path) {
            return Err("File already exists".to_string());
        }
        self.files.insert(path.to_string(), Vec::new());
        Ok(VNode::new(path.to_string(), file_type))
    }

    fn open(&self, path: &str) -> Result<VNode, String> {
        if self.files.contains_key(path) {
            Ok(VNode::new(path.to_string(), FileType::File))
        } else {
            Err("File not found".to_string())
        }
    }

    fn read(&self, file: &VNode, buf: &mut [u8], offset: usize) -> Result<usize, String> {
        if let Some(data) = self.files.get(&file.file_name) {
            let len = buf.len().min(data.len().saturating_sub(offset));
            buf[..len].copy_from_slice(&data[offset..offset + len]);
            Ok(len)
        } else {
            Err("File not found".to_string())
        }
    }

    fn write(&mut self, file: &VNode, buf: &[u8], offset: usize) -> Result<usize, String> {
        if let Some(data) = self.files.get_mut(&file.file_name) {
            if offset + buf.len() > data.len() {
                data.resize(offset + buf.len(), 0);
            }
            data[offset..offset + buf.len()].copy_from_slice(buf);
            Ok(buf.len())
        } else {
            Err("File not found".to_string())
        }
    }
}
