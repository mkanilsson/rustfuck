section .data
    mem TIMES 3000 db 0

section .text
    global _start

get_char:
    push    rbp
    mov     rbp, rsp
    sub     rsp, 16

    mov qword [rbp - 8], rax              ; save the buffer pointer

    mov rax, 0                  ; read syscall
    mov rdi, 0                  ; stdin
    mov rsi, qword [rbp - 9]                 ; buffer
    mov rdx, 1                 ; count
    syscall

    mov cl, byte [rbp - 9]
    mov rax, qword [rbp - 8]
    mov byte [rax], cl

    ;; clear buffer
    jmp .CLEAR_BUFFER_CONDITION
.CLEAR_BUFFER:
    mov rax, 0                  ; read syscall
    mov rdi, 0                  ; stdin
    mov rsi, qword [rbp - 9]                ; buffer
    mov rdx, 1                 ; count
    syscall
.CLEAR_BUFFER_CONDITION:
    mov cl, byte [rbp - 9]
    cmp cl, 10                  ; ascii \n
    jne .CLEAR_BUFFER

    add     rsp, 16
    pop rbp
    ret

_start:
    ;;  initialize stack
    push    rbp
    mov     rbp, rsp
    sub     rsp, 16
    mov     dword [rbp - 4], 0

    ;; char *ptr = mem
    mov     qword [rbp - 16], mem

     ;; >
    add     qword [rbp - 16], 1

     ;; <
    sub     qword [rbp - 16], 1

    ;; ++*ptr; (+)
    mov rax, qword [rbp - 16]
    add byte [rax], 5

    ;; --*ptr; (-)
    mov rax, qword [rbp - 16]
    sub byte [rax], 1

    ;; Add 48 as ascii offset
    mov rax, qword [rbp - 16]
    add byte [rax], 48

    ;; .
    mov rax, 1                  ; write syscall
    mov rdi, 1                  ; stdout
    mov rsi, qword [rbp - 16]   ; buffer
    mov rdx, 1                 ; count
    syscall

    mov rax, qword [rbp - 16]
    call get_char

    ;; ,
    mov rax, 0                  ; read syscall
    mov rdi, 0                  ; stdin
    mov rsi, qword [rbp - 16]   ; buffer
    mov rdx, 1                 ; count
    syscall

    ;; End
    mov rax, 60                 ; exit
    mov rdi, qword [rbp - 16]
    movsx rdi, byte [rdi]
    syscall
