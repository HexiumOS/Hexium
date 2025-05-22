/*
 * This file is part of Hexium OS.
 * Copyright (C) 2025 The Hexium OS Authors – see the AUTHORS file.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

#[cfg(not(target_arch = "x86_64"))]
use crate::error;
use crate::{
    hal::vfs::{Vfs, VfsError, Vnode, VnodeOps, VnodeType},
    print, trace, utils,
};
use alloc::sync::Arc;
use alloc::{
    format,
    string::{String, ToString},
};

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
    #[cfg(target_arch = "x86_64")]
    {
        if let Some(module_response) = crate::arch::limine::MODULE_REQUEST.get_response() {
            let modules = module_response.modules();
            if !modules.is_empty() {
                trace!("Ramdisk information:");
                print!("    Ramdisk address:        {:?}\n", modules[0].addr());
                print!("    Ramdisk size (bytes):   {:?}\n", modules[0].size());
                print!("    Ramdisk module path:    {:?}\n", modules[0].path());
            }

            let archive = unsafe {
                core::slice::from_raw_parts(
                    modules[0].addr() as *const u8,
                    modules[0].size() as usize,
                )
            };
            let ramfs = RamFs::new(archive);
            vfs.mount("/ramdisk", Arc::new(ramfs));
        }

        trace!("Ramdisk mounted at /ramdisk");
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        error!("Ramdisk is not supported on this architecture");
    }
}

pub fn tar_lookup<'a>(
    archive: &'a [u8],
    filename: &'a str,
) -> Option<(usize, &'a [u8], VnodeType)> {
    let mut ptr = 0;

    // Ensure filename starts with "./"
    let normalized_filename = if filename.starts_with("./") {
        filename.to_string()
    } else {
        format!("./{}", filename)
    };

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

        if file_name == normalized_filename.as_bytes() {
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
