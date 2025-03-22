use alloc::boxed::Box;
use alloc::string::{String, ToString};

pub trait FileSystem {
    fn mount(&mut self, path: &str) -> Result<(), ()>;
    fn unmount(&mut self) -> Result<(), String>;
    fn create(&mut self, path: &str, file_type: FileType) -> Result<VNode, String>;
    fn open(&self, path: &str) -> Result<VNode, String>;
    fn read(&self, file: &VNode, buf: &mut [u8], offset: usize) -> Result<usize, String>;
    fn write(&mut self, file: &VNode, buf: &[u8], offset: usize) -> Result<usize, String>;
}

pub struct VNode {
    pub file_name: String,
    pub file_type: FileType,
}

impl VNode {
    pub fn new(file_name: String, file_type: FileType) -> Self {
        VNode {  file_name, file_type }
    }

    pub fn default() -> Self {
        VNode::new("unnamed_vnode".to_string(),FileType::File)
    }
}

pub enum FileType {
    File = 0,
    Directory = 1,
}

pub struct VFS {
    pub fs: Option<Box<dyn FileSystem>>,
}

impl VFS {
    pub fn new(fs: Option<Box<dyn FileSystem>>) -> Self {
        VFS { fs }
    }

    pub fn mount_fs(&mut self, fs: Box<dyn FileSystem>) {
        self.fs = Some(fs);
    }

    pub fn unmount_fs(&mut self) -> Result<(), String> {
        if let Some(fs) = &mut self.fs {
            fs.unmount()
        } else {
            Err("No file system mounted".to_string())
        }
    }

    pub fn create_file(&mut self, path: &str, file_type: FileType) -> Result<VNode, String> {
        if let Some(fs) = &mut self.fs {
            fs.create(path, file_type)
        } else {
            Err("No file system mounted".to_string())
        }
    }

    pub fn open_file(&mut self, path: &str) -> Result<VNode, String> {
        if let Some(fs) = &self.fs {
            fs.open(path)
        } else {
            Err("No file system mounted".to_string())
        }
    }

    pub fn read_file(&mut self, file: &VNode, buf: &mut [u8], offset: usize) -> Result<usize, String> {
        if let Some(fs) = &self.fs {
            fs.read(file, buf, offset)
        } else {
            Err("No file system mounted".to_string())
        }
    }
}
