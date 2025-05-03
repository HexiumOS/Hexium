use crate::trace;
use alloc::{
    collections::btree_map::BTreeMap,
    string::{String, ToString},
    sync::Arc,
};
use spin::RwLock;

pub fn init() {
    // For testing purposes, we can create a dummy VFS instance
    let _vfs = Vfs::new();

    trace!("VFS initialized");
}

pub enum VfsError {
    NotFound,
}

pub struct Vnode {
    pub name: String,
    pub vtype: VnodeType,
    pub ops: Arc<dyn VnodeOps>,
}

pub trait VnodeOps {
    fn lookup(&self) -> Result<Vnode, VfsError>;
}

pub enum VnodeType {
    Regular,
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

    pub fn mount(&self, mount_point: &str, ops: Arc<dyn VnodeOps>) {
        self.mounts.write().insert(String::from(mount_point), ops);
    }

    pub fn unmount(&self, mount_point: &str) -> Result<(), VfsError> {
        let mut mounts = self.mounts.write();
        if mounts.remove(mount_point).is_some() {
            Ok(())
        } else {
            Err(VfsError::NotFound)
        }
    }

    fn find_fs(&self, path: &str) -> Result<(Arc<dyn VnodeOps>, String), VfsError> {
        let mounts = self.mounts.read();
        let mut best_match: Option<(&str, &Arc<dyn VnodeOps>)> = None;

        for (key, fs) in mounts.iter() {
            if path.starts_with(key) {
                match best_match {
                    Some((best_key, _)) if key.len() <= best_key.len() => {}
                    _ => best_match = Some((key.as_str(), fs)),
                }
            }
        }

        if let Some((key, fs)) = best_match {
            let relative_path = path.strip_prefix(key).unwrap_or("").trim_start_matches('/');
            Ok((fs.clone(), relative_path.to_string()))
        } else {
            Err(VfsError::NotFound)
        }
    }

    pub fn lookuppn(&self, full_path: &str) -> Result<Vnode, VfsError> {
        let (ops, rel_path) = self.find_fs(full_path)?;

        let vtype = if rel_path.is_empty() || full_path.ends_with('/') {
            VnodeType::Directory
        } else {
            VnodeType::Regular
        };

        let display_path = if rel_path.is_empty() {
            full_path.strip_prefix('/').unwrap_or(full_path)
        } else {
            &rel_path
        };

        Ok(Vnode {
            name: display_path.to_string(),
            vtype,
            ops,
        })
    }
}
