const MIB: usize = 1 << 20;
const GIB: usize = 1 << 30;
const BUFFER_SIZE: usize = MIB;
const BUFFER_CAP: usize = BUFFER_SIZE >> 3;

/// all memory
pub struct Buffer {
    free_list: Vec<u32>,
    // always points to the beggining of a Node
    head: u32,
    // head + len points to next free space, or head if none left
    len: u32,
    inner: [u64; BUFFER_CAP],
}

#[repr(transparent)]
pub struct Index(u32);

// Idea: keep track of the last used item, but also ones that are being evicted
// this means that eviction can happen asyncronously and without locking, just advance the valid pointer and start the eviction process,
// the memory is still claimed

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            free_list: Vec::new(),
            head: 0,
            len: 0,
            inner: [0; BUFFER_CAP],
        }
    }

    pub fn alloc(&mut self, size: Bucket) -> Option<Index> {
        if BUFFER_CAP as u32 - self.len > size.size() {
            let out = self.tail();

            self.len += size.size();
            Some(Index(out))
        } else {
            None
        }
    }
}

impl Buffer {
    #[inline]
    fn wrap(num: u32) -> u32 {
        num % BUFFER_CAP as u32
    }

    #[inline]
    fn tail(&self) -> u32 {
        Self::wrap(self.head + self.len)
    }

    #[inline]
    fn second_chance(&self) -> u32 {
        Self::wrap(self.head + (self.len >> 3))
    }
}

pub enum Bucket {
    B64 = 1,
    B512 = 5,
    B4096 = 7,
}

impl Bucket {
    fn size(&self) -> u32 {
        match self {
            Bucket::B64 => 64 >> 3,
            Bucket::B512 => 512 >> 3,
            Bucket::B4096 => 4096 >> 3,
        }
    }
}
