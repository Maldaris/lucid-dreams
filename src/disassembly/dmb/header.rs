use super::Context;

const DEFAULT_VERSION : &'static i8 = &(230 as i8);

pub struct Header {
   v_gen: i8,
   v_lhs: i8,
   v_rhs: i8,
   flags: i32,
   ex_flags: i32,
}

impl Header {
    pub fn new() -> Header {
       Header {
        v_gen: *DEFAULT_VERSION,
        v_lhs: *DEFAULT_VERSION,
        v_rhs: *DEFAULT_VERSION,
           flags: 0,
           ex_flags: 0,
       }
    }

    pub fn read(&mut self, mut ctx: &Context) {
        loop {

        }
    }
}