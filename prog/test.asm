; Demo program to test the ADDR instruction

add_stack:
    push %10            ; push 10 onto the stack
    push %24            ; push 24 onto the stack
    adds                ; add the top two values on the stack

pop B                   ; pop the result into B

another_add:
    push %5             ; push 5 onto the stack
    push %22            ; push 22 onto the stack
    adds                ; add the top two values on the stack

pop C                   ; pop the result into C

push %100               ; push 100 onto the stack

popping:
    pop A               ; pop the value into A
    pop B               ; pop the value into B

push %10                ; push 10 onto the stack
pop a                   ; pop the value into A
push %20                ; push 20 onto the stack
pop B                   ; pop the value into B

nop_test:
    nop                 ; do nothing
    nop                 ; do nothing
    nop                 ; do nothing

addr A B                ; add the values in A and B, result in A

pushr A                 ; push the result in A onto the stack
pop B                   ; pop the result into B

test_newer_registers:
    push %10            ; push 10 onto the stack
    push %20            ; push 20 onto the stack
    adds                ; add the top two values on the stack
    pop R0              ; pop the result into R0
    pushr R0            ; push the result in R0 onto the stack
    pop R4              ; pop the result into R4

; Now both A and B should contain the sum of their original values

sig $09             ; signal to the monitor that the program is done
