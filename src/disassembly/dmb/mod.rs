mod header;
mod grid;
mod class_table;
mod mob_type_table;
mod string_table;
mod list_table;
mod proc_table;
mod var_table;
mod subblock_7;
mod instance_table;
mod map_addtl_data;
mod std_object_ids;
mod cache_file_table;

const DEBUG_ENABLED : bool = false;

pub struct DMB {
    header: header::Header,

}

impl<'a> DMB {
    pub fn new(bytes: &'a [i8]) -> DMB {
        let rc : Context<'a> = Context::new(bytes, DEBUG_ENABLED);

        let mut ret = DMB {
           header: header::Header::new()
        };

        ret.read(rc);

        ret
    }

    pub fn read(&mut self, ctx: Context<'a>) {
        self.header.read(&ctx);
    }
}

trait Readable {
    fn i8(&mut self, i: &mut i8);
    fn i16(&mut self, i: &mut i16);
    fn i32(&mut self, i: &mut i32);
    fn f32(&mut self, f : &mut f32);
    fn ints(&mut self, group : &[i32]);
    fn floats(&mut self, group : &[f32]);
    fn bytes(&mut self, group : &[i8]);
}

pub struct Context<'a> {
    debug : bool,
    data : &'a[i8],
    position: usize,
    largeObjectIds: bool,
}

impl<'a> Context<'a> {
    pub fn new(bytes: &[i8], dbg: bool) -> Context {
        Context {
            debug: dbg,
            position: 0,
            data: bytes,
            largeObjectIds: false
        }
    }
}

impl Context<'_> {
    
    pub fn section(&self, text: &str) {
        if self.debug {
            println!("{} + @ 0x {:X}", text, self.position)
        }
    }
    fn id(&mut self, container: &mut i32) {
        if self.largeObjectIds {
            return self.i32(container);
        }
        self.i16(&mut (*container as i16));
        (*container) = (*container) & 0xFFFF;
    }
}

impl<'a> Readable for Context<'a> {
    fn i8(&mut self, i: &mut i8) {
        (*i) = self.data[self.position];
        self.position += 1;
    }
    fn i16(&mut self, i: &mut i16) {
        let mut byte : i8 = 0;
        self.i8(&mut byte);

    }
    fn i32(&mut self, i: &mut i32) {
        let mut short : i16 = 0;
        self.i16(&mut short);
        let lower = (short & 0xFFFF) as i32;
        self.i16(&mut short);
        (*i) = (((short & 0xFFFF) as i32) << 16) | lower;
    }
    fn f32(&mut self, f: &mut f32) {

    }
    fn ints(&mut self, group : &[i32]) {

    }
    fn floats(&mut self, group : &[f32]) {

    }
    fn bytes(&mut self, group : &[i8]) {

    }
}
