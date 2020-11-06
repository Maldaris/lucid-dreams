
use super::Readable;
use super::Context;
use super::ReadContext;

pub struct MobTypeEntry {
    class: u32,
    key: u32,
    sight_flags: u8,
    sight_flags_ex: u32,
    see_in_dark: u8,
    see_invisible: u8
}

impl MobTypeEntry {
   pub fn new() -> MobTypeEntry {
        MobTypeEntry {
            class: 0, key: 0,
            sight_flags: 0, sight_flags_ex: 0,
            see_in_dark: 0, see_invisible: 0
        }
   }
}

pub struct MobTypeTable {
    entries: Vec<MobTypeEntry>
}

impl MobTypeTable {
    pub fn new() -> MobTypeTable {
        MobTypeTable { entries: Vec::new() }
    }

    pub fn size(&self) -> usize {
        return self.entries.len();
    }
}

impl Readable for MobTypeTable {
    fn read(&mut self, ctx: &mut Context) {
        let count : usize = 0;
        ctx.u32(&mut (count as u32));
        for i in 0..count {
            self.entries.push(MobTypeEntry::new());
            self.entries[i].read(ctx);
        }
    }
}

impl Readable for MobTypeEntry {
    fn read(&mut self, ctx: &mut Context) {
        ctx.u32(&mut self.class);
        ctx.u32(&mut self.key);
        ctx.u8(&mut self.sight_flags);
        if (self.sight_flags as i8) < 0 {
            ctx.u32(&mut self.sight_flags_ex);
            ctx.u8(&mut self.see_in_dark);
            ctx.u8(&mut self.see_invisible);
        }
    }
}