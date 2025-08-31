# Hexium OS – Prerequisites

Welcome to Hexium OS!
This file lists all **hardware and software requirements** for developing, building, and testing the OS, as well as step-by-step setup instructions.

***

## Development Hardware

### Minimum Requirements

- **CPU:** x86_64 processor (64-bit)
- **Internet connection** for downloading dependencies


### Recommended Specifications

- **CPU:** Modern x86_64 processor with virtualization support
- **Storage:** SSD storage for faster build times

***

## Target Hardware

### Minimum Target Requirements

- **CPU:** x86_64 processor
- **RAM:** 128 MB RAM
- **Boot:** UEFI/CSM-compatible system


### Recommended Target Requirements

- **CPU:** x86_64 processor
- **RAM:** 512 MB RAM
- **Boot:** UEFI-compatible system with Secure Boot disabled


### Optional Features

- PS/2 keyboard
- Serial port (for debugging)

***

## Hardware Compatibility

### Tested Platforms

- Common virtual machines: **QEMU**, **VirtualBox**


### Virtual Machine Requirements

- Enable hardware virtualization (VT-x/AMD-V)
- Allocate at least **128 MB RAM**

***

## Development Requirements

### Required Software

- **Rust** (nightly toolchain)
- **Cargo** package manager
- Rust `rust-src` component for `x86_64-unknown-none`
- **QEMU** emulator
- **Git** version control
- **GNU xorriso**


### Optional Tools

- **GDB** debugger
- **Rust Analyzer**
- **VirtualBox** (for alternative testing)

***

## Setup Instructions

### Step 1: Install Rust Nightly

1. **Install Rust** using [rustup.rs](https://rustup.rs):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Install the Hexium-tested nightly version:**
```bash
rustup install nightly-2025-04-03
rustup override set nightly-2025-04-03
rustup component add rust-src
```

3. **Verify installation:**
```bash
rustc --version
cargo --version
```


### Step 2: Install Build Tools and Emulator

#### Linux (Debian/Ubuntu)

```bash
sudo apt update
sudo apt install make xorriso qemu-system-x86 git
```


#### Linux (Fedora)

```bash
sudo dnf install make xorriso qemu-system-x86 git
```


#### Linux (Arch)

```bash
sudo pacman -Syu make libisoburn qemu-system-x86 git
```


#### macOS (Homebrew)

```bash
brew install make xorriso qemu git
```


#### Windows (WSL)

1. Install **WSL** (Ubuntu recommended)
2. Inside WSL, follow Debian/Ubuntu instructions

### Step 3: Optional Tools Setup

**Install GDB for debugging:**

```bash
sudo apt install gdb   # Debian/Ubuntu
```

**Install Rust Analyzer** for IDE integration: [rust-analyzer](https://rust-analyzer.github.io/)

**Install VirtualBox** if you want to test Hexium OS outside QEMU.

### Step 4: Verify Setup

```bash
rustc --version
cargo --version
qemu-system-x86_64 --version
make --version
git --version
```


### Step 5: Clone and Build Hexium OS

```bash
git clone https://github.com/HexiumOS/Hexium.git
cd hexium-os
make
make run
```

You should now see **Hexium OS** booting in the virtual machine.

***

## Tips

> **Tip:** Keep this file updated when new dependencies are added.
> Always verify your hardware and virtualization settings before building.

