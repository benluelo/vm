:start
        ; init counter
        push1 0x00
        ; begin loop
:loop
        ; add 1
        push1 0x01
        add
        ; loop check
        push1 0x00
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
        push1 0x00
        write1
        push1 0x01 ; len
        push1 0x00 ; ptr
        exit
