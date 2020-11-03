use super::Readable;
use super::Context;
use super::ReadContext;

const CF_MOB : u8 = 2;
const CF_ATOM : u8 = 4;
const CF_AREA : u8 = 8;

const OBJ_NULL : u32 = 0xFFFF;

struct ClassEntry {
    name: u32,
    parent : u32,
    obj_name : u32,
    description : u32,
    icon : u32,
    icon_state : u32,

    direction : u8,

    dm_special_type_long : bool,
    dm_special_type : u32,

    text : u32,

    id_h: u32,
    unk_b: u16,
    unk_c: u16,

    unk_d: u16,
    unk_e: u16,
    
    suffix: u32,
    flags: u32,
    verb_table: u32,
    proc_table: u32,
    initializer: u32,
    initalized_vars_table: u32,
    var_table: u32,

    layer: f32,

    has_floats: u8,
    floats: [f32; 8],

    has_even_more_floats: u8,
    even_more_floats: [f32; 20],

    overriding_var_list: u32
}

impl ClassEntry {
    pub fn new() -> ClassEntry {
        ClassEntry {
            name: 0, parent: 0, obj_name: 0,
            description: 0, icon: 0, icon_state: 0,
            direction: 0,
            dm_special_type_long: false, dm_special_type: 0,
            text: 0, id_h: 0, unk_b: 0, unk_c: 0, unk_d: 0, unk_e: 0,
            suffix: 0, flags: 0, verb_table: 0, proc_table: 0, initializer: 0,
            initalized_vars_table: 0, var_table: 0, layer: 0.0,
            has_floats: 0,
            floats: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            has_even_more_floats: 0,
            even_more_floats: [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
            ],
            overriding_var_list: 0
        }
    }
}

struct ClassTable {
    entries: Vec<ClassEntry>
}

impl ClassTable {

}

impl Readable for ClassEntry {
    fn read(&mut self, ctx: &mut Context) {
        ctx.id(&mut self.name);
        ctx.id(&mut self.parent);
        ctx.id(&mut self.obj_name);
        ctx.id(&mut self.description);
        ctx.id(&mut self.icon);
        ctx.id(&mut self.icon_state);
        ctx.u8(&mut self.direction);

        if ctx.v_gen > 307 {
            ctx.u8(&mut (self.dm_special_type as u8));
            self.dm_special_type_long = self.dm_special_type == 0x0F;
            if self.dm_special_type_long {
                ctx.u32(&mut self.dm_special_type);
            }
        } else {
            self.dm_special_type = 1;
        }

        ctx.id(&mut self.text);

        if ctx.v_rhs >= 494 {
            ctx.id(&mut self.id_h);
            ctx.u16(&mut self.unk_b);
            ctx.u16(&mut self.unk_c);
        }

        if ctx.v_rhs >= 508 {
            ctx.u16(&mut self.unk_d);
            ctx.u16(&mut self.unk_e);
        }

        ctx.id(&mut self.suffix);

        if ctx.v_gen >= 306 {
            ctx.u32(&mut self.flags);
        } else {
            ctx.u32(&mut self.flags);
            self.flags &= 0xFF;
        }

        ctx.id(&mut self.verb_table);
        ctx.id(&mut self.proc_table);
        ctx.id(&mut self.initializer);
        ctx.id(&mut self.initalized_vars_table);
        ctx.id(&mut self.var_table);

        if ctx.v_gen >= 267 {
            ctx.f32(&mut self.layer);
        }

        if ctx.v_rhs >= 500 {
            ctx.u8(&mut self.has_floats);
            if self.has_floats != 0 {
                ctx.floats(&mut self.floats);
            }
        }

        if ctx.v_rhs >= 509 {
            ctx.u8(&mut self.has_even_more_floats);
            if self.has_even_more_floats != 0 {
                ctx.floats(&mut self.even_more_floats);
            }
        }

        if ctx.v_gen >= 306 {
            ctx.id(&mut self.overriding_var_list)
        }
    }
}

impl Readable for ClassTable {
    fn read(&mut self, ctx: &mut Context) {
        let count : usize = 0;
        ctx.u32(&mut (count as u32));
        for i in 0..count {
            self.entries.push(ClassEntry::new());
            self.entries[i].read(ctx);
        }
    }
}
