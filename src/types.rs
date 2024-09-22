/// | key size | val size | offset | type | fence | ref | look ahead |
///      14b       14b       16b       2b     1b     1b       16b
#[repr(transparent)]
pub struct KVMeta(u64);

impl KVMeta {
    #[inline]
    pub fn key_size(&self) -> u64 {
        self.0 >> 50
    }

    #[inline]
    pub fn val_size(&self) -> u64 {
        (self.0 << 14) >> 50
    }

    #[inline]
    pub fn offset(&self) -> u64 {
        (self.0 << 28) >> 48
    }

    #[inline]
    pub fn typ(&self) -> u64 {
        (self.0 << 44) >> 62
    }

    #[inline]
    pub fn fence(&self) -> bool {
        const MASK: u64 = 1 << 17;
        (MASK & self.0) == MASK
    }

    #[inline]
    pub fn ref_bit(&self) -> bool {
        const MASK: u64 = 1 << 16;
        (MASK & self.0) == MASK
    }

    #[inline]
    pub fn look_ahead(&self) -> u64 {
        const MASK: u64 = 0xFFFF;
        MASK & self.0
    }
}

// #[repr()]
// pub struct NodeMeta(u32, u32, u32);
