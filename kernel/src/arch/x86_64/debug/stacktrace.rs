use crate::arch::{addr::VirtAddr, boot::KERNEL_FILE_REQUEST, stdext::U64Ext};
use core::{arch::asm, slice::from_raw_parts};
use elf::{
    ElfBytes,
    endian::{AnyEndian, LittleEndian},
    file::Class,
    symbol::SymbolTable,
};
use rustc_demangle::demangle;

pub fn stacktrace<F>(f: F)
where
    F: Fn(usize, VirtAddr, &dyn core::fmt::Display),
{
    // Kernel file info
    let kernel_file_response = KERNEL_FILE_REQUEST.get_response().unwrap();
    let kernel_file_addr = VirtAddr::from_ptr(kernel_file_response.file().addr());
    let kernel_file_size = kernel_file_response.file().size().into_usize();

    // Kernel file data
    let kernel_file_bytes =
        unsafe { from_raw_parts(kernel_file_addr.as_mut_ptr::<u8>(), kernel_file_size) };
    let elf_file = ElfBytes::<AnyEndian>::minimal_parse(kernel_file_bytes).unwrap();

    // Symbol table
    let symtab_section_header = elf_file
        .section_header_by_name(".symtab")
        .unwrap()
        .expect("Kernel ELF file should contain a .symtab section");
    let symtab_section_data: (&[u8], Option<elf::compression::CompressionHeader>) =
        elf_file.section_data(&symtab_section_header).unwrap();
    let symbol_table = SymbolTable::new(LittleEndian, Class::ELF64, symtab_section_data.0);

    // String table
    let strtab_section_header = elf_file
        .section_header_by_name(".strtab")
        .unwrap()
        .expect("Kernel ELF file should contain a .strtab section");
    let string_table = elf_file
        .section_data_as_strtab(&strtab_section_header)
        .unwrap();

    let my_rbp: *const u64;
    unsafe {
        asm!(
            "mov {}, rbp",
            out(reg) my_rbp,
        );
    }

    let mut rbp = my_rbp;
    let mut count = 0;
    while !rbp.is_null() {
        let next_rbp = unsafe { *rbp };
        let instruction_pointer = unsafe { *(rbp.add(1)) };
        rbp = next_rbp as *const u64;

        let sym = symbol_table
            .iter()
            .find(|v| (v.st_value..v.st_value + v.st_size).contains(&instruction_pointer))
            .map(|s| string_table.get(s.st_name as usize).unwrap())
            .map(demangle);
        if let Some(sym) = sym {
            f(count, VirtAddr::new(instruction_pointer), &sym);
        } else {
            f(count, VirtAddr::new(instruction_pointer), &"<unknown>");
        }
        count += 1;
    }
}
