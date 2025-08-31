# Hexium Project TODOs

Welcome to the Hexium project!  

> **Note:** Please add todos here so everyone can see your vision and understand why things are being done.  
> Cross out completed tasks using `~~task~~`.  
> Do **not remove** any items; add new ones below.  
> Click on the links to see **detailed definitions** for each task.

---

## 🔧 Core OS Features / Roadmap

| Task | Status |
|------|--------|
| [Basic project and bootloader setup using Limine](#basic-project-and-bootloader-setup-using-limine) | ✅ Done |
| [Serial output using custom UART 16650 driver](#serial-output-using-custom-uart-16650-driver) | ✅ Done |
| [Custom global descriptor table without the x86_64 crate](#custom-global-descriptor-table-without-the-x86_64-crate) | ✅ Done |
| [Custom task state segment without the x86_64 crate](#custom-task-state-segment-without-the-x86_64-crate) | Pending |
| [Interrupt Descriptor Table without the x86_64 crate](#interrupt-descriptor-table-without-the-x86_64-crate) | ✅ Done |
| [Hardware interrupts using a custom pic8259 driver](#hardware-interrupts-using-a-custom-pic8259-driver) | Pending |
| [Clock using CMOS RTC and PIT](#clock-using-cmos-rtc-and-pit) | Pending |
| [Global heap allocator](#global-heap-allocator) | ✅ Done |
| [Physical memory management (bitmap frame allocator)](#physical-memory-management-bitmap-frame-allocator) | ✅ Done |
| [Paging (including custom kernel mapping)](#paging-including-custom-kernel-mapping) | Pending |
| [Virtual memory management](#virtual-memory-management) | Pending |
| [Initial ramdisk using UStar as the filesystem](#initial-ramdisk-using-ustar-as-the-filesystem) | Pending |
| [Virtual filesystem implementation](#virtual-filesystem-implementation) | Pending |
| [Preemptive scheduler](#preemptive-scheduler) | Pending |
| [System calls](#system-calls) | Pending |
| [ELF parsing and loading (might use a crate for this)](#elf-parsing-and-loading-might-use-a-crate-for-this) | Pending |
| [Switching to ring 3 (usermode)](#switching-to-ring-3-usermode) | Pending |
| [Standard library for userspace (Both for Rust and C)](#standard-library-for-userspace-both-for-rust-and-c) | Pending |
| [Basic disk driver such as ATA PIO](#basic-disk-driver-such-as-ata-pio) | Pending |
| [ACPI table parsing (using uACPI or acpi crate)](#acpi-table-parsing-using-uacpi-or-acpi-crate) | Pending |
| [ACPI shutdown](#acpi-shutdown) | Pending |
| [Initialization system](#initialization-system) | Pending |
| [HexaShell port](#hexashell-port) | Pending |
| [HexUtils port](#hexutils-port) | Pending |
| [Terminal text editor](#terminal-text-editor) | Pending |
| [Release 0.1.0!](#release-010) | Pending |

---

## 🔍 Detailed Definitions

### Basic project and bootloader setup using Limine
Define project structure, bootloader integration, linker script, entry point, and startup code.

### Serial output using custom UART 16650 driver
Implement basic serial output for debugging via COM ports.

### Custom global descriptor table without the x86_64 crate
Set up GDT manually with code/data segments, TSS, and descriptors.

### Custom task state segment without the x86_64 crate
Define TSS manually to manage stack switching and privilege transitions.

### Interrupt Descriptor Table without the x86_64 crate
Set up IDT manually, define ISRs, and link them to PIC.

### Hardware interrupts using a custom pic8259 driver
Initialize and handle PIC interrupts, mask/unmask IRQs.

### Clock using CMOS RTC and PIT
Implement timekeeping, periodic timer, and RTC read functions.

### Global heap allocator
Design and implement a heap allocator for dynamic memory allocation.

### Physical memory management (bitmap frame allocator)
Implement frame allocation and deallocation with a bitmap.

### Paging (including custom kernel mapping)
Set up paging tables, map kernel memory, and enable MMU.

### Virtual memory management
Implement higher-level abstraction for user and kernel memory.

### Initial ramdisk using UStar as the filesystem
Load initial filesystem into memory from UStar archive.

### Virtual filesystem implementation
Implement VFS layer to abstract filesystem operations.

### Preemptive scheduler
Implement multitasking and context switching between processes.

### System calls
Define syscall interface and handlers.

### ELF parsing and loading (might use a crate for this)
Load ELF binaries into memory and prepare for execution.

### Switching to ring 3 (usermode)
Transition tasks to usermode with proper segment selectors.

### Standard library for userspace (Both for Rust and C)
Provide minimal stdlib for user programs.

### Basic disk driver such as ATA PIO
Implement block-level read/write for disks.

### ACPI table parsing (using uACPI or acpi crate)
Parse ACPI tables to detect hardware info.

### ACPI shutdown
Implement proper system shutdown via ACPI.

### Initialization system
Define init process to start all essential services.

### HexaShell port
Port or implement shell to interact with OS.

### HexUtils port
Port utility programs for the OS environment.

### Terminal text editor
Implement basic text editor in terminal.

### Release 0.1.0!
Finalize initial release for testing and distribution.
