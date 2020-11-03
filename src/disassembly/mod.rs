mod dmb;

const DEBUG_ENABLED : bool = false;

trait ReadContext<'a> {
    fn u8(&mut self, i: &mut u8);
    fn u16(&mut self, i: &mut u16);
    fn u32(&mut self, i: &mut u32);
    fn f32(&mut self, f : &mut f64);
    fn ints(&mut self, group : &mut [u32]);
    fn floats(&mut self, group : &mut [f64]);
    fn bytes(&mut self, group : &mut [u8]);
}

trait Readable {
    fn read(&mut self, ctx : &mut Context);
}

pub struct Context<'a> {
    debug : bool,
    data : &'a[u8],
    position: usize,
    base_pointer: u32,
    large_object_ids: bool,
    v_gen: u16, v_lhs: u16, v_rhs: u16
}

impl<'a> Context<'a> {
    pub fn new(bytes: &'a[u8]) -> Context {
        Context {
            debug: DEBUG_ENABLED,
            position: 0,
            base_pointer: 0,
            data: bytes,
            large_object_ids: false,
            v_gen: 0, v_lhs: 0, v_rhs: 0
        }
    }
}

impl Context<'_> {
    pub fn section(&self, text: &str) {
        if self.debug {
            println!("{} + @ 0x {:X}", text, self.position)
        }
    }
    fn id(&mut self, container: &mut u32) {
        if self.large_object_ids {
            return self.u32(container);
        }
        self.u16(&mut (*container as u16));
        (*container) = (*container) & 0xFFFF;
    }
}

impl<'a> ReadContext<'a> for Context<'a> {
    fn u8(&mut self, i: &mut u8) {
        (*i) = self.data[self.position].clone();
        self.position += 1;
    }
    fn u16(&mut self, i: &mut u16) {
        let mut byte : u8 = 0;
        self.u8(&mut byte);
        let lower = byte as u16;
        self.u8(&mut byte);
        (*i) = (byte as u16) << 8 | lower;
    }
    fn u32(&mut self, i: &mut u32) {
        let mut short : u16 = 0;
        self.u16(&mut short);
        let lower = short as u32;
        self.u16(&mut short);
        (*i) = ((short as u32) << 16) | lower;
    }
    fn f32(&mut self, f: &mut f64) {
        let mut int: u32 = 0;
        self.u32(&mut int);
        (*f) = int as f64;
    }
    fn ints(&mut self, group : &mut [u32]) {
        for i in 0..group.len() {
            self.u32(&mut group[i])
        }
    }
    fn floats(&mut self, group : &mut [f64]) {
        for i in 0..group.len() {
            self.f32(&mut group[i])
        }
    }
    fn bytes(&mut self, group : &mut [u8]) {
        for i in 0..group.len() {
            self.u8(&mut group[i])
        }
    }
}
