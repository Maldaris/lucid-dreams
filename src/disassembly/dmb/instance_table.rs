
use super::Readable;
use super::Context;
use super::ReadContext;
use super::OBJ_NULL;

use crate::dm_types::values::ValueTag;

pub struct InstanceEntry {
    val_type: ValueTag,
    val: u32,
    initializer: u32
}

impl InstanceEntry {
    pub fn new() -> InstanceEntry {
        InstanceEntry { val_type: ValueTag::Null, val: 0, initializer: OBJ_NULL }
    }
}

impl Readable for InstanceEntry {
    fn read(&mut self, ctx: &mut Context) {
        let mut type_container : u8 = 0;
        ctx.u8(&mut type_container);
        self.val_type = ValueTag::from_byte(type_container);
        ctx.u32(&mut self.val);
        ctx.id(&mut self.initializer);
    }
}

pub struct InstanceTable {
    entries: Vec<InstanceEntry>
}

impl InstanceTable {
    pub fn new() -> InstanceTable {
        InstanceTable { entries: Vec::new() }
    }
}

impl Readable for InstanceTable {
    fn read(&mut self, ctx: &mut Context) {
        let mut count: u32 = 0;
        ctx.u32(&mut count);
        for i in 0..(count as usize) {
            self.entries.push(InstanceEntry::new());
            self.entries[i].read(ctx);
        }
    }
}

