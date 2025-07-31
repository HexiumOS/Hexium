use std::{env::var, process::Command};

fn main() {
    let arch = var("CARGO_CFG_TARGET_ARCH").unwrap();
    let out_dir = var("OUT_DIR").unwrap();

    println!("cargo:rustc-link-arg=-Tlinker-{arch}.ld");
    println!("cargo:rerun-if-changed=linker-{arch}.ld");

    if arch == "x86_64" {
        println!("cargo:rerun-if-changed=src/asm/x86_64/test.asm");

        // Assemble the assembly file into an ELF object file
        let status = Command::new("nasm")
            .arg("-f")
            .arg("elf64") // Use elf64 instead of bin
            .arg("-o")
            .arg(format!("{}/test.o", out_dir)) // Output to test.o
            .arg("src/asm/x86_64/test.asm")
            .status()
            .expect("Failed to run nasm");
        if !status.success() {
            panic!("nasm failed with exit status {}", status);
        }

        // Tell Cargo to link the object file
        println!("cargo:rustc-link-arg={}/test.o", out_dir);
    }
}
