use crate::{
    boot, fs::vfs::{FileSystem, FileType, VNode, VFS}, info, print, trace, utils
};
use alloc::boxed::Box;
use alloc::string::{String, ToString};

pub struct RamFs {
    //files: Vec<VNode>,
    archive: &'static [u8],
}

impl RamFs {
    pub fn new(archive: &'static [u8]) -> Self {
        RamFs {
            //files: Vec::new(),
            archive,
        }
    }
}

impl FileSystem for RamFs {
    fn mount(&mut self, _path: &str) -> Result<(), ()> {
        info!("RamFs mounted");
        Ok(())
    }

    fn unmount(&mut self) -> Result<(), String> {
        info!("RamFs unmounted");
        Ok(())
    }

    fn open(&self, path: &str) -> Result<VNode, String> {
        if let Some((_size, _content)) = tar_lookup(self.archive, path) {
            Ok(VNode::new(path.to_string(), FileType::File))
        } else {
            Err("File not found".to_string())
        }
    }

    fn read(&self, file: &VNode, buf: &mut [u8], offset: usize) -> Result<usize, String> {
        if let Some((_size, content)) = tar_lookup(self.archive, &file.file_name) {
            let len = buf.len().min(content.len().saturating_sub(offset));
            buf[..len].copy_from_slice(&content[offset..offset + len]);
            Ok(len)
        } else {
            Err("File not found".to_string())
        }
    }

    fn write(&mut self, _file: &VNode, _buf: &[u8], _offset: usize) -> Result<usize, String> {
        Err("Write operation not supported".to_string())
    }

    fn create(&mut self, _path: &str, _file_type: FileType) -> Result<VNode, String> {
        Err("Create operation not supported".to_string())
    }
}

pub fn init(vfs: &mut VFS) {
    if let Some(module_response) = boot::MODULE_REQUEST.get_response() {
        let modules = module_response.modules();
        if !modules.is_empty() {
            trace!("Ramdisk information:");
            print!("    Ramdisk address:        {:?}\n", modules[0].addr());
            print!("    Ramdisk size (bytes):   {:?}\n", modules[0].size());
            print!("    Ramdisk module path:    {:?}\n", modules[0].path());
            print!("\n");
        }

        let archive = unsafe {
            core::slice::from_raw_parts(
                modules[0].addr() as *const u8,
                (modules[0].size() as u64).try_into().unwrap(),
            )
        };
        let ramfs = Box::new(RamFs::new(archive));
        vfs.mount_fs(ramfs);
    }
}

pub fn tar_lookup<'a>(archive: &'a [u8], filename: &str) -> Option<(usize, &'a [u8])> {
    let mut ptr = 0;

    while ptr + 257 < archive.len() {
        if &archive[ptr + 257..ptr + 262] != b"ustar" {
            break;
        }

        let header_filename = &archive[ptr..ptr + 100];
        let name_len = header_filename.iter().position(|&x| x == 0).unwrap_or(100);
        let file_name = &header_filename[..name_len];

        let filesize = utils::octal_to_binrary(&archive[ptr + 124..ptr + 135]);

        if file_name == filename.as_bytes() {
            return Some((
                filesize as usize,
                &archive[ptr + 512..ptr + 512 + filesize as usize],
            ));
        }

        ptr = ptr + ((((filesize as usize) + 511) / 512) + 1) * 512;
    }
    None
}
