use crate::{boot, print, trace, utils};

pub fn init() {
    if let Some(module_response) = boot::MODULE_REQUEST.get_response() {
        let modules = module_response.modules();
        if !modules.is_empty() {
            trace!("Ramdisk information:\n");
            print!("    Ramdisk address:            {:?}\n", modules[0].addr());
            print!("    Ramdisk size (bytes):       {:?}\n", modules[0].size());
            print!("    Ramdisk module path:        {:?}\n", modules[0].path());
            print!("\n");
        }

        let archive = unsafe {
            core::slice::from_raw_parts(
                modules[0].addr() as *const u8,
                (modules[0].size() as u64).try_into().unwrap(),
            )
        };

        // Try without leading slash
        if let Some((size, content)) = tar_lookup(archive, "./welcome.txt") {
            print!("    Found welcome.txt, size: {}\n", size);
            if let Ok(message) = core::str::from_utf8(content) {
                print!("    Content:                     {}", message);
            } else {
                print!("    Failed to convert content to UTF-8\n");
            }
        } else {
            print!("    welcome.txt not found in archive\n");
        }

        print!("\n");
    }
}

pub fn tar_lookup<'a>(archive: &'a [u8], filename: &str) -> Option<(usize, &'a [u8])> {
    let mut ptr = 0;

    while ptr + 257 < archive.len() {
        // Check for ustar magic
        if &archive[ptr + 257..ptr + 262] != b"ustar" {
            print!("    No ustar magic at offset {}\n", ptr);
            break;
        }

        // Read the full filename from TAR header (100 bytes)
        let header_filename = &archive[ptr..ptr + 100];
        // Find the actual filename length (until first null byte)
        let name_len = header_filename.iter().position(|&x| x == 0).unwrap_or(100);
        let file_name = &header_filename[..name_len];

        let filesize = utils::octal_to_binrary(&archive[ptr + 124..ptr + 135]);

        // Debug print
        if let Ok(name) = core::str::from_utf8(file_name) {
            print!("    Found file in archive:      {}\n", name);
        }

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
