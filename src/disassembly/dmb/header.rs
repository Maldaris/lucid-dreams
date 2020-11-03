use super::Context;

const DEFAULT_VERSION : &'static u8 = &(230 as u8);

pub struct Header {
   vGEN: u8,
   vLHS: u8,
   vRHS: u8,
   flags: i32,
   exFlags: i32,
}

impl Header {
    pub fn new() -> Header {
       Header {
           vGEN: *DEFAULT_VERSION,
           vLHS: *DEFAULT_VERSION,
           vRHS: *DEFAULT_VERSION,
           flags: 0,
           exFlags: 0,
       }
    }

    pub fn read(&mut self, mut ctx: &Context) {
        loop {

        }
    }
}