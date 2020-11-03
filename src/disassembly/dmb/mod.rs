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

use super::ReadContext;
use super::Readable;
use super::Context;

pub struct Dmb {
    header: header::Header,
    grid: grid::Grid,
    total_size_of_all_strings: u32,
}

impl<'a> Dmb {
    pub fn new() -> Dmb {
        Dmb {
           header: header::Header::new(),
           grid: grid::Grid::new(),
           total_size_of_all_strings: 0
        }
    }

    pub fn read(&mut self, ctx: &mut Context) {
        ctx.section("header");
        self.header.read(ctx);
        ctx.section("grid");
        self.grid.read(ctx);
        ctx.u32(&mut self.total_size_of_all_strings);
        if ctx.debug {
            println!("total_size_of_all_strings: {}", self.total_size_of_all_strings);
        }
        
    }
}
