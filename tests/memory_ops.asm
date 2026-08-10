:start
        push1 0x24
        alloc

        push1 0x00
        push1 0x11
        write1

        push1 0x01
        push2 0x2211
        write2

        push1 0x03
        push3 0x332211
        write3

        push1 0x06
        push4 0x44332211
        write4

        push1 0x0a
        push5 0x5544332211
        write5

        push1 0x0f
        push6 0x665544332211
        write6

        push1 0x15
        push7 0x77665544332211
        write7

        push1 0x1c
        push8 0x8877665544332211
        write8

        push1 0x00
        read1

        push1 0x01
        read2

        push1 0x03
        read3

        push1 0x06
        read4

        push1 0x0a
        read5

        push1 0x0f
        read6

        push1 0x15
        read7

        push1 0x1c
        read8
