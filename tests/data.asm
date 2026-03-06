:start
        push1 0x00
        dread4
        push1 0x04
        dread4
        add
        push1 0x05
        alloc
        push1 0x00
        write5
        push1 0x05 ; len
        push1 0x00 ; ptr
        exit
