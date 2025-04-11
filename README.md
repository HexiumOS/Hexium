# **Hexium OS**

Welcome to **Hexium OS**, an experimental operating system written in Rust. This project explores the boundaries of Rust in systems programming and serves as a platform for learning and innovation.

## **🛠️ Features**

- :rocket: Limine Bootloader
- :computer: Flanterm terminal
- :scroll: Global Descriptor Table  
- :zap: Interrupts
- :page_facing_up: Paging Support
- :electric_plug: Serial Support
- :white_check_mark: Limine Bootloader
- :white_check_mark: Flanterm terminal
- :white_check_mark: Global Descriptor Table
- :white_check_mark: Interrupts
- :white_check_mark: Paging Support
- :white_check_mark: Memory Management
- :white_check_mark: In-memory File System
- :white_check_mark: Task State Segment (TSS)
- :white_check_mark: Heap allocator
- :x: Keyboard Driver
- :x: Shell
- :x: ACPI/AML Shutdown
- :x: CpuId Support
- :x: Mouse Driver
- :x: Graphical Interface (GUI)
- :x: ELF Loader
- :x: Network Driver
- :x: Audio Driver
- :x: FAT32 Support
- :x: OpenGL-like API
- :x: Integrated Development Environment (IDE)
- :x: C/C++ Compiler
- :x: Processes
- :x: Multitasking
- :x: Installation Setup
- :x: Web Browser
- :x: User Documentation
- :x: Package manager

## **⚙️ Building**

This project requires a nightly version of Rust because it uses some unstable features. You might need to run `rustup update nightly --force` to update to the latest nightly even if some components such as `rustfmt` are missing it. Additionally, ensure you have `rustc` and `cargo` version 1.86 or higher installed.

You will also need `xorriso`, a tool for creating ISO images.

You can build the project by running:

```bash
make
```

This creates an ISO

## **🚀 Running**

You can run the disk image in [QEMU] through:

```bash
make run
```

## **:open_file_folder: Project Structure**

```bash
/docs/              # Documentation
/kernel/src/        # Kernel source code
/kernel/target/     # Kernel output directory
/limine             # Limine and UEFI binaries (generated)
/ovmf               # Virtual firmware (generated)
/scripts            # Build & helper scripts
```

[QEMU]: https://www.qemu.org/
