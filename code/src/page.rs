pub const PAGE_SIZE: usize = 8192;  // Individual page size

pub struct Page{
    pub id: u64,                // 4 bytes integer - 4 billion page id's are possible.
    pub data: [u8; PAGE_SIZE]   // Array of page size elements
}

impl Page{
    pub fn new(id: u64) -> Self {
        Self { id, data: [0; PAGE_SIZE]
        }
    }
}