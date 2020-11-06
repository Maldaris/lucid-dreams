use super::Readable;
use super::Context;
use super::ReadContext;

pub struct SubBlock7 {
    entries: Vec<u32>
}

impl SubBlock7 {
    pub fn new() -> SubBlock7 {
        SubBlock7 { entries: Vec::new() }
    }
}

impl Readable for SubBlock7 {
    fn read(&mut self, ctx: &mut Context) {
        let mut count : u32 = 0;
        ctx.u32(&mut count);
        for _i in 0..(count as usize) {
            let mut entry : u32 = 0;
            ctx.u32(&mut entry);
            self.entries.push(entry);
        }
    }
}
