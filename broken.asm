:start
        pushl @ROOT_MAIN
        jump
:ROOT_MAIN
        push1 0x01 ; [1] 
        push0      ; [1, 0]
        push0      ; [1, 0, 0]
:MAIN_loop_start_a__28__29_
:MAIN_if_cond__37__39_
        push0      ; [1, 0, 0, 0]
        dup        ; [1, 0, 0, 0]
        push1 0x18 ; [1, 0, 0, 0, 24]
        lt         ; [1, 0, 0, true]
        not        ; [1, 0, 0, false]
        pushl @MAIN_if_tail_block__126__137_ ; [x, y, t, false, @tail]
        jnz
:MAIN_if_block__53__113_
        push1 0x02 ; [1, 0, 0, 2]
        push1 0x03 ; [1, 0, 0, 2, 3]
        dup        ; [1, 0, 0, 2, 1']
        mul        ; [1, 0, 0, 2]
        push1 0x03 ; [1, 0, 0, 2, 3]
        push1 0x03 ; [1, 0, 0, 2, 3, 3]
        dup        ; [1, 0, 0, 2, 3, 0]
        mul        ; [1, 0, 0, 2, 0]
        add        ; [1, 0, 0, 2]
        push1 0x05 ; [1, 0, 0, 2, 5]
        mod        ; [1, 0, 0, 2]
        push1 0x01 ; [1, 0, 0, 2, 1]
        dup        ; [1, 0, 0, 2, 0]
        push1 0x01 ; [1, 0, 0, 2, 0, 1]
        add        ; [1, 0, 0, 2, 1]
        push1 0x01 ; [1, 0, 0, 2, 1, 1]
        swap       ; [1, 0, 1, 2, 0]
        pop        ; [1, 0, 1, 2]
        pop
        pushl @MAIN_if_tail_end__126__137_
        jump
:MAIN_if_tail_block__126__137_
:MAIN_drop__a__loop_break__133__134____
        pushl @MAIN_loop_end_a__28__29_
        jump
:MAIN_if_tail_end__126__137_
:MAIN_drop__a__loop_exit__28__29____
        pushl @MAIN_loop_start_a__28__29_
        jump
:MAIN_loop_end_a__28__29_
