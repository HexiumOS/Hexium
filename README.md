# **InfinityOS**

Welcome to **InfinityOS**, an experimental operating system written in Rust. This project explores the boundaries of Rust in systems programming and serves as a platform for learning and innovation.

## **🛠️ Features**

- :white_check_mark: Limine Bootloader
- :white_check_mark: Flanterm terminal
- :x: Global Descriptor Table
- :x: Interrupts
- :x: Keyboard Driver
- :x: Memory Management
- :x: Shell
- :x: ACPI/AML Shutdown
- :x: CpuId Support
- :x: Serial Support
- :x: Mouse Driver
- :x: In-memory File System
- :x: Graphical Interface (GUI)
- :x: ELF Loader
- :x: Task State Segment (TSS)
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

This project requires a nightly version of Rust because it uses some unstable features. You might need to run `rustup update nightly --force` to update to the latest nightly even if some components such as `rustfmt` are missing it.

You can build the project by running:

```bash
make
```

This creates an ISO

## **🚀 Running**

You can run the disk image in [QEMU] through:

[QEMU]: https://www.qemu.org/

```bash
make run
```

## Project Structure

```bash
/docs/              # Documentation
/drivers            # Device drivers
/kernel/src/        # Kernel source code
/kernel/target/     # Kernel output directory
/limine             # Limine and UEFI binaries (generated)
/ovmf               # Virtual firmware (generated)
/scripts            # Build & helper scripts
```
