pub trait U64Ext {
    fn into_usize(self) -> usize;
}

impl U64Ext for u64 {
    #[allow(clippy::cast_possible_truncation)]
    fn into_usize(self) -> usize {
        unsafe { usize::try_from(self).unwrap_unchecked() }
    }
}
