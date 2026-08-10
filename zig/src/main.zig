const std = @import("std");
const Op = @import("Op.zig");

const Io = std.Io;

const zig = @import("zig");

// pub const panic = std.debug.no_panic;

// pub fn panic(_: []const u8, _: ?*std.builtin.StackTrace, _: ?usize) noreturn {
//     @trap();
// }

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
    const cycles, const res = try vm.run();
    const end = Io.Clock.real.now(init.io);
    const duration = std.Io.Timestamp.durationTo(start, end);

    switch (res) {
        .done => {},
        .eof => {
            try stdout_writer.print("eof\n", .{});
        },
        .trap => |code| {
            try stdout_writer.print("trap: {}\n", .{code});
        },
        .exit => |bz| {
            try stdout_writer.print("{x}\n", .{bz});
        },
    }
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

    pub fn run(self: *Vm) !struct { u64, RunResult } {
        var cycles: u64 = 0;

        while (true) {
            @branchHint(.likely);

            const res = try self.step();

            cycles += 1;

            switch (res) {
                .stepped => {
                    @branchHint(.likely);
                },
                .eof => {
                    @branchHint(.cold);
                    return .{ cycles, .eof };
                },
                .trap => |code| {
                    @branchHint(.cold);
                    return .{ cycles, .{ .trap = code } };
                },
                .exit => |bz| {
                    @branchHint(.cold);
                    return .{ cycles, .{ .exit = bz } };
                },
            }
        }

        return .{ cycles, .done };
    }

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

    inline fn getMut(self: *Vm, n: usize) Error!*u64 {
        @setRuntimeSafety(false);

        if (self.stack.items.len < n) {
            @branchHint(.cold);
            return Error.StackEmpty;
        } else {
            return &self.stack.items.ptr[(self.stack.items.len - 1) - n];
        }
    }

    inline fn get(self: *Vm, n: usize) Error!u64 {
        @setRuntimeSafety(false);

        if (self.stack.items.len < n) {
            @branchHint(.cold);
            return Error.StackEmpty;
        } else {
            return self.stack.items.ptr[(self.stack.items.len - 1) - n];
        }
    }

    inline fn pop(self: *Vm) Error!u64 {
        @setRuntimeSafety(false);

        if (self.stack.items.len == 0) {
            @branchHint(.cold);
            return Error.StackEmpty;
        } else {
            self.stack.items.len -= 1;
            return self.stack.items.ptr[self.stack.items.len];
        }
    }

    inline fn push(self: *Vm, value: u64) !void {
        @setRuntimeSafety(false);

        if (self.stack.items.len == self.stack.capacity) {
            @branchHint(.unlikely);
            try self.stack.ensureUnusedCapacity(self.gpa, 1);
        }
        self.stack.items.len += 1;
        self.stack.items.ptr[self.stack.items.len - 1] = value;
    }

    inline fn write_n(self: *Vm, comptime n: usize) Error!void {
        @setRuntimeSafety(false);

        // if (self.stack.items.len < 2) {
        //     @branchHint(.cold);
        //     return Error.StackEmpty;
        // }
        // const value = self.stack.items.ptr[self.stack.items.len - 1];
        // const ptr = try asPtr(self.stack.items.ptr[self.stack.items.len - 2]);
        // self.stack.items.len -= 2;

        const value = try self.pop();
        const ptr = try asPtr(try self.pop());
        try checkBounds(u8, self.memory.items, ptr + n);
        var bytes: [n]u8 = undefined;
        std.mem.writeInt(@Int(.unsigned, n * 8), &bytes, @truncate(value), .big);
        @memcpy(self.memory.items.ptr[ptr..][0..n], &bytes);
    }

    inline fn read_n(self: *Vm, comptime n: usize) Error!void {
        @setRuntimeSafety(false);

        const top: *u64 = try self.getMut(0);
        const ptr = try asPtr(top.*);
        try checkBounds(u8, self.memory.items, ptr + n);
        const res = self.memory.items.ptr[ptr..][0..n];
        top.* = u64_from_bytes(n, res.*);
    }

    inline fn dread_n(self: *Vm, comptime n: usize) Error!void {
        @setRuntimeSafety(false);

        const top = try self.getMut(0);
        const ptr = try asPtr(top.*);
        try checkBounds(u8, self.data, ptr + n);
        const res = self.data.ptr[ptr..][0..n];
        top.* = u64_from_bytes(n, res.*);
    }

    inline fn push_n(self: *Vm, comptime n: usize) Error!void {
        @setRuntimeSafety(false);

        if (self.pc + n > self.code.len) {
            @branchHint(.cold);
            return Error.Eof;
        }
        try self.push(u64_from_bytes(n, self.code.ptr[self.pc..][0..n].*));
        self.pc += n;
    }

    inline fn binop(self: *Vm, comptime op: fn (u64, u64) callconv(.@"inline") u64) Error!void {
        @setRuntimeSafety(false);

        const len = self.stack.items.len;

        if (len < 2) {
            @branchHint(.cold);
            return Error.StackEmpty;
        }

        const lhs = self.stack.items.ptr[len - 2];
        const rhs = self.stack.items.ptr[len - 1];

        self.stack.items.len -= 1;
        self.stack.items.ptr[len - 2] = op(lhs, rhs);
    }

    pub fn step(self: *Vm) Error!StepResult {
        @setRuntimeSafety(false);

        if (self.pc >= self.code.len) {
            @branchHint(.cold);
            return .eof;
        }

        const op = self.code.ptr[self.pc];

        self.pc += 1;

        switch (op) {
            Op.PUSH0 => try self.push(0),
            Op.PUSH1 => try self.push_n(1),
            Op.PUSH2 => try self.push_n(2),
            Op.PUSH3 => try self.push_n(3),
            Op.PUSH4 => try self.push_n(4),
            Op.PUSH5 => try self.push_n(5),
            Op.PUSH6 => try self.push_n(6),
            Op.PUSH7 => try self.push_n(7),
            Op.PUSH8 => try self.push_n(8),
            Op.DUP => {
                const idx = try self.getMut(0);
                const stack_idx = try tryAdd(try asPtr(idx.*), 1);

                idx.* = (try self.getMut(stack_idx)).*;
            },
            Op.DUP0 => try self.push(try self.get(0)),
            Op.SWAP => {
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
                if (self.stack.items.len < 2) {
                    @branchHint(.cold);
                    return Error.InvalidStackIdx;
                }
                std.mem.swap(u64, &self.stack.items.ptr[self.stack.items.len - 2], &self.stack.items.ptr[self.stack.items.len - 1]);
            },
            Op.POP => {
                if (self.stack.items.len == 0) {
                    @branchHint(.cold);
                    return Error.StackEmpty;
                } else {
                    self.stack.items.len -= 1;
                }
            },
            Op.ALLOC => {
                const size = try self.pop();
                try self.memory.appendNTimes(self.gpa, 0, size);
            },

            Op.WRITE1 => try self.write_n(1),
            Op.WRITE2 => try self.write_n(2),
            Op.WRITE3 => try self.write_n(3),
            Op.WRITE4 => try self.write_n(4),
            Op.WRITE5 => try self.write_n(5),
            Op.WRITE6 => try self.write_n(6),
            Op.WRITE7 => try self.write_n(7),
            Op.WRITE8 => try self.write_n(8),

            Op.READ1 => try self.read_n(1),
            Op.READ2 => try self.read_n(2),
            Op.READ3 => try self.read_n(3),
            Op.READ4 => try self.read_n(4),
            Op.READ5 => try self.read_n(5),
            Op.READ6 => try self.read_n(6),
            Op.READ7 => try self.read_n(7),
            Op.READ8 => try self.read_n(8),

            Op.DREAD1 => try self.dread_n(1),
            Op.DREAD2 => try self.dread_n(2),
            Op.DREAD3 => try self.dread_n(3),
            Op.DREAD4 => try self.dread_n(4),
            Op.DREAD5 => try self.dread_n(5),
            Op.DREAD6 => try self.dread_n(6),
            Op.DREAD7 => try self.dread_n(7),
            Op.DREAD8 => try self.dread_n(8),

            Op.DCOPY => {
                if (self.stack.items.len < 3) {
                    @branchHint(.cold);
                    return Error.StackEmpty;
                }

                const len = try asPtr(self.stack.items.ptr[self.stack.items.len - 1]);
                const dst = try asPtr(self.stack.items.ptr[self.stack.items.len - 2]);
                const src = try asPtr(self.stack.items.ptr[self.stack.items.len - 3]);

                try checkBounds(u8, self.data, src + len);
                try checkBounds(u8, self.memory.items, dst + len);

                self.stack.items.len -= 3;

                @memcpy(self.memory.items.ptr[dst..(dst + len)], self.data.ptr[src..(src + len)]);
            },

            Op.DLEN => try self.push(@intCast(self.data.len)),

            Op.ADD => try self.binop(Op.add),
            Op.SUB => try self.binop(Op.sub),
            Op.MUL => try self.binop(Op.mul),
            Op.DIV => {
                const len = self.stack.items.len;

                if (len < 2) {
                    @branchHint(.cold);
                    return Error.StackEmpty;
                }

                const a = self.pop() catch {
                    unreachable;
                };

                self.stack.items.ptr[len - 2] = try Op.div(self.stack.items.ptr[len - 2], a);
            },
            Op.EXP => try self.binop(Op.expmod),
            Op.MOD => {
                const len = self.stack.items.len;

                if (len < 2) {
                    @branchHint(.cold);
                    return Error.StackEmpty;
                }

                const a = self.pop() catch {
                    unreachable;
                };

                self.stack.items.ptr[len - 2] = try Op.mod(self.stack.items.ptr[len - 2], a);
            },
            Op.EQ => try self.binop(Op.eq),
            Op.NEQ => try self.binop(Op.neq),
            Op.LT => try self.binop(Op.lt),
            Op.GT => try self.binop(Op.gt),
            Op.NOT => {
                const a = try self.getMut(0);
                a.* = Op.not(a.*);
            },
            Op.SHR => try self.binop(Op.shr),
            Op.SHL => try self.binop(Op.shl),
            Op.NEG => {
                const a = try self.getMut(0);
                a.* = Op.neg(a.*);
            },
            Op.OR => try self.binop(Op.or_),
            Op.XOR => try self.binop(Op.xor),
            Op.AND => try self.binop(Op.and_),

            Op.JUMP => {
                const dst = try self.pop();
                self.pc = try asPtr(dst);
            },
            Op.JNZ => {
                if (self.stack.items.len < 2) {
                    @branchHint(.cold);
                    return Error.StackEmpty;
                }

                const dst = self.stack.items.ptr[self.stack.items.len - 1];
                const value = self.stack.items.ptr[self.stack.items.len - 2];

                self.stack.items.len -= 2;

                if (value != 0) {
                    @branchHint(.unpredictable);
                    self.pc = try asPtr(dst);
                }
            },
            Op.CALL => {
                const top = try self.getMut(0);
                const address = try asPtr(top.*);
                top.* = @intCast(self.pc);
                self.pc = address;
            },
            Op.EXIT => {
                @branchHint(.unlikely);
                const len = try self.pop();
                const ptr = try self.pop();

                try checkBounds(u8, self.memory.items, ptr + len);

                return StepResult{ .exit = self.memory.items.ptr[ptr..][0..len] };
            },
            Op.TRAP => {
                @branchHint(.unlikely);
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

inline fn u64_from_bytes(comptime n: usize, arr: [n]u8) u64 {
    return @intCast(std.mem.readInt(@Int(.unsigned, n * 8), &arr, .big));
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

const RunResultTag = enum {
    done,
    eof,
    trap,
    exit,
};
const RunResult = union(RunResultTag) {
    done: void,
    eof: void,
    trap: u64,
    exit: []const u8,
};

inline fn asPtr(val: u64) !usize {
    @setRuntimeSafety(false);

    if (val > std.math.maxInt(usize)) {
        @branchHint(.cold);
        return Error.InvalidStackValue;
    } else {
        return @intCast(val);
    }
}

inline fn tryAdd(val: usize, n: usize) !usize {
    @setRuntimeSafety(false);

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
    @setRuntimeSafety(false);

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
