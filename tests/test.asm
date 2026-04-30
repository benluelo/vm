:start
        ; init counter
        push0
        ; begin loop
:loop
        ; add 1
        push1 0x01
        add
        ; loop check
        push0
        dup
        push1 0x03
        sub
        ; jump to beginning of loop (offset 2 in the bytecode) if the value on the top of the
        ; stack is non-zero (value - 3)
        pushl @loop
        jnz
        ; end loop
        ; init memory for value
        push1 0x01
        alloc
        ; update value in memory
        push0
        write1
        push1 0x01 ; len
        push0      ; ptr
        exit
