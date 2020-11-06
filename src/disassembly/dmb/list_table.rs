use super::Readable;
use super::Context;
use super::ReadContext;

pub struct ListTable {
    entries: Vec<u32>
}

impl ListTable {
    pub fn new() -> ListTable {
        ListTable { entries: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }
}

impl Readable for ListTable {
    fn read(&mut self, ctx: &mut Context) {
        let mut entry_count = 0;
        ctx.u16(&mut entry_count);
        entry_count &= 0xFFFF;
        for _i in 0..entry_count {
            let mut tmp : u32 = 0;
            ctx.u32(&mut tmp);
            self.entries.push(tmp);
        }
    }
}
