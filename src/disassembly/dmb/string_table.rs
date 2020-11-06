extern crate easy_strings;

use crate::algorithms::xorjump9;
use crate::algorithms::nqcrc;

use easy_strings::{EZString};

use super::Readable;
use super::Context;
use super::ReadContext;

pub struct StringTable {
    entries: Vec<EZString>,
    hash: u32
}

impl StringTable {
    pub fn new() -> StringTable {
        StringTable { entries: Vec::new(), hash: 0 }
    }

    pub fn calculate_hash(&self) -> u32 {
        let mut ret : u32 = 0;
        self.entries.iter().for_each(|entry| {
            ret = nqcrc::nqcrc_hash(ret, entry.as_bytes());
        });
        ret
    }
    pub fn hash_match(&self) -> bool { self.hash == self.calculate_hash() }
}

impl Readable for StringTable {
    fn read(&mut self, ctx: &mut Context) {
        let mut count : u32 = 0;
        ctx.id(&mut count);
        for _i in 0..count {
            let mut total_length : u32 = 0;
            loop {
                let mut val : u16 = ctx.position as u16 - ctx.base_pointer as u16;
                let mut tmp : u16 = 0;
                ctx.u16(&mut tmp);
                val ^= tmp;
                total_length += val as u32;
                if val as i16 != -1 as i16 {
                    break;
                }
            }
            let mut str : String = String::new();
            str.reserve(total_length as usize);
            let key : u8 = 0;
            for _j in 0..total_length {
                let mut byte : u8 = 0;
                ctx.u8(&mut byte);
                str.push(byte as char);
            }
            let mut bytes : Vec<u8> = str.as_bytes().to_vec();
            xorjump9::xor_jump_9(&mut bytes, key);
            self.entries.push(EZString::from(String::from_utf8(bytes).unwrap()));
        }
        if ctx.v_gen > 468 {
            ctx.u32(&mut self.hash);
        }
    }
}