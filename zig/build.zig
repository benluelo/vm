const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{ .preferred_optimize_mode = .ReleaseFast });
    const mod = b.addModule("zig", .{
        .root_source_file = b.path("src/root.zig"),
        .single_threaded = true,
        .target = target,
    });

    const exe = b.addExecutable(.{
        .name = "vm",
        .root_module = b.createModule(.{
            // .unwind_tables = .none,
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = optimize,
            .single_threaded = true,
            .imports = &.{
                .{ .name = "zig", .module = mod },
            },
        }),
    });

    exe.lto = .full;
    // exe.use_llvm = false;
    // exe.use_lld = false;
    exe.linkage = .static;
    exe.root_module.error_tracing = false;

    const exe_check = b.addExecutable(.{
        .name = "foo",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = optimize,
            .imports = &.{
                .{ .name = "zig", .module = mod },
            },
        }),
    });

    const check = b.step("check", "Check if foo compiles");
    check.dependOn(&exe_check.step);

    b.installArtifact(exe);

    // const installAssembly = b.addInstallBinFile(exe.getEmittedAsm(), "assembly.s");
    // b.getInstallStep().dependOn(&installAssembly.step);

    // const installLlvmIr = b.addInstallBinFile(exe.getEmittedLlvmIr(), "llvm.ir");
    // b.getInstallStep().dependOn(&installLlvmIr.step);

    const run_step = b.step("run", "Run the app");

    const run_cmd = b.addRunArtifact(exe);
    run_step.dependOn(&run_cmd.step);

    run_cmd.step.dependOn(b.getInstallStep());

    run_cmd.addPassthruArgs();

    const mod_tests = b.addTest(.{
        .root_module = mod,
    });

    const run_mod_tests = b.addRunArtifact(mod_tests);

    const exe_tests = b.addTest(.{
        .root_module = exe.root_module,
    });

    const run_exe_tests = b.addRunArtifact(exe_tests);

    const test_step = b.step("test", "Run tests");
    test_step.dependOn(&run_mod_tests.step);
    test_step.dependOn(&run_exe_tests.step);
}
