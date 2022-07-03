global _start

section .data
    SYS_READ: equ 0
    SYS_WRITE: equ 1
    SYS_EXIT: equ 60

    STDIN: equ 0
    STDOUT: equ 1

section .bss
    BUFFER: resb 30000

section .text

; Prints the current byte pointed to
writec:
    ; sys_write(fd, buf, count)
    mov rax, SYS_WRITE
    mov rdi, STDOUT
    mov rsi, r12
    ; Function only prints one char at a time
    mov rdx, 1
    syscall

    ret

readc:
    ; Read one char into r12
    ; sys_read(fd, char *buf, count)
    mov rax, SYS_READ
    mov rdi, STDIN
    mov rsi, r12
    mov rdx, 1
    syscall
READC_END:
    ret

exit:
    ; sys_exit (0)
    mov rax, SYS_EXIT
    mov rdi, 0
    syscall


_start:
    ; r12 is the pointer
    mov qword r12, BUFFER
