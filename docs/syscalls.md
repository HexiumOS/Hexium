# Syscall API for Infinity oS

## The basics

The Infinity OS syscall API is very similar to Linux to provide cross-os compability. I recommend learning about Linux syscalls before these.
The syscall arguments will be passed in this order of registers:

* `rax`: The function ID
* `rdi`: Argument 0
* `rsi`: Argument 1
* `rdx`: Argument 2
* `r10`: Argument 3
* `r8`: Argument 4
* `r9`: Argument 5

## Syscalls

| Name  | Number (`rax`) | Description                 | Arg 0\n(`rdi`)         | Arg 1\n(`rsi`)     | Arg 2\n(`rdx`)  | Arg 3\n(`r10`) | Arg 4\n(`r8`) | Arg 5\n(`r9`)     |
|-------|----------------|-----------------------------|--------------------------|----------------------|-------------------|------------------|-----------------|---------------------|
| Read  | 0              | Read from a file descriptor | unsigned int fd          | char *buf            | size_t count      |                  |                 |                     |
| Write | 1              | Write to a file descriptor  | unsigned int fd          | const char *buf      | size_t count      |                  |                 |                     |
