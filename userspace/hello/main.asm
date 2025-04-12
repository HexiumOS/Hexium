%define ENDL 10

global _start

section .text

_start:
    mov rax, 1
    mov rdi, 1
    mov rsi, msg_hello
    mov rdx, 14
    syscall

    mov rax, 60
    mov rdi, 0
    syscall


section .data
    msg_hello: db "Hello, world!", ENDL
