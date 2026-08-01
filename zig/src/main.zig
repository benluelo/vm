const std = @import("std");
const Io = std.Io;

const zig = @import("zig");

pub fn main(init: std.process.Init) !void {
    // This is appropriate for anything that lives as long as the process.
    const arena: std.mem.Allocator = std.heap.brk_allocator;

    // In order to do I/O operations need an `Io` instance.
    const io = init.io;

    // Accessing command line arguments:
    const args = try init.minimal.args.toSlice(arena);
    const file_path = args[1];
    const file = try std.Io.Dir.cwd().readFileAlloc(io, file_path, arena, Io.Limit.unlimited);
    defer arena.free(file);
    const input_file = args[2];
    const input = try std.Io.Dir.cwd().readFileAlloc(io, input_file, arena, Io.Limit.unlimited);
    defer arena.free(input);
    // const is_hex = args.len == 4 and std.mem.eql(u8, args[3], "--hex");
    // std.log.info("file_path: {s}, file: {s}, input: {s}, is_hex: {}", .{ file_path, file, input, is_hex });

    var vm = Vm.init(arena, file, input);

    // Stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_file_writer: Io.File.Writer = .init(.stdout(), io, &stdout_buffer);
    const stdout_writer = &stdout_file_writer.interface;

    const start = Io.Clock.real.now(init.io);

    var cycles: u64 = 0;

    while (true) {
        const res = try vm.step();

        cycles += 1;

        switch (res) {
            .stepped => {
                @branchHint(.likely);
                // try stdout_writer.print("stepped\n", .{});
                // try stdout_writer.flush();
                // continue;
            },
            .eof => {
                @branchHint(.cold);
                try stdout_writer.print("eof\n", .{});
                break;
            },
            .trap => |code| {
                @branchHint(.cold);
                try stdout_writer.print("trap: {}\n", .{code});
                break;
            },
            .exit => |bz| {
                @branchHint(.cold);
                try stdout_writer.print("{x}\n", .{bz});
                break;
            },
        }
    }

    const end = Io.Clock.real.now(init.io);
    const duration = std.Io.Timestamp.durationTo(start, end);

    try stdout_writer.print("time: {}ms\n", .{duration.toMilliseconds()});
    try stdout_writer.print("total cycles: {}", .{cycles});

    try stdout_writer.flush(); // Don't forget to flush!
}

pub const Vm = struct {
    gpa: std.mem.Allocator,
    code: []u8,
    data: []const u8,
    stack: std.ArrayList(u64),
    memory: std.ArrayList(u8),
    pc: usize,

    pub fn init(gpa: std.mem.Allocator, code: []u8, data: []const u8) Vm {
        return Vm{
            .gpa = gpa,
            .code = code,
            .data = data,
            .stack = .empty,
            .memory = .empty,
            .pc = 0,
        };
    }

    inline fn getMut(self: Vm, n: usize) Error!*u64 {
        if (self.stack.items.len < n) {
            @branchHint(.cold);
            return Error.StackEmpty;
        } else {
            return &self.stack.items.ptr[(self.stack.items.len - 1) - n];
        }
    }

    inline fn pop(self: *Vm) Error!u64 {
        if (self.stack.items.len == 0) {
            @branchHint(.cold);
            return Error.StackEmpty;
        } else {
            self.stack.items.len -= 1;
            return self.stack.items.ptr[self.stack.items.len];
        }
    }

    inline fn push(self: *Vm, value: u64) !void {
        if (self.stack.items.len == self.stack.capacity) {
            // @branchHint(.unlikely);
            try self.stack.ensureUnusedCapacity(self.gpa, 1);
        }
        self.stack.items.len += 1;
        self.stack.items.ptr[self.stack.items.len - 1] = value;

        // self.stack.appendAssumeCapacity(value);
        // try self.stack.append(self.gpa, value);
    }

    inline fn write_n(self: *Vm, comptime n: u4) Error!void {
        const value = try self.pop();
        const ptr = try asPtr(try self.pop());
        var bytes: [8]u8 = undefined;
        std.mem.writeInt(u64, &bytes, value, .big);
        try checkBounds(u8, self.memory.items, ptr + n);
        @memcpy(self.memory.items.ptr[ptr..][0..n], bytes[(8 - n)..]);
    }

    inline fn read_n(self: *Vm, comptime n: u4) Error!void {
        const top: *u64 = try self.getMut(0);
        const ptr = try asPtr(top.*);
        try checkBounds(u8, self.memory.items, ptr + n);
        const res = self.memory.items.ptr[ptr..][0..n];
        top.* = u64_from_bytes(res);
    }

    inline fn dread_n(self: *Vm, comptime n: u4) Error!void {
        const top = try self.getMut(0);
        const ptr = try asPtr(top.*);
        try checkBounds(u8, self.data, ptr + n);
        const res = self.data.ptr[ptr..][0..n];
        top.* = u64_from_bytes(res);
    }

    inline fn binop(self: *Vm, comptime op: fn (u64, u64) callconv(.@"inline") u64) Error!void {
        const len = self.stack.items.len;

        if (len < 2) {
            @branchHint(.cold);
            return Error.StackEmpty;
        }

        const lhs = self.stack.items.ptr[len - 2];
        const rhs = self.stack.items.ptr[len - 1];

        self.stack.items.len = len - 1;
        self.stack.items.ptr[len - 2] = op(lhs, rhs);
    }

    pub fn step(self: *Vm) Error!StepResult {
        if (self.pc >= self.code.len) {
            @branchHint(.cold);
            return .eof;
        }

        //     // std.log.info("pc: {}", .{self.pc});

        const op = self.code.ptr[self.pc];

        self.pc += 1;

        //     // std.log.info("op: {x}", .{op});
        //     std.log.info("stack: {any}", .{self.stack.items});

        switch (op) {
            Op.PUSH0 => {
                //             std.log.info("PUSH0", .{});
                try self.push(0);
            },
            Op.PUSH1 => {
                //             std.log.info("PUSH1", .{});
                if (self.pc + 1 > self.code.len) {
                    @branchHint(.cold);
                    return Error.Eof;
                }
                try self.push(u64_from_bytes(self.code.ptr[self.pc..][0..1]));
                self.pc += 1;
            },
            Op.PUSH2 => {
                //             std.log.info("PUSH2", .{});
                if (self.pc + 2 > self.code.len) return Error.Eof;
                try self.push(u64_from_bytes(self.code.ptr[self.pc..][0..2]));
                self.pc += 2;
            },
            Op.PUSH3 => {
                //             std.log.info("PUSH3", .{});
                if (self.pc + 3 > self.code.len) return Error.Eof;
                try self.push(u64_from_bytes(self.code.ptr[self.pc..][0..3]));
                self.pc += 3;
            },
            Op.PUSH4 => {
                //             std.log.info("PUSH4", .{});
                if (self.pc + 4 > self.code.len) return Error.Eof;
                try self.push(u64_from_bytes(self.code.ptr[self.pc..][0..4]));
                self.pc += 4;
            },
            Op.PUSH5 => {
                //             std.log.info("PUSH5", .{});
                if (self.pc + 5 > self.code.len) return Error.Eof;
                try self.push(u64_from_bytes(self.code.ptr[self.pc..][0..5]));
                self.pc += 5;
            },
            Op.PUSH6 => {
                //             std.log.info("PUSH6", .{});
                if (self.pc + 6 > self.code.len) return Error.Eof;
                try self.push(u64_from_bytes(self.code.ptr[self.pc..][0..6]));
                self.pc += 6;
            },
            Op.PUSH7 => {
                //             std.log.info("PUSH7", .{});
                if (self.pc + 7 > self.code.len) return Error.Eof;
                try self.push(u64_from_bytes(self.code.ptr[self.pc..][0..7]));
                self.pc += 7;
            },
            Op.PUSH8 => {
                //             std.log.info("PUSH8", .{});
                if (self.pc + 8 > self.code.len) return Error.Eof;
                try self.push(u64_from_bytes(self.code.ptr[self.pc..][0..8]));
                self.pc += 8;
            },
            Op.DUP => {
                //             std.log.info("DUP", .{});
                const idx = try self.getMut(0);
                const stack_idx = try tryAdd(try asPtr(idx.*), 1);

                idx.* = (try self.getMut(stack_idx)).*;
            },
            Op.DUP0 => {
                //             std.log.info("DUP0", .{});

                try self.push((try self.getMut(0)).*);
            },
            Op.SWAP => {
                //             std.log.info("SWAP", .{});
                const idx = try tryAdd(try asPtr(try self.pop()), 1);
                const len = self.stack.items.len;
                if (len < idx) {
                    @branchHint(.cold);
                    return Error.InvalidStackIdx;
                }
                const a_idx = len - 1;
                const b_idx = a_idx - idx;
                std.mem.swap(u64, &self.stack.items.ptr[a_idx], &self.stack.items.ptr[b_idx]);
            },
            Op.SWAP0 => {
                //             std.log.info("SWAP0", .{});
                const b_idx = trySub(self.stack.items.len, 2) orelse return Error.InvalidStackIdx;
                const a_idx = self.stack.items.len - 1;
                std.mem.swap(u64, &self.stack.items.ptr[a_idx], &self.stack.items.ptr[b_idx]);
            },
            Op.POP => {
                //             std.log.info("POP", .{});
                _ = try self.pop();
            },
            Op.ALLOC => {
                //             std.log.info("ALLOC", .{});
                const size = try self.pop();
                try self.memory.appendNTimes(self.gpa, 0, size);
            },

            Op.WRITE1 => {
                //             std.log.info("WRITE1", .{});
                try self.write_n(1);
            },
            Op.WRITE2 => {
                //             std.log.info("WRITE2", .{});
                try self.write_n(2);
            },
            Op.WRITE3 => {
                //             std.log.info("WRITE3", .{});
                try self.write_n(3);
            },
            Op.WRITE4 => {
                //             std.log.info("WRITE4", .{});
                try self.write_n(4);
            },
            Op.WRITE5 => {
                //             std.log.info("WRITE5", .{});
                try self.write_n(5);
            },
            Op.WRITE6 => {
                //             std.log.info("WRITE6", .{});
                try self.write_n(6);
            },
            Op.WRITE7 => {
                //             std.log.info("WRITE7", .{});
                try self.write_n(7);
            },
            Op.WRITE8 => {
                //             std.log.info("WRITE8", .{});
                try self.write_n(8);
            },

            Op.READ1 => {
                //             std.log.info("READ1", .{});
                try self.read_n(1);
            },
            Op.READ2 => {
                //             std.log.info("READ2", .{});
                try self.read_n(2);
            },
            Op.READ3 => {
                //             std.log.info("READ3", .{});
                try self.read_n(3);
            },
            Op.READ4 => {
                //             std.log.info("READ4", .{});
                try self.read_n(4);
            },
            Op.READ5 => {
                //             std.log.info("READ5", .{});
                try self.read_n(5);
            },
            Op.READ6 => {
                //             std.log.info("READ6", .{});
                try self.read_n(6);
            },
            Op.READ7 => {
                //             std.log.info("READ7", .{});
                try self.read_n(7);
            },
            Op.READ8 => {
                //             std.log.info("READ8", .{});
                try self.read_n(8);
            },

            Op.DREAD1 => {
                //             std.log.info("DREAD1", .{});
                try self.dread_n(1);
            },
            Op.DREAD2 => {
                //             std.log.info("DREAD2", .{});
                try self.dread_n(2);
            },
            Op.DREAD3 => {
                //             std.log.info("DREAD3", .{});
                try self.dread_n(3);
            },
            Op.DREAD4 => {
                //             std.log.info("DREAD4", .{});
                try self.dread_n(4);
            },
            Op.DREAD5 => {
                //             std.log.info("DREAD5", .{});
                try self.dread_n(5);
            },
            Op.DREAD6 => {
                //             std.log.info("DREAD6", .{});
                try self.dread_n(6);
            },
            Op.DREAD7 => {
                //             std.log.info("DREAD7", .{});
                try self.dread_n(7);
            },
            Op.DREAD8 => {
                //             std.log.info("DREAD8", .{});
                try self.dread_n(8);
            },

            Op.DCOPY => {
                //             std.log.info("DCOPY", .{});
                if (self.stack.items.len < 3) {
                    return Error.StackEmpty;
                }

                // const srcPtr, const dstPtr, const lenPtr = self.stack.items[(self.stack.items.len - 3)..];

                const len = try asPtr(self.stack.items.ptr[self.stack.items.len - 1]);
                const dst = try asPtr(self.stack.items.ptr[self.stack.items.len - 2]);
                const src = try asPtr(self.stack.items.ptr[self.stack.items.len - 3]);

                try checkBounds(u8, self.data, src + len);
                try checkBounds(u8, self.memory.items, dst + len);

                self.stack.items.len = self.stack.items.len - 3;

                @memcpy(self.memory.items.ptr[dst..(dst + len)], self.data.ptr[src..(src + len)]);
            },

            Op.DLEN => {
                //             std.log.info("DLEN", .{});
                try self.push(@intCast(self.data.len));
            },

            Op.ADD => {
                //             std.log.info("ADD", .{});
                try self.binop(Op.add);
            },
            Op.SUB => {
                //             std.log.info("SUB", .{});
                try self.binop(Op.sub);
            },
            Op.MUL => {
                //             std.log.info("MUL", .{});
                try self.binop(Op.mul);
            },
            Op.DIV => {
                //             std.log.info("DIV", .{});
                const len = self.stack.items.len;

                if (len < 2) {
                    return Error.StackEmpty;
                }

                const a = self.pop() catch {
                    unreachable;
                };

                self.stack.items.ptr[len - 2] = try Op.div(self.stack.items.ptr[len - 2], a);
                //             std.log.info("div", .{});
            },
            Op.EXP => {
                //             std.log.info("EXP", .{});
                try self.binop(Op.expmod);
            },
            Op.MOD => {
                //             std.log.info("MOD", .{});
                const a = try self.pop();
                const b = try self.getMut(0);
                b.* = try Op.mod(b.*, a);
                //             std.log.info("mod", .{});
            },
            Op.EQ => {
                //             std.log.info("EQ", .{});
                try self.binop(Op.eq);
            },
            Op.NEQ => {
                //             std.log.info("NEQ", .{});
                try self.binop(Op.neq);
            },
            Op.LT => {
                //             std.log.info("LT", .{});
                try self.binop(Op.lt);
            },
            Op.GT => {
                //             std.log.info("GT", .{});
                try self.binop(Op.gt);
            },
            Op.NOT => {
                //             std.log.info("NOT", .{});
                const a = try self.getMut(0);
                a.* = Op.not(a.*);
                //             std.log.info("not", .{});
            },
            Op.SHR => {
                //             std.log.info("SHR", .{});
                try self.binop(Op.shr);
            },
            Op.SHL => {
                //             std.log.info("SHL", .{});
                try self.binop(Op.shl);
            },
            Op.NEG => {
                //             std.log.info("NEG", .{});
                const a = try self.getMut(0);
                a.* = Op.neg(a.*);
            },
            Op.OR => {
                //             std.log.info("OR", .{});
                try self.binop(Op.or_);
            },
            Op.XOR => {
                //             std.log.info("XOR", .{});
                try self.binop(Op.xor);
            },
            Op.AND => {
                //             std.log.info("AND", .{});
                try self.binop(Op.and_);
            },

            Op.JUMP => {
                //             std.log.info("JUMP", .{});
                const dst = try self.pop();
                //             // std.log.info("dst: {}", .{dst});
                self.pc = try asPtr(dst);
            },
            Op.JNZ => {
                //             std.log.info("JNZ", .{});
                const dst = try self.pop();
                const value = try self.pop();
                if (value != 0) {
                    self.pc = try asPtr(dst);
                }
            },
            Op.CALL => {
                //             std.log.info("CALL", .{});
                const top = try self.getMut(0);
                const address = try asPtr(top.*);
                top.* = @intCast(self.pc);
                self.pc = address;
            },
            Op.EXIT => {
                //             std.log.info("EXIT", .{});
                const len = try self.pop();
                const ptr = try self.pop();

                try checkBounds(u8, self.memory.items, ptr + len);

                return StepResult{ .exit = self.memory.items.ptr[ptr..][0..len] };
            },
            Op.TRAP => {
                //             std.log.info("TRAP", .{});
                const value = try self.pop();
                return StepResult{ .trap = value };
            },
            else => {
                @branchHint(.cold);
                return Error.UnknownOp;
            },
        }

        return .stepped;
    }
};

pub const Op = struct {
    pub const PUSH0: u8 = 0x00;
    /// Push a single byte to to the stack.
    pub const PUSH1: u8 = 0x01;
    /// Push 2 bytes to the stack.
    pub const PUSH2: u8 = 0x02;
    /// Push 3 bytes to the stack.
    pub const PUSH3: u8 = 0x03;
    /// Push 4 bytes to the stack.
    pub const PUSH4: u8 = 0x04;
    /// Push 5 bytes to the stack.
    pub const PUSH5: u8 = 0x05;
    /// Push 6 bytes to the stack.
    pub const PUSH6: u8 = 0x06;
    /// Push 7 bytes to the stack.
    pub const PUSH7: u8 = 0x07;
    /// Push 8 bytes (a full word) to the stack.
    pub const PUSH8: u8 = 0x08;
    /// Pop the item on the top of the stack as N and duplicate the Nth stack item.
    ///
    /// | Stack Input   | Stack Output  |
    /// | ------------- | ------------- |
    /// | `[..., a, N]` | `[..., a, a]` |
    pub const DUP: u8 = 0x09;
    /// Duplicate the item on the top of the stack.
    ///
    /// | Stack Input | Stack Output  |
    /// | ------------| ------------- |
    /// | `[..., a]`  | `[..., a, a]` |
    pub const DUP0: u8 = 0x0a;
    /// Pop the item on the top of the stack as N and swap the first and (N-1)th stack items.
    ///
    /// | Stack Input                  | Stack Output                 |
    /// | ---------------------------- | ---------------------------- |
    /// | `[..., a, (...{n-1}), b, n]` | `[..., b, (...{n-1}), a, n]` |
    pub const SWAP: u8 = 0x0b;
    /// Swap the top two items on the stack.
    ///
    /// | Stack Input   | Stack Output  |
    /// | ------------- | ------------- |
    /// | `[..., a, b]` | `[..., b, a]` |
    pub const SWAP0: u8 = 0x0c;
    /// Pop the top of the stack, returning an error if the stack is empty.
    ///
    /// | Stack Input | Stack Output |
    /// | ----------- | ------------ |
    /// | `[..., a]`  | `[...]`      |
    pub const POP: u8 = 0x0d;
    /// Pop the top of the stack anad grow the memory by that number of bytes.
    ///
    /// | Stack Input   | Stack Output |
    /// | ------------- | ------------ |
    /// | `[..., size]` | `[...]`      |
    pub const ALLOC: u8 = 0x20;
    pub const WRITE1: u8 = 0x21;
    pub const WRITE2: u8 = 0x22;
    pub const WRITE3: u8 = 0x23;
    pub const WRITE4: u8 = 0x24;
    pub const WRITE5: u8 = 0x25;
    pub const WRITE6: u8 = 0x26;
    pub const WRITE7: u8 = 0x27;
    /// Write the full value on the top of the stack at the memory location of the
    /// 2nd top item in the stack.
    pub const WRITE8: u8 = 0x28;
    pub const READ1: u8 = 0x29;
    pub const READ2: u8 = 0x2a;
    pub const READ3: u8 = 0x2b;
    pub const READ4: u8 = 0x2c;
    pub const READ5: u8 = 0x2d;
    pub const READ6: u8 = 0x2e;
    pub const READ7: u8 = 0x2f;
    /// Read the full value at the memory location specified by the top value on the
    /// stack to the top of the stack.
    ///
    /// Unwritten memory is read as zero.
    pub const READ8: u8 = 0x30;
    pub const DREAD1: u8 = 0x31;
    pub const DREAD2: u8 = 0x32;
    pub const DREAD3: u8 = 0x33;
    pub const DREAD4: u8 = 0x34;
    pub const DREAD5: u8 = 0x35;
    pub const DREAD6: u8 = 0x36;
    pub const DREAD7: u8 = 0x37;
    /// Read the full value at the memory location specified by the top value on the
    /// stack to the top of the stack.
    ///
    /// Data read beyond the data length is read as zero.
    pub const DREAD8: u8 = 0x38;
    /// Copy a portion of data delimited by src..src+len to memory at dst.
    ///
    /// | Stack Input            | Stack Output           |
    /// | ---------------------- | ---------------------- |
    /// | `[..., src, dst, len]` | `[...]`                |
    pub const DCOPY: u8 = 0x39;
    /// Push the length of the data to the stack.
    ///
    /// | Stack Input | Stack Output |
    /// | ----------- | ------------ |
    /// | `[...]`     | `[..., len]` |
    pub const DLEN: u8 = 0x3a;
    /// Wrapping addition.
    ///
    /// | Stack Input   | Stack Output   |
    /// | ------------- | -------------- |
    /// | `[..., b, a]` | `[..., b + a]` |
    pub const ADD: u8 = 0x40;
    /// Wrapping subtraction.
    ///
    /// | Stack Input   | Stack Output   |
    /// | ------------- | -------------- |
    /// | `[..., b, a]` | `[..., b - a]` |
    pub const SUB: u8 = 0x41;
    /// Wrapping multiplication.
    ///
    /// | Stack Input   | Stack Output   |
    /// | ------------- | -------------- |
    /// | `[..., b, a]` | `[..., b * a]` |
    pub const MUL: u8 = 0x42;
    /// Floor division.
    ///
    /// | Stack Input   | Stack Output    |
    /// | ------------- | --------------- |
    /// | `[..., b, a]` | `[..., b // a]` |
    ///
    /// This operation will return an error if the divisor is zero.
    pub const DIV: u8 = 0x43;
    /// Wrapping exponentiation.
    ///
    /// | Stack Input   | Stack Output    |
    /// | ------------- | --------------- |
    /// | `[..., b, a]` | `[..., b ** a]` |
    pub const EXP: u8 = 0x44;
    /// Modulus (remainder).
    ///
    /// | Stack Input   | Stack Output   |
    /// | ------------- | -------------- |
    /// | `[..., b, a]` | `[..., b % a]` |
    pub const MOD: u8 = 0x45;
    /// Equality.
    ///
    /// | Stack Input   | Stack Output    |
    /// | ------------- | --------------- |
    /// | `[..., b, a]` | `[..., b == a]` |
    pub const EQ: u8 = 0x4a;
    /// Inequality.
    ///
    /// | Stack Input   | Stack Output    |
    /// | ------------- | --------------- |
    /// | `[..., b, a]` | `[..., b != a]` |
    pub const NEQ: u8 = 0x4b;
    /// Less-than comparison.
    ///
    /// | Stack Input   | Stack Output   |
    /// | ------------- | -------------- |
    /// | `[..., b, a]` | `[..., b < a]` |
    pub const LT: u8 = 0x4c;
    /// Greater-than comparison.
    ///
    /// | Stack Input   | Stack Output   |
    /// | ------------- | -------------- |
    /// | `[..., b, a]` | `[..., b > a]` |
    pub const GT: u8 = 0x4d;
    /// Logical NOT.
    ///
    /// | Stack Input | Stack Output |
    /// | ----------- | ------------ |
    /// | `[..., a]`  | `[..., !a]`  |
    ///
    /// Note that this is not bitwise negation (see [`Op::NEG`]). The value is treated as a boolean, and value pushed back to the stack will only ever be 0 or 1.
    pub const NOT: u8 = 0x4e;
    /// Left shift.
    ///
    /// | Stack Input   | Stack Output    |
    /// | ------------- | --------------- |
    /// | `[..., b, a]` | `[..., a << b]` |
    ///
    /// The shift is "unbounded", and as such will always return 0 if the shift value is >= 64.
    pub const SHL: u8 = 0x4f;
    /// Right shift.
    ///
    /// | Stack Input   | Stack Output    |
    /// | ------------- | --------------- |
    /// | `[..., b, a]` | `[..., a >> b]` |
    ///
    /// The shift is "unbounded", and as such will always return 0 if the shift value is >= 64.
    pub const SHR: u8 = 0x50;
    /// Bitwise negation.
    ///
    /// | Stack Input | Stack Output |
    /// | ----------- | ------------ |
    /// | `[..., a]`  | `[..., ~a]`  |
    pub const NEG: u8 = 0x51;
    /// Bitwise OR.
    ///
    /// | Stack Input   | Stack Output   |
    /// | ------------- | -------------- |
    /// | `[..., b, a]` | `[..., b | a]` |
    pub const OR: u8 = 0x52;
    /// Bitwise XOR.
    ///
    /// | Stack Input   | Stack Output   |
    /// | ------------- | -------------- |
    /// | `[..., b, a]` | `[..., b ^ a]` |
    pub const XOR: u8 = 0x53;
    /// Bitwise AND.
    ///
    /// | Stack Input   | Stack Output   |
    /// | ------------- | -------------- |
    /// | `[..., b, a]` | `[..., b & a]` |
    pub const AND: u8 = 0x54;
    /// Pop an instruction pointer off the stack and jump to the address.
    ///
    /// | Stack Input   | Stack Output |
    /// | ------------- | ------------ |
    /// | `[..., addr]` | `[...]`      |
    pub const JUMP: u8 = 0xa0;
    /// Pop the top value off of the stack and jump to the contained address
    /// if the value is non-zero.
    ///
    /// | Stack Input         | Stack Output |
    /// | ------------------- | ------------ |
    /// | `[..., addr, cond]` | `[...]`      |
    pub const JNZ: u8 = 0xa1;
    /// Pop an instruction pointer off the stack, push the current address to the stack, and then jump to the previously popped address.
    ///
    /// | Stack Input   | Stack Output |
    /// | ------------- | ------------ |
    /// | `[..., addr]` | `[..., ret]` |
    pub const CALL: u8 = 0xa2;
    /// Terminate execution with a payload. The top two values of the stack will
    /// be read as the pointer to and length of the return data.
    ///
    /// | Stack Input       | Stack Output           |
    /// | ----------------- | ---------------------- |
    /// | `[..., ptr, len]` | `<program terminates>` |
    pub const EXIT: u8 = 0xa4;
    /// Terminate execution with an error code. The top value of the stack is
    /// used as the error code.
    ///
    /// | Stack Input   | Stack Output           |
    /// | ------------- | ---------------------- |
    /// | `[..., code]` | `<program terminates>` |
    pub const TRAP: u8 = 0xa5;

    pub inline fn add(a: u64, b: u64) u64 {
        return a +% b;
    }

    pub inline fn sub(a: u64, b: u64) u64 {
        return a -% b;
    }

    pub inline fn mul(a: u64, b: u64) u64 {
        return a *% b;
    }

    pub inline fn div(a: u64, b: u64) !u64 {
        if (b == 0) {
            @branchHint(.cold);
            return Error.DivideByZero;
        } else {
            return a / b;
        }
    }

    pub inline fn not(a: u64) u64 {
        return @intFromBool(a == 0);
    }

    pub inline fn gt(a: u64, b: u64) u64 {
        return @intFromBool(a > b);
    }

    pub inline fn lt(a: u64, b: u64) u64 {
        return @intFromBool(a < b);
    }

    pub inline fn neq(a: u64, b: u64) u64 {
        return @intFromBool(a != b);
    }

    pub inline fn eq(a: u64, b: u64) u64 {
        return @intFromBool(a == b);
    }

    pub inline fn mod(a: u64, b: u64) !u64 {
        if (b == 0) {
            @branchHint(.cold);
            return Error.DivideByZero;
        } else {
            return a % b;
        }
    }

    pub inline fn and_(a: u64, b: u64) u64 {
        return a & b;
    }

    pub inline fn xor(a: u64, b: u64) u64 {
        return a ^ b;
    }

    pub inline fn or_(a: u64, b: u64) u64 {
        return a | b;
    }

    pub inline fn neg(a: u64) u64 {
        return ~a;
    }

    pub inline fn expmod(a: u64, b: u64) u64 {
        if (b == 0) {
            return 1;
        }

        var acc: u128 = 1;
        var base = @as(u128, a);
        var exp = b;

        while (true) {
            if ((exp & 1) == 1) {
                acc = (acc * base) % std.math.maxInt(u64);
                // since exp!=0, finally the exp must be 1.
                if (exp == 1) {
                    return @intCast(acc);
                }
            }
            exp >>= 1;
            base = (base * base) % std.math.maxInt(u64);
        }
    }

    pub inline fn shr(a: u64, shift: u64) u64 {
        return std.math.shr(u64, a, shift);
    }

    pub inline fn shl(a: u64, shift: u64) u64 {
        return std.math.shl(u64, a, shift);
    }
};

inline fn u64_from_bytes(arr: []const u8) u64 {
    // // std.log.info("arr: {any}", .{arr});
    var v: [8]u8 = [_]u8{ 0, 0, 0, 0, 0, 0, 0, 0 };
    // // std.log.info("arr.len: {}", .{arr.len});
    @memcpy(v[(8 - arr.len)..], arr);
    // // std.log.info("v: {any}", .{v});
    return std.mem.readInt(u64, &v, .big);
}

const StepResultTag = enum {
    stepped,
    eof,
    trap,
    exit,
};
const StepResult = union(StepResultTag) {
    stepped: void,
    eof: void,
    trap: u64,
    exit: []const u8,
};

inline fn asPtr(val: u64) !usize {
    if (val > std.math.maxInt(usize)) {
        @branchHint(.cold);
        return Error.InvalidStackValue;
    } else {
        return @intCast(val);
    }
}

inline fn tryAdd(val: usize, n: usize) !usize {
    const res, const overflow = @addWithOverflow(val, n);

    if (overflow != 0) {
        @branchHint(.cold);
        return Error.InvalidStackValue;
    }

    return res;
}

inline fn trySub(val: usize, n: usize) ?usize {
    const res, const overflow = @subWithOverflow(val, n);

    if (overflow != 0) {
        @branchHint(.cold);
        return null;
    }

    return res;
}

inline fn checkBounds(comptime T: type, list: []const T, idx: usize) !void {
    if (list.len < idx) {
        @branchHint(.cold);
        return Error.Segfault;
    }
}

const Error = error{
    OutOfMemory,

    StackEmpty,
    InvalidStackIdx,
    Segfault,
    Eof,
    DivideByZero,
    InvalidStackValue,
    UnknownOp,
    PointerTooBig,
};
