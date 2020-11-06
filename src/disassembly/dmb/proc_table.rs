
use super::Readable;
use super::Context;
use super::ReadContext;

pub struct ProcTableEntry {
    path: u32,
    name: u32,
    id_baa: u32,
    verb_category: u32,
    unk_a: u8,
    unk_b: u8,
    unk_c: u8,
    unk_c_x: u32,
    unk_c_y: u8,
    code: u32,
    locals: u32,
    args: u32,
}

impl ProcTableEntry {
    pub fn new() -> ProcTableEntry {
        ProcTableEntry {
            path: 0, name: 0, id_baa: 0, verb_category: 0,
            unk_a: 0, unk_b: 0, unk_c: 0, unk_c_x: 0, unk_c_y: 0,
            code: 0, locals: 0, args: 0
        }
    }
}

impl Readable for ProcTableEntry {
    fn read(&mut self, ctx: &mut Context) {
        if ctx.v_gen >= 224 || ctx.large_object_ids {
            ctx.id(&mut self.path);
        }
        ctx.id(&mut self.name);
        ctx.id(&mut self.id_baa);
        ctx.id(&mut self.verb_category);
        ctx.u8(&mut self.unk_a);
        ctx.u8(&mut self.unk_b);
        ctx.u8(&mut self.unk_c);
        if (self.unk_c as i8) < 0 {
            ctx.u32(&mut self.unk_c_x);
            ctx.u8(&mut self.unk_c_y);
        }
        ctx.id(&mut self.code);
        ctx.id(&mut self.locals);
        ctx.id(&mut self.args);
    }
}

pub struct ProcTable {
    entries: Vec<ProcTableEntry>
}

impl ProcTable {
    pub fn new() -> ProcTable {
        ProcTable { entries: Vec::new() }
    }

    pub fn size(&self) -> usize { self.entries.len() }
}

impl Readable for ProcTable {
    fn read(&mut self, ctx: &mut Context) {
        let mut count : u32 = 0;
        ctx.u32(&mut count);
        for i in 0..(count as usize) {
            self.entries.push(ProcTableEntry::new());
            self.entries[i].read(ctx);
        }
    }
}