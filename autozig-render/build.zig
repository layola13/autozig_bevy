const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // 构建 Zig 渲染核心库
    const lib = b.addSharedLibrary(.{
        .name = "autozig_render_core",
        .root_source_file = .{ .path = "src/zig/render.zig" },
        .target = target,
        .optimize = optimize,
    });

    // 安装库
    b.installArtifact(lib);

    // 创建测试
    const tests = b.addTest(.{
        .root_source_file = .{ .path = "src/zig/render.zig" },
        .target = target,
        .optimize = optimize,
    });

    const run_tests = b.addRunArtifact(tests);
    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&run_tests.step);

    // 创建示例
    const example = b.addExecutable(.{
        .name = "render_example",
        .root_source_file = .{ .path = "examples/basic.zig" },
        .target = target,
        .optimize = optimize,
    });
    example.linkLibrary(lib);

    b.installArtifact(example);

    const run_example = b.addRunArtifact(example);
    const example_step = b.step("example", "Run example");
    example_step.dependOn(&run_example.step);
}
