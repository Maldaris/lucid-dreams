
use super::Readable;
use super::Context;
use super::ReadContext;

pub struct CacheId {
    unique_id : u32,
    filetype: u8
}

impl CacheId {
    pub fn new() -> CacheId {
        CacheId { unique_id: 0, filetype: 0 }
    }
    pub fn equal(&self, other: &CacheId) -> bool {
        other.unique_id == self.unique_id && other.filetype == self.filetype
    }
}

impl Readable for CacheId {
    fn read(&mut self, ctx: &mut Context) {
        ctx.u32(&mut self.unique_id);
        ctx.u8(&mut self.filetype);
    }
}

pub struct CacheFileTable {
    entries: Vec<CacheId>
}

impl CacheFileTable {
    pub fn new() -> CacheFileTable {
        CacheFileTable { entries: Vec::new() }
    }
}

impl Readable for CacheFileTable {
    fn read(&mut self, ctx: &mut Context) {
        let mut count : u32 = 0;
        ctx.u32(&mut count);
        for i in 0..(count as usize) {
            self.entries.push(CacheId::new());
            self.entries[i].read(ctx);
        }
    }
}