; Demo program to test the ADDR instruction

PUSH #10            ; push 10 onto the stack
PUSH #24            ; push 24 onto the stack
ADDS                ; add the top two values on the stack

POP B               ; pop the result into B

PUSH #5             ; push 5 onto the stack
PUSH #22            ; push 22 onto the stack
ADDS                ; add the top two values on the stack

POP C               ; pop the result into C

PUSH #100           ; push 100 onto the stack

POP A               ; pop the value into A
POP B               ; pop the value into B

PUSH #10            ; push 10 onto the stack
POP A               ; pop the value into A
PUSH #20            ; push 20 onto the stack
POP B               ; pop the value into B

NOP                 ; do nothing
NOP                 ; do nothing
NOP                 ; do nothing

ADDR A B            ; add the values in A and B, result in A

PUSHR A             ; push the result in A onto the stack
POP B               ; pop the result into B

; Now both A and B should contain the sum of their original values

SIG $09             ; signal to the monitor that the program is done
