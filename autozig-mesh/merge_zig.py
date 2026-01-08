#!/usr/bin/env python3
import os
import re

# 创建输出文件头部
output = """const std = @import("std");

// ============================================================================
// This file contains all mesh module code merged inline
// to avoid import issues with autozig-build system
// ============================================================================

"""

# 定义要合并的文件列表（按依赖顺序）
files_to_merge = [
    ('zig/vertex.zig', 'Vertex Data Structures'),
    ('zig/mesh.zig', 'Mesh Core'),
    ('zig/primitives.zig', 'Primitive Generators'),
    ('zig/gpu_mesh.zig', 'GPU Mesh'),
    ('zig/vertex_layout.zig', 'Vertex Layout'),
    ('zig/mesh_utils.zig', 'Mesh Utils')
]

# 要过滤掉的import模式
import_patterns = [
    r'const\s+std\s*=\s*@import\s*\(\s*"std"\s*\)\s*;',
    r'const\s+vertex\s*=\s*@import\s*\(\s*"vertex\.zig"\s*\)\s*;',
    r'const\s+mesh\s*=\s*@import\s*\(\s*"mesh\.zig"\s*\)\s*;',
    r'const\s+vertex_mod\s*=\s*@import\s*\(\s*"vertex\.zig"\s*\)\s*;',
    r'const\s+mesh_mod\s*=\s*@import\s*\(\s*"mesh\.zig"\s*\)\s*;',
    r'const\s+Vertex\s*=\s*vertex[_a-zA-Z]*\.Vertex\s*;',
    r'const\s+Mesh\s*=\s*mesh[_a-zA-Z]*\.Mesh\s*;',
]

for filepath, title in files_to_merge:
    if not os.path.exists(filepath):
        print(f"Warning: {filepath} not found, skipping")
        continue
    
    output += f"\n// ============================================================================\n"
    output += f"// {title} (from {filepath})\n"
    output += f"// ============================================================================\n\n"
    
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
        lines = content.split('\n')
        filtered_lines = []
        
        for line in lines:
            # 检查是否匹配任何import模式
            should_skip = False
            for pattern in import_patterns:
                if re.match(pattern, line.strip()):
                    should_skip = True
                    break
            
            if not should_skip:
                filtered_lines.append(line)
        
        output += '\n'.join(filtered_lines)
        output += '\n'

# 写入合并后的文件
with open('zig/mesh_all.zig', 'w', encoding='utf-8') as f:
    f.write(output)

print(f"✓ Successfully merged {len(files_to_merge)} files")
print(f"✓ Output file size: {len(output)} bytes")
print(f"✓ Total lines: {len(output.split(chr(10)))}")