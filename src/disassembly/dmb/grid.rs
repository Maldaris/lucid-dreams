use super::Readable;
use super::ReadContext;
use super::Context;

pub struct Grid<'a> {
    x: u16, y: u16, z: u16,
    turf: Option<&'a[u32]>,
    area: Option<&'a[u32]>,
    addtl_turfs: Option<&'a[u32]>,
}

impl<'a> Grid<'a> {
    pub fn new() -> Grid<'a> {
        Grid {
            x: 0, y: 0, z: 0,
            turf: None, area: None, addtl_turfs: None
        }
    }
}

impl<'a> Readable for Grid<'a> {

    fn read(&mut self, ctx: &mut Context) {
        ctx.u16(&mut self.x);
        self.x &= 0xFFFF;
        ctx.u16(&mut self.y);
        self.y &= 0xFFFF;
        ctx.u16(&mut self.z);
        self.z &= 0xFFFF;

        let total = self.x * self.y * self.z;
        self.turf = Some(vec![0; total.into()]);
        self.area = Some(vec![0; total.into()]);
        self.addtl_turfs = Some(vec![0; total.into()]);

        let l_turf : &[u32] = self.turf.unwrap();
        let l_area : &[u32] = self.area.unwrap();
        let l_addtl_turfs : &[u32] = self.addtl_turfs.unwrap();

        let mut i : usize = 0;
        while i < total.into() {
            ctx.id(&mut l_turf[i]);
            ctx.id(&mut l_area[i]);
            ctx.id(&mut l_addtl_turfs[i]);
            let mut dup : u8 = 0;
            ctx.u8(&mut dup);
            dup &= 0xFF;
            for j in 0..(dup as usize) {
                l_turf[i + j] = l_turf[i];
                l_area[i + j] = l_area[i];
                l_addtl_turfs[i + j] = l_addtl_turfs[i];
            }
            i += dup.into();
        }

    }
}