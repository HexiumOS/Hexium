# **Hexium OS**

Welcome to **Hexium OS**, an experimental operating system written in Rust. This project explores the boundaries of Rust in systems programming and serves as a platform for learning and innovation

## **:hammer: Building**

This project requires a nightly version of Rust because it uses some unstable features. You might need to run `rustup update nightly --force` to update to the latest nightly even if some components such as `rustfmt` are missing it. Additionally, ensure you have `rustc` and `cargo` version 1.86 or higher and `gmake` installed.

You will also need `xorriso`, a tool for creating ISO images and `curl` for downloading dependencies

You can build the project by running:

```bash
make
```

This creates an ISO named `hexium-{arch}.iso`

## **:rocket: Running**

You can run the disk image in [QEMU] through:

```bash
make run
```

## **:scroll: License**

This project is licensed under the GNU General Public License v3.0 - see the [COPYING](COPYING) file for details.

[QEMU]: https://www.qemu.org/
