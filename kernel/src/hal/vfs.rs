use crate::trace;
use alloc::{collections::btree_map::BTreeMap, string::String, sync::Arc};
use spin::RwLock;

pub fn init() {
    // For testing purposes, we can create a dummy VFS instance
    let _vfs = Vfs::new();

    trace!("VFS initialized");
}

pub enum VfsError {}

pub struct Vnode {
    pub name: String,
    pub vtype: VnodeType,
    pub ops: Arc<dyn VnodeOps>,
}

pub trait VnodeOps {
    fn lookup(&self) -> Result<Vnode, VfsError>;
}

pub enum VnodeType {
    File,
    Directory,
}

pub struct Vfs {
    mounts: RwLock<BTreeMap<String, Arc<dyn VnodeOps>>>,
}

impl Vfs {
    pub fn new() -> Self {
        trace!("Creating new VFS instance");
        Vfs {
            mounts: RwLock::new(BTreeMap::new()),
        }
    }
}
