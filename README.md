# Hexium OS 🌌

Welcome to **Hexium OS**, an experimental operating system written from scratch in **Rust**.  
This project explores the power of Rust for **low-level systems programming** and serves as a modern platform for **learning, experimentation, and innovation**.

Whether you're an experienced systems developer or just curious about how operating systems work, you've come to the right place!

---

## 🚀 Quick Start

### Step 1: Clone the Repository

    git clone https://github.com/HexiumOS/Hexium.git
    cd hexium-os

### Step 2: Build the OS

    make

This will compile the Rust kernel, link everything, and create a bootable ISO named `build/hexium-x86_64.iso`.

### Step 3: Run in QEMU

    make run

This will launch a QEMU VM that boots Hexium OS.  
Close the QEMU window to shut down.

---

## 📋 Prerequisites

Before building, you'll need to set up your environment (Rust nightly, build tools, and QEMU).  
See the full [`PREREQUISITES`](PREREQUISITES.md) guide.

---

## 📜 License

This project is licensed under the GNU General Public License v3.0.  
See the [`COPYING`](COPYING) file for full details.