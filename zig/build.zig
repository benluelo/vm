const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{ .preferred_optimize_mode = .ReleaseFast });
    const mod = b.addModule("root", .{
        .root_source_file = b.path("src/root.zig"),
        .optimize = optimize,
        .single_threaded = true,
        .target = target,
    });

    const lib = b.addLibrary(.{
        .linkage = .static,
        .name = "vm_zig",
        .root_module = mod,
    });

    // const exe = b.addExecutable(.{
    //     .name = "vm",
    //     .root_module = b.createModule(.{
    //         // .unwind_tables = .none,
    //         .root_source_file = b.path("src/main.zig"),
    //         .target = target,
    //         .optimize = optimize,
    //         .single_threaded = true,
    //         .imports = &.{
    //             .{ .name = "zig", .module = mod },
    //         },
    //     }),
    // });

    lib.lto = .full;
    // exe.use_llvm = false;
    // exe.use_lld = false;
    lib.linkage = .static;
    lib.root_module.error_tracing = false;

    // const exe_check = b.addExecutable(.{
    //     .name = "foo",
    //     .root_module = b.createModule(.{
    //         .root_source_file = b.path("src/main.zig"),
    //         .target = target,
    //         .optimize = optimize,
    //         .imports = &.{
    //             .{ .name = "zig", .module = mod },
    //         },
    //     }),
    // });

    // const check = b.step("check", "Check if foo compiles");
    // check.dependOn(&exe_check.step);

    b.installArtifact(lib);

    // exe.getEmittedImplib();

    // const installAssembly = b.addInstallBinFile(exe.getEmittedAsm(), "assembly.s");
    // b.getInstallStep().dependOn(&installAssembly.step);

    // const installLlvmIr = b.addInstallBinFile(exe.getEmittedLlvmIr(), "llvm.ir");
    // b.getInstallStep().dependOn(&installLlvmIr.step);

    const lib_tests = b.addTest(.{
        .root_module = lib.root_module,
    });

    const run_lib_tests = b.addRunArtifact(lib_tests);

    const test_step = b.step("test", "Run tests");
    test_step.dependOn(&run_lib_tests.step);
}
