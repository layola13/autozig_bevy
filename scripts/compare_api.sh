#!/bin/bash

# AutoZig vs Bevy API 对比生成器
# 生成 Markdown 表格，标记未实现的 API

AUTOZIG_DIR="/home/sonygod/projects/autozig/autozig_bevy"
BEVY_DIR="/home/sonygod/projects/autozig/bevy/crates"
OUTPUT_FILE="$AUTOZIG_DIR/docs/API_COMPARISON.md"

# 模块列表
MODULES="app ecs math render transform mesh pbr light color input window time state asset sprite ui text reflect tasks ptr utils diagnostic log image camera"

# 开始生成报告
echo "# AutoZig vs Bevy API 对比报告" > "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "> 自动生成于: $(date '+%Y-%m-%d %H:%M:%S')" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# 总计数
total_autozig=0
total_bevy=0
total_matched=0

echo "## 模块汇总" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "| 模块 | AutoZig | Bevy | 匹配 | 完成度 |" >> "$OUTPUT_FILE"
echo "|------|---------|------|------|--------|" >> "$OUTPUT_FILE"

for module in $MODULES; do
    bevy_crate="bevy_$module"
    autozig_src="$AUTOZIG_DIR/autozig-$module/src"
    bevy_src="$BEVY_DIR/$bevy_crate/src"
    
    if [[ ! -d "$autozig_src" ]] || [[ ! -d "$bevy_src" ]]; then
        continue
    fi
    
    # 提取 API 数量
    autozig_count=$(grep -roh "pub \(struct\|enum\|trait\) [A-Za-z_][A-Za-z0-9_]*" "$autozig_src" 2>/dev/null | sort -u | wc -l)
    bevy_count=$(grep -roh "pub \(struct\|enum\|trait\) [A-Za-z_][A-Za-z0-9_]*" "$bevy_src" 2>/dev/null | sort -u | wc -l)
    
    # 计算匹配数
    bevy_names=$(grep -roh "pub \(struct\|enum\|trait\) [A-Za-z_][A-Za-z0-9_]*" "$bevy_src" 2>/dev/null | sed 's/pub \(struct\|enum\|trait\) //' | sort -u)
    autozig_content=$(cat "$autozig_src"/*.rs 2>/dev/null)
    
    matched=0
    for item in $bevy_names; do
        if echo "$autozig_content" | grep -q "\(struct\|enum\|trait\) $item"; then
            ((matched++))
        fi
    done
    
    if [[ $bevy_count -gt 0 ]]; then
        pct=$((matched * 100 / bevy_count))
    else
        pct=0
    fi
    
    total_autozig=$((total_autozig + autozig_count))
    total_bevy=$((total_bevy + bevy_count))
    total_matched=$((total_matched + matched))
    
    echo "| $module | $autozig_count | $bevy_count | $matched | ${pct}% |" >> "$OUTPUT_FILE"
done

# 总计
if [[ $total_bevy -gt 0 ]]; then
    total_pct=$((total_matched * 100 / total_bevy))
else
    total_pct=0
fi

echo "| **总计** | **$total_autozig** | **$total_bevy** | **$total_matched** | **${total_pct}%** |" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# 详细对比
echo "---" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "## 详细 API 对比" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

for module in $MODULES; do
    bevy_crate="bevy_$module"
    autozig_src="$AUTOZIG_DIR/autozig-$module/src"
    bevy_src="$BEVY_DIR/$bevy_crate/src"
    
    if [[ ! -d "$autozig_src" ]] || [[ ! -d "$bevy_src" ]]; then
        continue
    fi
    
    echo "### $module" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
    echo "| 类型 | API 名称 | 状态 |" >> "$OUTPUT_FILE"
    echo "|------|---------|------|" >> "$OUTPUT_FILE"
    
    # 提取 Bevy API (使用 grep -r 递归)
    bevy_structs=$(grep -roh "pub struct [A-Za-z_][A-Za-z0-9_]*" "$bevy_src" 2>/dev/null | sed 's/pub struct //' | sort -u)
    bevy_enums=$(grep -roh "pub enum [A-Za-z_][A-Za-z0-9_]*" "$bevy_src" 2>/dev/null | sed 's/pub enum //' | sort -u)
    bevy_traits=$(grep -roh "pub trait [A-Za-z_][A-Za-z0-9_]*" "$bevy_src" 2>/dev/null | sed 's/pub trait //' | sort -u)
    
    # 读取 AutoZig 代码
    autozig_all=$(cat "$autozig_src"/*.rs 2>/dev/null)
    
    # Structs (限制数量，避免文件过大)
    count=0
    for item in $bevy_structs; do
        if [[ $count -ge 100 ]]; then
            echo "| struct | ... | (更多省略) |" >> "$OUTPUT_FILE"
            break
        fi
        if echo "$autozig_all" | grep -q "struct $item"; then
            echo "| struct | \`$item\` | ✅ |" >> "$OUTPUT_FILE"
        else
            echo "| struct | \`$item\` | 🔴 缺失 |" >> "$OUTPUT_FILE"
        fi
        ((count++))
    done
    
    # Enums
    count=0
    for item in $bevy_enums; do
        if [[ $count -ge 50 ]]; then
            echo "| enum | ... | (更多省略) |" >> "$OUTPUT_FILE"
            break
        fi
        if echo "$autozig_all" | grep -q "enum $item"; then
            echo "| enum | \`$item\` | ✅ |" >> "$OUTPUT_FILE"
        else
            echo "| enum | \`$item\` | 🔴 缺失 |" >> "$OUTPUT_FILE"
        fi
        ((count++))
    done
    
    # Traits
    for item in $bevy_traits; do
        if echo "$autozig_all" | grep -q "trait $item"; then
            echo "| trait | \`$item\` | ✅ |" >> "$OUTPUT_FILE"
        else
            echo "| trait | \`$item\` | 🔴 缺失 |" >> "$OUTPUT_FILE"
        fi
    done
    
    echo "" >> "$OUTPUT_FILE"
done

echo "---" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "## 图例" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "- ✅ = 已实现" >> "$OUTPUT_FILE"
echo "- 🔴 = 缺失 (需要新增)" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "*注意: 此对比仅检查类型名称是否存在，不验证方法签名是否完全一致。*" >> "$OUTPUT_FILE"
echo "*为避免文件过大，每个类别最多显示100个struct和50个enum。*" >> "$OUTPUT_FILE"

echo "生成完成: $OUTPUT_FILE"
wc -l "$OUTPUT_FILE"
