    // -
        movq    -8(%rbp), %rax
        movzbl  (%rax), %eax
        subl    $1, %eax
        movl    %eax, %edx
        movq    -8(%rbp), %rax
        movb    %dl, (%rax)
