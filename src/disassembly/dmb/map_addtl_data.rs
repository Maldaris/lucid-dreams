
use super::Readable;
use super::Context;
use super::ReadContext;
use super::OBJ_NULL;

pub struct MapAddtlDataEntry {
    offset: u16,
    instance: u32
}

impl MapAddtlDataEntry {
    pub fn new() -> MapAddtlDataEntry {
        MapAddtlDataEntry { offset: 0, instance: OBJ_NULL }
    }
}

impl Readable for MapAddtlDataEntry {
    fn read(&mut self, ctx: &mut Context) {
        ctx.u16(&mut self.offset);
        ctx.id(&mut self.instance);
    }
}

pub struct MapAddtlData {
    entries: Vec<MapAddtlDataEntry>
}

impl MapAddtlData {
    pub fn new() -> MapAddtlData {
        MapAddtlData { entries: Vec::new() }
    }
}

impl Readable for MapAddtlData {
    fn read(&mut self, ctx: &mut Context) {
        let mut count : u32 = 0;
        ctx.u32(&mut count);
        for i in 0..(count as usize) {
            self.entries.push(MapAddtlDataEntry::new());
            self.entries[i].read(ctx);
        }
    }
}