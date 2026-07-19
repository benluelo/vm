[2m2026-07-19T19:31:17.985559Z[0m [32m INFO[0m [2mvm[0m[2m:[0m ran const prop/eval loop 50 times
[2m2026-07-19T19:31:17.987981Z[0m [32m INFO[0m [2mvm[0m[2m:[0m ran const prop/eval loop 2 times
[2m2026-07-19T19:31:18.016641Z[0m [32m INFO[0m [2mvm[0m[2m:[0m ran const prop/eval loop 2 times
:@start
	pushl @$ROOT/MAIN:0
	jump
:$ROOT/MAIN/MAIN:def_load64_0_0:0
:MAIN:def_load64_0_0
:MAIN:def_load64_0_0/RETS_INIT
	push0
:MAIN:def_load64_0_0/BODY
	push1 0x02
	dup
	read8
	push0
	swap
	pop
	push0
	dup
	push1 0x08
	shl
	push8 0xff00ff00ff00ff00
	and
	push1 0x01
	dup
	push1 0x08
	shr
	push7 0xff00ff00ff00ff
	and
	or
	push0
	swap
	pop
	push0
	dup
	push1 0x10
	shl
	push8 0xffff0000ffff0000
	and
	push1 0x01
	dup
	push1 0x10
	shr
	push6 0xffff0000ffff
	and
	or
	push0
	swap
	pop
	push0
	dup
	push1 0x20
	shl
	push1 0x01
	dup
	push1 0x20
	shr
	or
	push0
	swap
	pop
:MAIN:def_load64_0_0/CLEANUP
	push1 0x01
	swap
	pop
	jump
:$ROOT/MAIN/MAIN:def_store64_0_1:0
:MAIN:def_store64_0_1
:MAIN:def_store64_0_1/RETS_INIT
:MAIN:def_store64_0_1/BODY
	push1 0x01
	dup
	push1 0x08
	shl
	push8 0xff00ff00ff00ff00
	and
	push1 0x02
	dup
	push1 0x08
	shr
	push7 0xff00ff00ff00ff
	and
	or
	push1 0x01
	swap
	pop
	push1 0x01
	dup
	push1 0x10
	shl
	push8 0xffff0000ffff0000
	and
	push1 0x02
	dup
	push1 0x10
	shr
	push6 0xffff0000ffff
	and
	or
	push1 0x01
	swap
	pop
	push1 0x02
	dup
	push1 0x02
	dup
	push1 0x20
	shl
	push1 0x03
	dup
	push1 0x20
	shr
	or
	write8
:MAIN:def_store64_0_1/CLEANUP
	push1 0x01
	swap
	pop
	pop
	jump
:$ROOT/MAIN/MAIN:def_xor64_0_2:0
:MAIN:def_xor64_0_2
:MAIN:def_xor64_0_2/RETS_INIT
:MAIN:def_xor64_0_2/BODY
	push1 0x01
	dup
	push1 0x08
	shl
	push8 0xff00ff00ff00ff00
	and
	push1 0x02
	dup
	push1 0x08
	shr
	push7 0xff00ff00ff00ff
	and
	or
	push1 0x01
	swap
	pop
	push1 0x01
	dup
	push1 0x10
	shl
	push8 0xffff0000ffff0000
	and
	push1 0x02
	dup
	push1 0x10
	shr
	push6 0xffff0000ffff
	and
	or
	push1 0x01
	swap
	pop
	push1 0x02
	dup
	push1 0x03
	dup
	read8
	push1 0x03
	dup
	push1 0x20
	shl
	push1 0x04
	dup
	push1 0x20
	shr
	or
	xor
	write8
:MAIN:def_xor64_0_2/CLEANUP
	push1 0x01
	swap
	pop
	pop
	jump
:$ROOT/MAIN:0
	push1 0xc8
	alloc
	push1 0xc8
	push1 0x28
	alloc
	push1 0x01
	dlen
	push1 0x88
	div
	add
	push1 0xf0
	push1 0x88
	push1 0x02
	dup
	mul
	alloc
	push0
	push1 0xf0
	dlen
	dcopy
	dlen
	push1 0xf0
	add
	push1 0x06
	write1
	push1 0xef
	push1 0x88
	push1 0x03
	dup
	mul
	add
	push1 0xef
	push1 0x88
	push1 0x04
	dup
	mul
	add
	read1
	push1 0x80
	or
	write1
	push0
:MAIN:loop_start_blocks:2
	push0
	dup
	dlen
	gt
	pushl @MAIN:if_1
	jnz
	pushl @MAIN:if_0
	jump
:MAIN:if_1
:MAIN:drop::blocks::loop_break_blocks:2_3::<none>
:MAIN:drop::blocks::loop_break_blocks:2_3::blocks:2
	pushl @MAIN:loop_end_blocks:2
	jump
	pushl @MAIN:if_0
	jump
:MAIN:if_0
	push0
	push0
	read8
	push1 0xf0
	push1 0x03
	dup
	add
	read8
	xor
	write8
	push1 0x08
	push1 0x08
	read8
	push1 0xf8
	push1 0x03
	dup
	add
	read8
	xor
	write8
	push1 0x10
	push1 0x10
	read8
	push2 0x0100
	push1 0x03
	dup
	add
	read8
	xor
	write8
	push1 0x18
	push1 0x18
	read8
	push2 0x0108
	push1 0x03
	dup
	add
	read8
	xor
	write8
	push1 0x20
	push1 0x20
	read8
	push2 0x0110
	push1 0x03
	dup
	add
	read8
	xor
	write8
	push1 0x28
	push1 0x28
	read8
	push2 0x0118
	push1 0x03
	dup
	add
	read8
	xor
	write8
	push1 0x30
	push1 0x30
	read8
	push2 0x0120
	push1 0x03
	dup
	add
	read8
	xor
	write8
	push1 0x38
	push1 0x38
	read8
	push2 0x0128
	push1 0x03
	dup
	add
	read8
	xor
	write8
	push1 0x40
	push1 0x40
	read8
	push2 0x0130
	push1 0x03
	dup
	add
	read8
	xor
	write8
	push1 0x48
	push1 0x48
	read8
	push2 0x0138
	push1 0x03
	dup
	add
	read8
	xor
	write8
	push1 0x50
	push1 0x50
	read8
	push2 0x0140
	push1 0x03
	dup
	add
	read8
	xor
	write8
	push1 0x58
	push1 0x58
	read8
	push2 0x0148
	push1 0x03
	dup
	add
	read8
	xor
	write8
	push1 0x60
	push1 0x60
	read8
	push2 0x0150
	push1 0x03
	dup
	add
	read8
	xor
	write8
	push1 0x68
	push1 0x68
	read8
	push2 0x0158
	push1 0x03
	dup
	add
	read8
	xor
	write8
	push1 0x70
	push1 0x70
	read8
	push2 0x0160
	push1 0x03
	dup
	add
	read8
	xor
	write8
	push1 0x78
	push1 0x78
	read8
	push2 0x0168
	push1 0x03
	dup
	add
	read8
	xor
	write8
	push1 0x80
	push1 0x80
	read8
	push2 0x0170
	push1 0x03
	dup
	add
	read8
	xor
	write8
	push1 0x11
	push0
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push0
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push0
	swap
	pop
	push1 0x08
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push0
	swap
	pop
	push1 0x10
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push0
	swap
	pop
	push1 0x18
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push0
	swap
	pop
	push1 0x20
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x01
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x0100000000000000
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x8280000000000000
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x8a80000000000080
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push7 0x80008000000080
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x8b80000000000000
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x0100008000000000
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x8180008000000080
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x0980000000000080
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x8a00000000000000
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x8800000000000000
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x0980008000000000
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x0a00008000000000
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x8b80008000000000
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x8b00000000000080
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x8980000000000080
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x0380000000000080
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x0280000000000080
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x8000000000000080
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x0a80000000000000
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x0a00008000000080
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x8180008000000080
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x8080000000000080
	xor
	write8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x08
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x02
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x03
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x50
	push1 0x02
	dup
	push1 0x01
	shl
	push1 0x03
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x38
	push1 0x02
	dup
	push1 0x03
	shl
	push1 0x03
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x58
	push1 0x02
	dup
	push1 0x06
	shl
	push1 0x03
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x88
	push1 0x02
	dup
	push1 0x0a
	shl
	push1 0x03
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x90
	push1 0x02
	dup
	push1 0x0f
	shl
	push1 0x03
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x18
	push1 0x02
	dup
	push1 0x15
	shl
	push1 0x03
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x28
	push1 0x02
	dup
	push1 0x1c
	shl
	push1 0x03
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x80
	push1 0x02
	dup
	push1 0x24
	shl
	push1 0x03
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x40
	push1 0x02
	dup
	push1 0x2d
	shl
	push1 0x03
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa8
	push1 0x02
	dup
	push1 0x37
	shl
	push1 0x03
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xc0
	push1 0x02
	dup
	push1 0x02
	shl
	push1 0x03
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x20
	push1 0x02
	dup
	push1 0x0e
	shl
	push1 0x03
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x78
	push1 0x02
	dup
	push1 0x1b
	shl
	push1 0x03
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb8
	push1 0x02
	dup
	push1 0x29
	shl
	push1 0x03
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x98
	push1 0x02
	dup
	push1 0x38
	shl
	push1 0x03
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x68
	push1 0x02
	dup
	push1 0x08
	shl
	push1 0x03
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x60
	push1 0x02
	dup
	push1 0x19
	shl
	push1 0x03
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x10
	push1 0x02
	dup
	push1 0x2b
	shl
	push1 0x03
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xa0
	push1 0x02
	dup
	push1 0x3e
	shl
	push1 0x03
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x70
	push1 0x02
	dup
	push1 0x12
	shl
	push1 0x03
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0xb0
	push1 0x02
	dup
	push1 0x27
	shl
	push1 0x03
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x48
	push1 0x02
	dup
	push1 0x3d
	shl
	push1 0x03
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x30
	push1 0x02
	dup
	push1 0x14
	shl
	push1 0x03
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push0
	dup
	push1 0x01
	swap
	pop
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push0
	swap
	pop
	push1 0x08
	push1 0x02
	dup
	push1 0x2c
	shl
	push1 0x03
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push0
	push0
	read8
	push8 0x0100008000000000
	xor
	write8
	push1 0xc8
	push1 0xc8
	push0
	read8
	push1 0x28
	read8
	xor
	push1 0x50
	read8
	xor
	push1 0x78
	read8
	xor
	push1 0xa0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd0
	push1 0x08
	read8
	push1 0x30
	read8
	xor
	push1 0x58
	read8
	xor
	push1 0x80
	read8
	xor
	push1 0xa8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xd8
	push1 0x10
	read8
	push1 0x38
	read8
	xor
	push1 0x60
	read8
	xor
	push1 0x88
	read8
	xor
	push1 0xb0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe0
	push1 0x18
	read8
	push1 0x40
	read8
	xor
	push1 0x68
	read8
	xor
	push1 0x90
	read8
	xor
	push1 0xb8
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	push1 0x20
	read8
	push1 0x48
	read8
	xor
	push1 0x70
	read8
	xor
	push1 0x98
	read8
	xor
	push1 0xc0
	read8
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0xe8
	read8
	push1 0xd0
	read8
	push1 0x01
	shl
	push1 0xd0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x03
	swap
	pop
	push0
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x28
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x50
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x78
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa0
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc8
	read8
	push1 0xd8
	read8
	push1 0x01
	shl
	push1 0xd8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x03
	swap
	pop
	push1 0x08
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x30
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x58
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x80
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xa8
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd0
	read8
	push1 0xe0
	read8
	push1 0x01
	shl
	push1 0xe0
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x03
	swap
	pop
	push1 0x10
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x38
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x60
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x88
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb0
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xd8
	read8
	push1 0xe8
	read8
	push1 0x01
	shl
	push1 0xe8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x03
	swap
	pop
	push1 0x18
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x40
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x68
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x90
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xb8
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xe0
	read8
	push1 0xc8
	read8
	push1 0x01
	shl
	push1 0xc8
	read8
	push1 0x3f
	shr
	xor
	xor
	push1 0x03
	swap
	pop
	push1 0x20
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x48
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x70
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x98
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0xc0
	push1 0x04
	dup
	pushl @MAIN:def_xor64_0_2
	call
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x02
	swap
	pop
	push1 0x50
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x50
	push1 0x03
	dup
	push1 0x01
	shl
	push1 0x04
	dup
	push1 0x3f
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0x38
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x38
	push1 0x03
	dup
	push1 0x03
	shl
	push1 0x04
	dup
	push1 0x3d
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0x58
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x58
	push1 0x03
	dup
	push1 0x06
	shl
	push1 0x04
	dup
	push1 0x3a
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0x88
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x88
	push1 0x03
	dup
	push1 0x0a
	shl
	push1 0x04
	dup
	push1 0x36
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0x90
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x90
	push1 0x03
	dup
	push1 0x0f
	shl
	push1 0x04
	dup
	push1 0x31
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0x18
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x18
	push1 0x03
	dup
	push1 0x15
	shl
	push1 0x04
	dup
	push1 0x2b
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0x28
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x28
	push1 0x03
	dup
	push1 0x1c
	shl
	push1 0x04
	dup
	push1 0x24
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0x80
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x80
	push1 0x03
	dup
	push1 0x24
	shl
	push1 0x04
	dup
	push1 0x1c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0x40
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x40
	push1 0x03
	dup
	push1 0x2d
	shl
	push1 0x04
	dup
	push1 0x13
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0xa8
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0xa8
	push1 0x03
	dup
	push1 0x37
	shl
	push1 0x04
	dup
	push1 0x09
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0xc0
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0xc0
	push1 0x03
	dup
	push1 0x02
	shl
	push1 0x04
	dup
	push1 0x3e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0x20
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x20
	push1 0x03
	dup
	push1 0x0e
	shl
	push1 0x04
	dup
	push1 0x32
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0x78
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x78
	push1 0x03
	dup
	push1 0x1b
	shl
	push1 0x04
	dup
	push1 0x25
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0xb8
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0xb8
	push1 0x03
	dup
	push1 0x29
	shl
	push1 0x04
	dup
	push1 0x17
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0x98
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x98
	push1 0x03
	dup
	push1 0x38
	shl
	push1 0x04
	dup
	push1 0x08
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0x68
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x68
	push1 0x03
	dup
	push1 0x08
	shl
	push1 0x04
	dup
	push1 0x38
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0x60
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x60
	push1 0x03
	dup
	push1 0x19
	shl
	push1 0x04
	dup
	push1 0x27
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0x10
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x10
	push1 0x03
	dup
	push1 0x2b
	shl
	push1 0x04
	dup
	push1 0x15
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0xa0
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0xa0
	push1 0x03
	dup
	push1 0x3e
	shl
	push1 0x04
	dup
	push1 0x02
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0x70
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x70
	push1 0x03
	dup
	push1 0x12
	shl
	push1 0x04
	dup
	push1 0x2e
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0xb0
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0xb0
	push1 0x03
	dup
	push1 0x27
	shl
	push1 0x04
	dup
	push1 0x19
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0x48
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x48
	push1 0x03
	dup
	push1 0x3d
	shl
	push1 0x04
	dup
	push1 0x03
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0x30
	pushl @MAIN:def_load64_0_0
	call
	push1 0x01
	swap
	pop
	push1 0x30
	push1 0x03
	dup
	push1 0x14
	shl
	push1 0x04
	dup
	push1 0x2c
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x01
	dup
	push1 0x02
	swap
	pop
	push1 0x2c
	push0
	push1 0x08
	pushl @MAIN:def_load64_0_0
	call
	push1 0x03
	swap
	pop
	push1 0x08
	push1 0x05
	dup
	push1 0x2c
	shl
	push1 0x06
	dup
	push1 0x14
	shr
	xor
	pushl @MAIN:def_store64_0_1
	call
	push1 0x03
	dup
	push1 0x04
	swap
	pop
	push1 0x18
	push1 0xc8
	push0
	read8
	write8
	push1 0xd0
	push1 0x08
	read8
	write8
	push1 0xd8
	push1 0x10
	read8
	write8
	push1 0xe0
	push1 0x18
	read8
	write8
	push1 0xe8
	push1 0x20
	read8
	write8
	push0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x08
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x10
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x18
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x20
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x28
	read8
	write8
	push1 0xd0
	push1 0x30
	read8
	write8
	push1 0xd8
	push1 0x38
	read8
	write8
	push1 0xe0
	push1 0x40
	read8
	write8
	push1 0xe8
	push1 0x48
	read8
	write8
	push1 0x28
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x30
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x38
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x40
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x48
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x50
	read8
	write8
	push1 0xd0
	push1 0x58
	read8
	write8
	push1 0xd8
	push1 0x60
	read8
	write8
	push1 0xe0
	push1 0x68
	read8
	write8
	push1 0xe8
	push1 0x70
	read8
	write8
	push1 0x50
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x58
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x60
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x68
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x70
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0x78
	read8
	write8
	push1 0xd0
	push1 0x80
	read8
	write8
	push1 0xd8
	push1 0x88
	read8
	write8
	push1 0xe0
	push1 0x90
	read8
	write8
	push1 0xe8
	push1 0x98
	read8
	write8
	push1 0x78
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0x80
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0x88
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0x90
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0x98
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0xc8
	push1 0xa0
	read8
	write8
	push1 0xd0
	push1 0xa8
	read8
	write8
	push1 0xd8
	push1 0xb0
	read8
	write8
	push1 0xe0
	push1 0xb8
	read8
	write8
	push1 0xe8
	push1 0xc0
	read8
	write8
	push1 0xa0
	push1 0xc8
	read8
	push1 0xd0
	read8
	neg
	push1 0xd8
	read8
	and
	xor
	write8
	push1 0xa8
	push1 0xd0
	read8
	push1 0xd8
	read8
	neg
	push1 0xe0
	read8
	and
	xor
	write8
	push1 0xb0
	push1 0xd8
	read8
	push1 0xe0
	read8
	neg
	push1 0xe8
	read8
	and
	xor
	write8
	push1 0xb8
	push1 0xe0
	read8
	push1 0xe8
	read8
	neg
	push1 0xc8
	read8
	and
	xor
	write8
	push1 0xc0
	push1 0xe8
	read8
	push1 0xc8
	read8
	neg
	push1 0xd0
	read8
	and
	xor
	write8
	push1 0x05
	push1 0x05
	push0
	push0
	read8
	push8 0x0880008000000080
	xor
	write8
	push1 0x18
	push1 0x0c
	dup
	push1 0x88
	add
	push1 0x0c
	swap
	pop
:MAIN:drop::blocks::loop_exit_[blocks:2]::blocks:2
	pop
	pop
	pop
	pop
	pop
	pop
	pop
	pop
	pop
	pop
	pop
	pop
	pushl @MAIN:loop_start_blocks:2
	jump
:MAIN:loop_end_blocks:2
	push0
	push1 0x20
	exit

