use super::Readable;
use super::Context;
use super::ReadContext;
use super::OBJ_NULL;

pub struct StandardObjectIds {
    mob: u32,
    turf: u32,
    area: u32,
    procs: u32,
    global_variable_initializer: u32,
    id_bac: u32,
    name: u32,
    id_old_1: u32,
    tick_time_millis: u32,
    client: u32,
    image: u32,
    unk_b: u8,
    unk_c: u8,
    unk_d: u16,
    unk_e: u8,
    id_c: u32,
    id_even_more: Vec<u32>,
    id_x: u32,
    unk_f: u16,
    unk_px: u16,
    unk_py: u16,
    hub_password_hashed: u32,
    server_name: u32,
    unk_bg: [u32; 2],
    unk_g: u16,
    id_ay: [u32; 2],
    hub: u32,
    channel: u32,
    id_azb: u32,

    icon_size_x: u16,
    icon_size_y: u16,
    unk_z: u16,
}

impl StandardObjectIds {
    pub fn new() -> StandardObjectIds {
        StandardObjectIds {
            mob: OBJ_NULL, turf: OBJ_NULL, area: OBJ_NULL, procs: OBJ_NULL, global_variable_initializer: OBJ_NULL,
            id_bac: OBJ_NULL, name: OBJ_NULL, id_old_1: OBJ_NULL, tick_time_millis: 100, client: OBJ_NULL,
            image: OBJ_NULL, unk_b: 0, unk_c: 1, unk_d: 0, unk_e: 0, id_c: OBJ_NULL, id_even_more: Vec::new(), id_x: OBJ_NULL,
            unk_f: 2827, unk_px: 0, unk_py: 0, hub_password_hashed: OBJ_NULL, server_name: OBJ_NULL,
            unk_bg: [0; 2], unk_g: 30, id_ay: [OBJ_NULL; 2], hub: OBJ_NULL, channel: OBJ_NULL,
            id_azb: OBJ_NULL, icon_size_x: 32, icon_size_y: 32, unk_z: 32768
        }
    }
}

impl Readable for StandardObjectIds {
    fn read(&mut self, ctx: &mut Context) {
        ctx.id(&mut self.mob);
        ctx.id(&mut self.turf);
        ctx.id(&mut self.area);
        ctx.id(&mut self.procs);
        ctx.id(&mut self.global_variable_initializer);
        ctx.id(&mut self.id_bac);
        ctx.id(&mut self.name);
        if ctx.v_gen < 368 {
            ctx.id(&mut self.id_old_1);
        }
        ctx.id(&mut self.tick_time_millis);
        ctx.id(&mut self.client);
        if ctx.v_gen >= 308 {
            ctx.id(&mut self.image);
        }
        ctx.u8(&mut self.unk_b);
        ctx.u8(&mut self.unk_c);
        if ctx.v_gen >= 415 {
            ctx.u16(&mut self.unk_d);
        }
        ctx.u8(&mut self.unk_e);
        if ctx.v_gen >= 230 {
            ctx.id(&mut self.id_c);
        }
        if ctx.v_gen >= 507 {
            let mut count : u16 = 0;
            ctx.u16(&mut count);
            for _i in 0..count {
                let mut id : u32 = 0;
                ctx.u32(&mut id);
                self.id_even_more.push(id);
            }
        }
        if ctx.v_gen < 507 {
            ctx.id(&mut self.id_x);
        }
        if ctx.v_gen >= 232 {
            ctx.u16(&mut self.unk_f);
        }
        if ctx.v_gen >= 235 && ctx.v_gen < 368 {
            ctx.u16(&mut self.unk_px);
        }
        if ctx.v_gen >= 236 && ctx.v_gen < 368 {
            ctx.u16(&mut self.unk_py);
        }
        if ctx.v_gen >= 241 {
            ctx.id(&mut self.hub_password_hashed);
        }
        if ctx.v_gen >= 266 {
            ctx.id(&mut self.server_name);
            ctx.u32(&mut self.unk_bg[0]);
            ctx.u32(&mut self.unk_bg[1]);
        }
        if ctx.v_gen >= 272 {
            ctx.u16(&mut self.unk_g);
            ctx.u32(&mut self.id_ay[0]);
            ctx.u32(&mut self.id_ay[1]);
        }
        if ctx.v_gen >= 276 {
            ctx.u32(&mut self.hub);
        }
        if ctx.v_gen >= 305 {
            ctx.id(&mut self.channel);
        }
        if ctx.v_gen >= 360 {
            ctx.u32(&mut self.id_azb);
        }
        if ctx.v_lhs >= 455 {
            ctx.u16(&mut self.icon_size_x);
            ctx.u16(&mut self.icon_size_y);
            ctx.u16(&mut self.unk_z);
        } else {
            self.icon_size_x = 32;
            self.icon_size_y = 32;
            self.unk_z = 32768;
        }
    }
}