use crate::{boot, print, trace};

pub fn init() {
    if let Some(module_response) = boot::MODULE_REQUEST.get_response() {
        let modules = module_response.modules();
        if !modules.is_empty() {
            trace!("Ramdisk imformation:\n");
            print!("    Ramdisk address:            {:?}\n", modules[0].addr());
            print!("    Ramdisk size (bytes):       {:?}\n", modules[0].size());
            print!("    Ramdisk module path:        {:?}\n", modules[0].path());
            print!("\n");
        }
    }
}
