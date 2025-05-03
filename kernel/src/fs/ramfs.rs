use crate::{
    boot,
    hal::vfs::{Vfs, VfsError, Vnode, VnodeOps, VnodeType},
    print, trace, utils,
};
use alloc::string::{String, ToString};
use alloc::sync::Arc;

pub struct RamFs {
    archive: &'static [u8],
}

impl RamFs {
    pub fn new(archive: &'static [u8]) -> Self {
        RamFs { archive }
    }
}

impl VnodeOps for RamFs {
    fn lookup(&self, path: &str) -> Result<Vnode, VfsError> {
        match tar_lookup(self.archive, path) {
            Some((_size, _content, vtype)) => Ok(Vnode {
                name: path.to_string(),
                vtype,
                ops: Arc::new(RamFs::new(self.archive)),
            }),
            None => Err(VfsError::NotFound),
        }
    }

    fn read(
        &self,
        file: &Vnode,
        buf: &mut [u8],
        offset: usize,
        _length: usize,
    ) -> Result<usize, String> {
        if let Some((size, content, _)) = tar_lookup(self.archive, &file.name) {
            let start = offset.min(size);
            let end = (offset + buf.len()).min(size);
            let len = end - start;
            if len > 0 {
                buf[..len].copy_from_slice(&content[start..end]);
            }
            Ok(len)
        } else {
            Err("File not found".to_string())
        }
    }
}

pub fn init(vfs: &Vfs) {
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
            core::slice::from_raw_parts(modules[0].addr() as *const u8, modules[0].size() as usize)
        };
        let ramfs = RamFs::new(archive);
        vfs.mount("/", Arc::new(ramfs));
    }
}

pub fn tar_lookup<'a>(
    archive: &'a [u8],
    filename: &'a str,
) -> Option<(usize, &'a [u8], VnodeType)> {
    let mut ptr = 0;

    while ptr + 257 < archive.len() {
        if &archive[ptr + 257..ptr + 262] != b"ustar" {
            break;
        }

        let header_filename = &archive[ptr..ptr + 100];
        let name_len = header_filename.iter().position(|&x| x == 0).unwrap_or(100);
        let file_name = &header_filename[..name_len];

        let typeflag = archive[ptr + 156];
        let vtype = if typeflag == b'5' {
            VnodeType::Directory
        } else {
            VnodeType::Regular
        };

        let filesize = utils::octal_to_binrary(&archive[ptr + 124..ptr + 135]);

        if file_name == filename.as_bytes() {
            return Some((
                filesize as usize,
                &archive[ptr + 512..ptr + 512 + filesize as usize],
                vtype,
            ));
        }

        ptr += (((filesize as usize + 511) / 512) + 1) * 512;
    }
    None
}
