#!/bin/bash

# AutoZig vs Bevy Prelude API 完成度统计
# 对比 AutoZig 公开类型数量 vs Bevy prelude 导出类型数量

AUTOZIG_DIR="/home/sonygod/projects/autozig/autozig_bevy"
BEVY_DIR="/home/sonygod/projects/autozig/bevy/crates"
OUTPUT_FILE="$AUTOZIG_DIR/docs/API_PRELUDE_COMPARISON.md"

MODULES="app ecs math render transform mesh pbr light color input window time state asset sprite ui camera"

echo "# AutoZig vs Bevy Prelude API 完成度统计" > "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "> 对比 AutoZig 公开类型数量 vs Bevy prelude 导出类型数量" >> "$OUTPUT_FILE"
echo "> 自动生成于: $(date '+%Y-%m-%d %H:%M:%S')" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

total_autozig=0
total_bevy=0

echo "## 模块完成度汇总" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "| 模块 | AutoZig 类型 | Bevy Prelude | 完成度 | 状态 |" >> "$OUTPUT_FILE"
echo "|------|-------------|--------------|--------|------|" >> "$OUTPUT_FILE"

# 收集数据用于排序
declare -a results=()

for module in $MODULES; do
    autozig_src="$AUTOZIG_DIR/autozig-$module/src"
    bevy_src="$BEVY_DIR/bevy_$module/src"
    
    if [[ ! -d "$autozig_src" ]] || [[ ! -d "$bevy_src" ]]; then
        continue
    fi
    
    # AutoZig 公开类型数量 (递归搜索所有 .rs 文件)
    autozig_types=$(find "$autozig_src" -name "*.rs" -exec grep -E "^pub (struct|enum|trait) [A-Z]" {} \; 2>/dev/null | wc -l)
    
    # Bevy prelude 只提取 use crate:: 行中的类型名
    bevy_prelude=$(grep -A 20 "pub mod prelude" "$bevy_src/lib.rs" 2>/dev/null | grep "use crate" | grep -oE "[A-Z][A-Za-z0-9_]+" | sort -u | wc -l)
    
    if [[ $bevy_prelude -gt 0 ]]; then
        pct=$((autozig_types * 100 / bevy_prelude))
    else
        pct=0
    fi
    
    # 状态标记
    if [[ $pct -ge 100 ]]; then
        status="✅ 超标"
    elif [[ $pct -ge 50 ]]; then
        status="✅ 良好"
    elif [[ $pct -ge 25 ]]; then
        status="⚠️ 基础"
    else
        status="🔴 不足"
    fi
    
    total_autozig=$((total_autozig + autozig_types))
    total_bevy=$((total_bevy + bevy_prelude))
    
    results+=("$pct|$module|$autozig_types|$bevy_prelude|$status")
done

# 按完成度排序输出
IFS=$'\n' sorted=($(sort -t'|' -k1 -rn <<<"${results[*]}"))
unset IFS

for result in "${sorted[@]}"; do
    IFS='|' read -r pct module autozig bevy status <<< "$result"
    echo "| $module | $autozig | $bevy | **${pct}%** | $status |" >> "$OUTPUT_FILE"
done

# 总计
if [[ $total_bevy -gt 0 ]]; then
    total_pct=$((total_autozig * 100 / total_bevy))
else
    total_pct=0
fi

echo "| **总计** | **$total_autozig** | **$total_bevy** | **${total_pct}%** | |" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# 分类统计
echo "---" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "## 完成度分级" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

echo "### ✅ 良好 (≥50%)" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
for result in "${sorted[@]}"; do
    IFS='|' read -r pct module autozig bevy status <<< "$result"
    if [[ $pct -ge 50 ]]; then
        echo "- **$module**: ${pct}%" >> "$OUTPUT_FILE"
    fi
done
echo "" >> "$OUTPUT_FILE"

echo "### ⚠️ 基础 (25-49%)" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
for result in "${sorted[@]}"; do
    IFS='|' read -r pct module autozig bevy status <<< "$result"
    if [[ $pct -ge 25 ]] && [[ $pct -lt 50 ]]; then
        echo "- **$module**: ${pct}%" >> "$OUTPUT_FILE"
    fi
done
echo "" >> "$OUTPUT_FILE"

echo "### 🔴 不足 (<25%)" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
for result in "${sorted[@]}"; do
    IFS='|' read -r pct module autozig bevy status <<< "$result"
    if [[ $pct -lt 25 ]]; then
        echo "- **$module**: ${pct}%" >> "$OUTPUT_FILE"
    fi
done
echo "" >> "$OUTPUT_FILE"

# 详细对比
echo "---" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "## 详细类型对比" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

for module in $MODULES; do
    autozig_src="$AUTOZIG_DIR/autozig-$module/src"
    bevy_src="$BEVY_DIR/bevy_$module/src"
    
    if [[ ! -d "$autozig_src" ]] || [[ ! -d "$bevy_src" ]]; then
        continue
    fi
    
    echo "### $module" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
    
    # 获取 Bevy prelude 类型
    bevy_types=$(grep -A 100 "pub mod prelude" "$bevy_src/lib.rs" 2>/dev/null | grep -oE "[A-Z][A-Za-z0-9_]+" | sort -u | head -30)
    
    # 获取 AutoZig 代码
    autozig_code=$(cat "$autozig_src"/*.rs 2>/dev/null)
    
    echo "| Bevy Prelude 类型 | AutoZig 状态 |" >> "$OUTPUT_FILE"
    echo "|------------------|--------------|" >> "$OUTPUT_FILE"
    
    count=0
    for typ in $bevy_types; do
        if [[ ${#typ} -lt 3 ]]; then continue; fi
        if [[ "$typ" == "Doc" ]] || [[ "$typ" == "Hidden" ]]; then continue; fi
        
        if echo "$autozig_code" | grep -qE "(struct|enum|trait|type) $typ"; then
            echo "| \`$typ\` | ✅ |" >> "$OUTPUT_FILE"
        else
            echo "| \`$typ\` | 🔴 |" >> "$OUTPUT_FILE"
        fi
        
        ((count++))
        if [[ $count -ge 20 ]]; then
            echo "| ... | (更多省略) |" >> "$OUTPUT_FILE"
            break
        fi
    done
    
    echo "" >> "$OUTPUT_FILE"
done

echo "---" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "## 结论" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "**总体完成度: ${total_pct}%**" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "- AutoZig 公开类型总数: $total_autozig" >> "$OUTPUT_FILE"
echo "- Bevy Prelude 类型总数: $total_bevy" >> "$OUTPUT_FILE"

echo ""
echo "生成完成: $OUTPUT_FILE"
wc -l "$OUTPUT_FILE"
