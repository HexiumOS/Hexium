#Hexium OS 

Welcome to **Hexium OS**, an experimental operating system written from scratch in Rust. This project explores the powerful capabilities of Rust for low-level systems programming and serves as a modern platform for learning, experimentation, and innovation. Whether you're an experienced systems developer or just curious about how operating systems work, you've come to the right place!

## 📋 What You'll Need (Prerequisites)

Before you can build Hexium OS, you'll need to set up your development environment. This project relies on specific tools to compile the code, create a bootable image, and run the OS in a virtual machine.

### 1. The Rust Nightly Toolchain 🦀

Hexium OS uses cutting-edge features of the Rust language that are not yet available in the stable version. This is common for OS development. Therefore, you need the nightly version of the Rust toolchain.

If you don't have Rust installed, get it from [rustup.rs](https://rustup.rs).

Once rustup is installed, run this command in your terminal to install the specific nightly version and its components:

    rustup install nightly-2025-04-03
    Set this specific version as the default for this project:

    # Run this inside the Hexium OS project directory
    rustup override set nightly-2025-04-03
    This ensures you are using the exact same tested toolchain as the project, preventing unexpected build errors.

### 2. Build Tools \& Emulator 🛠️

You'll need a few command-line tools. Below are installation instructions for common operating systems.

- **gmake**: A tool that automates the build process by running the commands in the Makefile.
- **xorriso**: A utility that creates the final bootable .iso disk image from our compiled code.
- **qemu**: A powerful and popular machine emulator. It acts like a virtual computer, allowing us to run and test our OS without needing to install it on real hardware.


#### On Linux (Debian / Ubuntu)

    sudo apt update
    sudo apt install make xorriso qemu-system-x86
    
#### On Linux (Fedora)

    sudo dnf install make xorriso qemu-system-x86
    
#### On Linux (Arch)

    sudo pacman -Syu make libisoburn qemu-system-x86
    
#### On macOS (using Homebrew)

    brew install make xorriso qemu
    
#### On Windows (using WSL)

The best way to build on Windows is by using the Windows Subsystem for Linux (WSL). Once you have WSL set up (we recommend the Ubuntu distribution), open your WSL terminal and follow the Linux (Debian/Ubuntu) instructions above.

## 🚀 Step-by-Step: Building and Running Hexium OS

With the prerequisites installed, you're ready to build and run the OS!

### Step 1: Clone the Repository

First, download the source code from GitHub.

    git clone https://github.com/HexiumOS/Hexium.git
    cd hexium-os
    
### Step 2: Build the Operating System

Now, let's compile the code and package it into a bootable disk image. The Makefile in the project automates all the complex steps for you. Simply run:

    make
    This command will compile the Rust kernel, link all the pieces together, and use xorriso to create a bootable ISO file named `hexium-x86_64.iso` in the project's `build/` directory.

### Step 3: Run Hexium OS in the QEMU Emulator

After the build is successful, you can boot the OS in QEMU.

    make run
    This command will launch a QEMU virtual machine window, which will boot from the .iso file you just created. You should see Hexium OS starting up!

To shut down the virtual machine, simply close the QEMU window.

## 📜 License

This project is licensed under the GNU General Public License v3.0. See the `COPYING` file for the full details.
