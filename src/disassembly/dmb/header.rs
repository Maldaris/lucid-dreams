extern crate easy_strings;

use easy_strings::{EZString, ez};

use super::Readable;
use super::Context;
use super::ReadContext;

const DEFAULT_VERSION : &'static u16 = &(230 as u16);

pub struct Header {
   v_gen: u16,
   v_lhs: u16,
   v_rhs: u16,
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
}

impl Readable for Header {
    fn read(&mut self, ctx: &mut Context) {
        let mut b : u8 = 0;
        let mut s = ez("");
        let mut shebang = ez("");
        let mut compat_line : bool = false;

        loop {
            (*ctx).u8(&mut b);
            if b == 10 {
                if !compat_line {
                    if s.starts_with("#") {
                        shebang += &s + &ez("\n");
                        (ctx.base_pointer) = ctx.position as u32;
                    } else {
                        let split = s.split(" v").collect::<Vec<EZString>>();
                        let tmp = split[1].parse::<u16>().unwrap();
                        self.v_rhs = tmp.clone();
                        self.v_lhs = tmp.clone();
                        self.v_gen = tmp.clone();
                        compat_line = true;
                    }
                } else {
                    let version_bits = s.split(" v")
                        .collect::<Vec<EZString>>()
                        [1].split(" ")
                        .collect::<Vec<EZString>>();
                    if version_bits.len() < 2 {
                        self.v_lhs = version_bits[0].parse::<u16>().unwrap();
                        self.v_rhs = self.v_lhs.clone();
                    } else {
                        self.v_lhs = version_bits[0].parse::<u16>().unwrap();
                        self.v_rhs = version_bits[1].parse::<u16>().unwrap();
                    }

                    break;
                }
                s = ez("");
            } else {
                s.push(b as char);
            }
        }

        ctx.u32(&mut (self.flags as u32));
        self.ex_flags = 0;
        if (self.flags as u32 & 0x80000000) != 0 {
            ctx.u32(&mut (self.ex_flags as u32));
        }

        ctx.v_gen = self.v_gen.clone();
        ctx.v_lhs = self.v_lhs.clone();
        ctx.v_rhs = self.v_rhs.clone();
        ctx.large_object_ids = (self.flags as u32 & 0x40000000) != 0;

        if ctx.debug {
            println!("version: {} {} {}", self.v_gen, self.v_lhs, self.v_rhs);
            println!("large object ids: {}", ctx.large_object_ids);
            println!("base pointer: {}", ctx.base_pointer);
        }
    }
}