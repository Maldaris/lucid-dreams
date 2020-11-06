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

const OBJ_NULL : u32 = 0xFFFF;

pub struct Dmb {
    header: header::Header,
    grid: grid::Grid,
    total_size_of_all_strings: u32,
    class_table: class_table::ClassTable,
    mob_type_table: mob_type_table::MobTypeTable,
    string_table: string_table::StringTable,
    list_table: list_table::ListTable,
    proc_table: proc_table::ProcTable,
    var_table: var_table::VarTable,
    subblock_7: subblock_7::SubBlock7,
    instances: instance_table::InstanceTable,
    map_addtl_data: map_addtl_data::MapAddtlData,
    std_object_ids: std_object_ids::StandardObjectIds,
    cache_file_table: cache_file_table::CacheFileTable,
}

impl<'a> Dmb {
    pub fn new() -> Dmb {
        Dmb {
           header: header::Header::new(),
           grid: grid::Grid::new(),
           total_size_of_all_strings: 0,
           class_table: class_table::ClassTable::new(),
           mob_type_table: mob_type_table::MobTypeTable::new(),
           string_table: string_table::StringTable::new(),
           list_table: list_table::ListTable::new(),
           proc_table: proc_table::ProcTable::new(),
           var_table: var_table::VarTable::new(),
           subblock_7: subblock_7::SubBlock7::new(),
           instances: instance_table::InstanceTable::new(),
           map_addtl_data: map_addtl_data::MapAddtlData::new(),
           std_object_ids: std_object_ids::StandardObjectIds::new(),
           cache_file_table: cache_file_table::CacheFileTable::new(),
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
        ctx.section("class_table");
        self.class_table.read(ctx);
        if ctx.debug {
            println!("class_table: {}", self.class_table.size());
        }
        ctx.section("mob_types_table");
        self.mob_type_table.read(ctx);
        if ctx.debug {
            println!("mob_types_table: {}", self.mob_type_table.size());
        }
        ctx.section("strings");
        self.string_table.read(ctx);
        if ctx.debug {
            println!("strings hash match embedded?: {}", self.string_table.hash_match());
        }
        ctx.section("list_table");
        self.list_table.read(ctx);        
        if ctx.debug {
            println!("list_table: {}", self.list_table.size());
        }

        ctx.section("proc_table");
        self.proc_table.read(ctx);

        ctx.section("var_table");
        self.var_table.read(ctx);

        ctx.section("subblock_7");
        self.subblock_7.read(ctx);

        ctx.section("instances");
        self.instances.read(ctx);

        ctx.section("map_addtl_data");
        self.map_addtl_data.read(ctx);

        ctx.section("std_object_ids");
        self.std_object_ids.read(ctx);
        
        ctx.section("cache_file_table");
        self.cache_file_table.read(ctx);
    }
}
