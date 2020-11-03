use super::Readable;
use super::ReadContext;
use super::Context;

pub struct Grid {
    x: u16, y: u16, z: u16,
    turf: Vec<u32>,
    area: Vec<u32>,
    addtl_turfs: Vec<u32>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            x: 0, y: 0, z: 0,
            turf: Vec::new(),
            area: Vec::new(),
            addtl_turfs: Vec::new(),
        }
    }
}

impl<'a> Readable for Grid {

    fn read(&mut self, ctx: &mut Context) {
        ctx.u16(&mut self.x);
        ctx.u16(&mut self.y);
        ctx.u16(&mut self.z);

        let total = self.x * self.y * self.z;
        let mut i : usize = 0;
        while i < total.into() {
            ctx.id(&mut self.turf[i]);
            ctx.id(&mut self.area[i]);
            ctx.id(&mut self.addtl_turfs[i]);
            let mut dup : u8 = 0;
            ctx.u8(&mut dup);
            for j in 0..(dup as usize) {
              self.turf[i + j] = self.turf[i].clone();
              self.area[i + j] = self.area[i].clone();
              self.addtl_turfs[i + j] = self.addtl_turfs[i].clone();
            }
            i += dup as usize;
        }

    }
}