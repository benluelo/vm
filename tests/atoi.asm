; given a non-empty input of digits, convert it into an integer mod 2 ** 64

:start                            ; []
        ; ensure that the data isn't empty
        dlen                      ; [dlen]
        pushl @atoi               ; [dlen, @atoi]
        jnz
        push1 0xab                ; [0xab]
        trap

; assumptions: dlen is not zero
:atoi                             ; []
        ; i
        push1 0x00                ; [i]
        ; n
        push1 0x00                ; [i, n]

        :loop                     ; [i, n]
                ; read i'th byte
                push1 0x01        ; [i, n, 1]
                dup               ; [i, n, i]

                ; read the current digit at i
                dread1            ; [i, n, ascii]

                ; convert the digit on the stack to it's numerical value
                pushl @digit_atoi ; [i, n, ascii, @digit_atoi]
                call              ; [i, n, digit]

                ; build place value for digit (i.e. the '2' in '1234' is 200)
                push1 0x0a        ; [i, n, digit, 10]
                dlen              ; [i, n, digit, 10, dlen]
                ; read i
                push1 0x04        ; [i, n, digit, 10, dlen, 4]
                dup               ; [i, n, digit, 10, dlen, i]
                push1 0x01        ; [i, n, digit, 10, dlen, i, 1]
                add               ; [i, n, digit, 10, dlen, i + 1]
                sub               ; [i, n, digit, 10, dlen - (i + 1)]
                exp               ; [i, n, digit, 10 ** dlen - (i + 1)]
                mul               ; [i, n, digit * (10 ** dlen - (i + 1))]
                ; update n in place
                add               ; [i, n]

                ; read i
                push1 0x01        ; [i, n, 1]
                dup               ; [i, n, i]

                ; add 1
                push1 0x01        ; [i, n, i, 1]
                add               ; [i, n, i + 1]

                ; dup new i
                push1 0x00        ; [i, n, i + 1, 0]
                dup               ; [i, n, i + 1, i + 1]

                ; update i with i + 1
                push1 0x02        ; [i, n, i + 1, i + 1, 2]
                swap              ; [i, n, i + 1, i]
                pop               ; [i, n, i + 1]

                dlen              ; [i, n, i + 1, dlen]
                neq               ; [i, n, (i + 1) != dlen]

                ; jump to beginning of loop if the value on the top of the
                ; stack is non-zero
                pushl @loop       ; [i, n, i + 1 != dlen, @loop]
                jnz

                ; end loop

                ; init memory for value
                push1 0x08        ; [i, n, 8]
                alloc             ; [i, n]

                ; write n to memory
                push1 0x00        ; [i, n, 0]
                write8            ; [i, n]

                push1 0x08        ; [i, n, 8]
                push1 0x00        ; [i, n, 8, 0]
                exit

; converts from the ascii representation of a digit to it's value
; will trap if the value is outside the range 0x30-0x39 inclusive (0-9)
; stack values:
; [0]: the return address
; [1]: the ascii byte to convert
:digit_atoi                       ; [..., ascii, @ret]
        ; check if < b'0'
        push1 0x01                ; [..., ascii, @ret, 1]
        dup                       ; [..., ascii, @ret, ascii]
        push1 0x30                ; [..., ascii, @ret, ascii, 0x30]
        lt                        ; [..., ascii, @ret, ascii < 0x30]

        ; check if > b'9'
        push1 0x02                ; [..., ascii, @ret, ascii < 0x30, 2]
        dup                       ; [..., ascii, @ret, ascii < 0x30, ascii]
        push1 0x39                ; [..., ascii, @ret, ascii < 0x30, ascii, 0x39]
        gt                        ; [..., ascii, @ret, ascii < 0x30, ascii > 0x39]

        ; combine both comparisons
        add                       ; [..., ascii, @ret, (ascii < 0x30) + (ascii > 0x39)]
                                  ;                    (0x30 >= ascii >= 0x39)

        ; check if above checks are false
        push1 0x00                ; [..., ascii, @ret, (0x30 >= ascii >= 0x39), 0]
        eq                        ; [..., ascii, @ret, !(0x30 >= ascii >= 0x39)]

        pushl @digit_atoi_ret     ; [..., ascii, @ret, !(0x30 >= ascii >= 0x39), @digit_atoi_ret]
        jnz
        push1 0xac                ; [..., ascii, @ret, 0xac]
        trap

        :digit_atoi_ret           ; [..., ascii, @ret]
                push1 0x00        ; [..., ascii, @ret, 0]
                swap              ; [..., @ret, ascii]
                push1 0x30        ; [..., @ret, ascii, 0x30]
                sub               ; [..., @ret, ascii - 0x30]
                                  ;             digit
                push1 0x00        ; [..., @ret, digit, 0]
                swap              ; [..., ascii, @ret]
                jump              ; [..., ascii]
