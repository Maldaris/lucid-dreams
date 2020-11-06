
use super::Readable;
use super::Context;
use super::ReadContext;
use crate::dm_types::values::ValueTag;

pub struct VarTableEntry {
    val_type: ValueTag,
    value: u32,
    name: u32
}

impl VarTableEntry {
    pub fn new() -> VarTableEntry {
        VarTableEntry { val_type: ValueTag::Null, value: 0, name: 0 }
    }
}

impl Readable for VarTableEntry {
    fn read(&mut self, ctx: &mut Context) {
        let mut type_container : u8 = 0;
        ctx.u8(&mut type_container);

        self.val_type = ValueTag::from_byte(type_container);
        ctx.u32(&mut self.value);
        ctx.id(&mut self.name);
    }
}

pub struct VarTable {
    entries: Vec<VarTableEntry>,
    unk: u32
}

impl VarTable {
    pub fn new() -> VarTable {
        VarTable { entries: Vec::new(), unk: 0 }
    }
}

impl Readable for VarTable {
    fn read(&mut self, ctx: &mut Context) {
        let mut count : u32 = 0;
        ctx.u32(&mut count);
        for i in 0..(count as usize) {
            self.entries.push(VarTableEntry::new());
            self.entries[i].read(ctx);
        }
        if ctx.v_gen >= 512 && ctx.v_lhs >= 512 {
            ctx.u32(&mut self.unk);
        }
    }
}
