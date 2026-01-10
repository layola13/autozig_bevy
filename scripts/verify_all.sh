#!/bin/bash

# AutoZig Bevy 模块验证脚本
# 批量编译所有模块并统计 API 完成度

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# 统计变量
TOTAL=0
PASSED=0
FAILED=0
TOTAL_AUTOZIG_TYPES=0
TOTAL_BEVY_TYPES=0
TOTAL_COMPILE_TIME=0
TOTAL_TEST_TIME=0
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# 配置选项
RUN_TESTS="${RUN_TESTS:-yes}"  # 默认运行测试

# 开始时间
START_TIME=$(date +%s)

# 获取脚本目录
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
AUTOZIG_BEVY_DIR="$(dirname "$SCRIPT_DIR")"
BEVY_DIR="/home/sonygod/projects/autozig/bevy/crates"
OUTPUT_FILE="$AUTOZIG_BEVY_DIR/docs/API_VERIFICATION_REPORT.md"

# 日志函数
log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[✓]${NC} $1"; }
log_error() { echo -e "${RED}[✗]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[!]${NC} $1"; }
log_section() {
    echo -e "\n${BLUE}======================================${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}======================================${NC}\n"
}

# 统计模块公开类型数量
count_pub_types() {
    local src_dir="$1"
    find "$src_dir" -name "*.rs" -exec grep -E "^pub (struct|enum|trait) [A-Z]" {} \; 2>/dev/null | wc -l
}

# 统计 Bevy prelude 类型数量（更准确的方法）
count_bevy_prelude() {
    local lib_rs="$1/lib.rs"
    if [ ! -f "$lib_rs" ]; then
        echo 0
        return
    fi
    
    # 提取 prelude 块并计算类型
    local prelude_block=$(sed -n '/pub mod prelude/,/^}/p' "$lib_rs" 2>/dev/null)
    echo "$prelude_block" | grep -oE "[A-Z][A-Za-z0-9_]+" | sort -u | grep -vE "^(Doc|Hidden|Cfg)$" | wc -l
}

# 验证单个模块
verify_module() {
    local module_name="$1"
    local autozig_dir="$AUTOZIG_BEVY_DIR/autozig-$module_name"
    local bevy_dir="$BEVY_DIR/bevy_$module_name"
    
    TOTAL=$((TOTAL + 1))
    
    # 检查目录存在
    if [ ! -d "$autozig_dir" ]; then
        log_warning "模块 autozig-$module_name 不存在"
        return 1
    fi
    
    log_info "验证 autozig-$module_name ..."
    
    # 编译检查
    cd "$autozig_dir"
    
    local compile_log="/tmp/autozig_bevy_${module_name}.log"
    local module_start=$(date +%s%3N)  # 毫秒级计时
    
    if cargo check 2>&1 | tee "$compile_log" | grep -qE "error\["; then
        local module_end=$(date +%s%3N)
        local module_time=$((module_end - module_start))
        log_error "autozig-$module_name: 编译失败"
        FAILED=$((FAILED + 1))
        
        # 统计类型（即使编译失败）
        local autozig_types=$(count_pub_types "$autozig_dir/src")
        local bevy_types=0
        if [ -d "$bevy_dir/src" ]; then
            bevy_types=$(count_bevy_prelude "$bevy_dir/src")
        fi
        
        echo "$module_name|$autozig_types|$bevy_types|FAIL|$module_time" >> /tmp/autozig_bevy_stats.txt
        
        cd - > /dev/null
        return 1
    fi
    
    local module_end=$(date +%s%3N)
    local module_time=$((module_end - module_start))
    TOTAL_COMPILE_TIME=$((TOTAL_COMPILE_TIME + module_time))
    
    log_success "autozig-$module_name: 编译成功"
    PASSED=$((PASSED + 1))
    
    # 统计类型
    local autozig_types=$(count_pub_types "$autozig_dir/src")
    local bevy_types=0
    if [ -d "$bevy_dir/src" ]; then
        bevy_types=$(count_bevy_prelude "$bevy_dir/src")
    fi
    
    TOTAL_AUTOZIG_TYPES=$((TOTAL_AUTOZIG_TYPES + autozig_types))
    TOTAL_BEVY_TYPES=$((TOTAL_BEVY_TYPES + bevy_types))
    
    echo "$module_name|$autozig_types|$bevy_types|PASS|$module_time" >> /tmp/autozig_bevy_stats.txt
    
    log_info "  编译时间: ${module_time}ms"
    
    # 运行测试（如果启用）
    if [ "$RUN_TESTS" = "yes" ]; then
        local test_start=$(date +%s%3N)
        local test_log="/tmp/autozig_bevy_test_${module_name}.log"
        
        if cargo test 2>&1 | tee "$test_log" | grep -qE "test result:.*FAILED"; then
            local test_end=$(date +%s%3N)
            local test_time=$((test_end - test_start))
            TOTAL_TEST_TIME=$((TOTAL_TEST_TIME + test_time))
            TESTS_RUN=$((TESTS_RUN + 1))
            TESTS_FAILED=$((TESTS_FAILED + 1))
            log_error "  测试失败 (${test_time}ms)"
            echo "$module_name|FAIL|$test_time" >> /tmp/autozig_bevy_tests.txt
        else
            local test_end=$(date +%s%3N)
            local test_time=$((test_end - test_start))
            TOTAL_TEST_TIME=$((TOTAL_TEST_TIME + test_time))
            TESTS_RUN=$((TESTS_RUN + 1))
            TESTS_PASSED=$((TESTS_PASSED + 1))
            log_success "  测试通过 (${test_time}ms)"
            echo "$module_name|PASS|$test_time" >> /tmp/autozig_bevy_tests.txt
        fi
    fi
    
    cd - > /dev/null
    return 0
}

# 生成 Markdown 报告
generate_report() {
    log_section "生成验证报告"
    
    echo "# AutoZig Bevy 模块验证报告" > "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
    echo "> 通过编译验证和类型统计" >> "$OUTPUT_FILE"
    echo "> 生成时间: $(date '+%Y-%m-%d %H:%M:%S')" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
    
    # 汇总表
    echo "## 编译状态汇总" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
    echo "| 指标 | 数量 |" >> "$OUTPUT_FILE"
    echo "|------|------|" >> "$OUTPUT_FILE"
    echo "| 总模块数 | $TOTAL |" >> "$OUTPUT_FILE"
    echo "| ✅ 编译通过 | $PASSED |" >> "$OUTPUT_FILE"
    echo "| ❌ 编译失败 | $FAILED |" >> "$OUTPUT_FILE"
    echo "| 通过率 | $((PASSED * 100 / TOTAL))% |" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
    
    # 类型统计表
    echo "## API 类型统计" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
    echo "| 模块 | AutoZig 类型 | Bevy Prelude | 完成度 | 编译时间 | 状态 |" >> "$OUTPUT_FILE"
    echo "|------|-------------|--------------|--------|---------|------|" >> "$OUTPUT_FILE"
    
    # 按完成度排序输出
    sort -t'|' -k2 -rn /tmp/autozig_bevy_stats.txt | while IFS='|' read -r module autozig bevy status compile_time; do
        if [ "$bevy" -gt 0 ]; then
            pct=$((autozig * 100 / bevy))
        else
            pct="N/A"
        fi
        
        if [ "$status" = "PASS" ]; then
            status_icon="✅"
        else
            status_icon="❌"
        fi
        
        echo "| $module | $autozig | $bevy | ${pct}% | ${compile_time}ms | $status_icon |" >> "$OUTPUT_FILE"
    done
    
    echo "" >> "$OUTPUT_FILE"
    
    # 总计
    if [ $TOTAL_BEVY_TYPES -gt 0 ]; then
        total_pct=$((TOTAL_AUTOZIG_TYPES * 100 / TOTAL_BEVY_TYPES))
    else
        total_pct=0
    fi
    
    echo "## 总体完成度" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
    echo "- AutoZig 公开类型总数: **$TOTAL_AUTOZIG_TYPES**" >> "$OUTPUT_FILE"
    echo "- Bevy Prelude 类型总数: **$TOTAL_BEVY_TYPES**" >> "$OUTPUT_FILE"
    echo "- 总体完成度: **${total_pct}%**" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
    
    # 时间统计
    local end_time=$(date +%s)
    local total_seconds=$((end_time - START_TIME))
    
    echo "## 编译时间统计" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
    echo "- 模块编译总时间: **${TOTAL_COMPILE_TIME}ms** ($((TOTAL_COMPILE_TIME / 1000))秒)" >> "$OUTPUT_FILE"
    echo "- 脚本执行总时间: **${total_seconds}秒**" >> "$OUTPUT_FILE"
    echo "- 平均每模块编译: **$((TOTAL_COMPILE_TIME / TOTAL))ms**" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
    
    # 测试统计（从文件读取）
    if [ "$RUN_TESTS" = "yes" ] && [ -s /tmp/autozig_bevy_tests.txt ]; then
        local tests_run=$(wc -l < /tmp/autozig_bevy_tests.txt)
        local tests_passed=$(grep -c "|PASS|" /tmp/autozig_bevy_tests.txt 2>/dev/null || echo 0)
        local tests_failed=$(grep -c "|FAIL|" /tmp/autozig_bevy_tests.txt 2>/dev/null || echo 0)
        local test_total_time=0
        while IFS='|' read -r module status time; do
            test_total_time=$((test_total_time + time))
        done < /tmp/autozig_bevy_tests.txt
        
        echo "## 测试结果统计" >> "$OUTPUT_FILE"
        echo "" >> "$OUTPUT_FILE"
        echo "| 指标 | 数量 |" >> "$OUTPUT_FILE"
        echo "|------|------|" >> "$OUTPUT_FILE"
        echo "| 测试模块数 | $tests_run |" >> "$OUTPUT_FILE"
        echo "| ✅ 测试通过 | $tests_passed |" >> "$OUTPUT_FILE"
        echo "| ❌ 测试失败 | $tests_failed |" >> "$OUTPUT_FILE"
        echo "| 测试总时间 | ${test_total_time}ms ($((test_total_time / 1000))秒) |" >> "$OUTPUT_FILE"
        if [ $tests_run -gt 0 ]; then
            echo "| 平均每模块测试 | $((test_total_time / tests_run))ms |" >> "$OUTPUT_FILE"
        fi
        echo "" >> "$OUTPUT_FILE"
        
        # 失败模块列表
        if [ $tests_failed -gt 0 ]; then
            echo "### 测试失败模块" >> "$OUTPUT_FILE"
            echo "" >> "$OUTPUT_FILE"
            grep "|FAIL|" /tmp/autozig_bevy_tests.txt | while IFS='|' read -r module status time; do
                echo "- ❌ \`$module\` (${time}ms)" >> "$OUTPUT_FILE"
            done
            echo "" >> "$OUTPUT_FILE"
        fi
    fi
    
    # 结论
    echo "## 结论" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
    if [ $FAILED -eq 0 ]; then
        echo "✅ **所有模块编译通过**" >> "$OUTPUT_FILE"
    else
        echo "❌ **有 $FAILED 个模块编译失败**" >> "$OUTPUT_FILE"
    fi
    
    if [ "$RUN_TESTS" = "yes" ] && [ -s /tmp/autozig_bevy_tests.txt ]; then
        local tests_failed_count=$(grep -c "|FAIL|" /tmp/autozig_bevy_tests.txt 2>/dev/null || echo 0)
        if [ "$tests_failed_count" -eq 0 ]; then
            echo "✅ **所有测试通过**" >> "$OUTPUT_FILE"
        else
            echo "❌ **有 $tests_failed_count 个模块测试失败**" >> "$OUTPUT_FILE"
        fi
    fi
    
    log_success "报告已生成: $OUTPUT_FILE"
}

# 主函数
main() {
    log_section "AutoZig Bevy 模块验证"
    
    # 清理临时文件
    rm -f /tmp/autozig_bevy_stats.txt
    rm -f /tmp/autozig_bevy_tests.txt
    touch /tmp/autozig_bevy_stats.txt
    touch /tmp/autozig_bevy_tests.txt
    
    log_info "测试模式: $RUN_TESTS"
    log_info "设置 RUN_TESTS=no 跳过测试"
    echo ""
    
    # 模块列表
    MODULES=(
        "app"
        "ecs"
        "math"
        "render"
        "transform"
        "mesh"
        "pbr"
        "light"
        "color"
        "input"
        "window"
        "time"
        "state"
        "asset"
        "sprite"
        "ui"
        "camera"
        "text"
        "reflect"
        "tasks"
        "ptr"
        "utils"
        "diagnostic"
        "log"
        "image"
        "hierarchy"
        "core-pipeline"
        "shader"
        "winit"
    )
    
    log_info "开始验证 ${#MODULES[@]} 个模块..."
    echo ""
    
    # 遍历验证
    for module in "${MODULES[@]}"; do
        verify_module "$module" || true
    done
    
    # 生成报告
    generate_report
    
    # 输出总结
    log_section "验证结果"
    
    local end_time=$(date +%s)
    local total_seconds=$((end_time - START_TIME))
    
    echo -e "总计: $TOTAL 个模块"
    echo -e "${GREEN}通过: $PASSED${NC}"
    echo -e "${RED}失败: $FAILED${NC}"
    echo ""
    echo -e "AutoZig 类型总数: ${CYAN}$TOTAL_AUTOZIG_TYPES${NC}"
    echo -e "Bevy Prelude 类型: ${CYAN}$TOTAL_BEVY_TYPES${NC}"
    
    if [ $TOTAL_BEVY_TYPES -gt 0 ]; then
        echo -e "总体完成度: ${GREEN}$((TOTAL_AUTOZIG_TYPES * 100 / TOTAL_BEVY_TYPES))%${NC}"
    fi
    
    echo ""
    echo -e "${YELLOW}编译时间统计:${NC}"
    echo -e "  模块编译总时间: ${CYAN}${TOTAL_COMPILE_TIME}ms${NC} ($((TOTAL_COMPILE_TIME / 1000))秒)"
    echo -e "  脚本执行总时间: ${CYAN}${total_seconds}秒${NC}"
    echo -e "  平均每模块编译: ${CYAN}$((TOTAL_COMPILE_TIME / TOTAL))ms${NC}"
    
    if [ "$RUN_TESTS" = "yes" ] && [ -s /tmp/autozig_bevy_tests.txt ]; then
        local tests_run=$(wc -l < /tmp/autozig_bevy_tests.txt)
        local tests_passed=$(grep -c "|PASS|" /tmp/autozig_bevy_tests.txt 2>/dev/null || echo 0)
        local tests_failed=$(grep -c "|FAIL|" /tmp/autozig_bevy_tests.txt 2>/dev/null || echo 0)
        local test_total_time=0
        while IFS='|' read -r module status time; do
            test_total_time=$((test_total_time + time))
        done < /tmp/autozig_bevy_tests.txt
        
        echo ""
        echo -e "${YELLOW}测试统计:${NC}"
        echo -e "  测试模块数: ${CYAN}$tests_run${NC}"
        echo -e "  ${GREEN}通过: $tests_passed${NC}  ${RED}失败: $tests_failed${NC}"
        echo -e "  测试总时间: ${CYAN}${test_total_time}ms${NC} ($((test_total_time / 1000))秒)"
        if [ $tests_run -gt 0 ]; then
            echo -e "  平均每模块测试: ${CYAN}$((test_total_time / tests_run))ms${NC}"
        fi
        
        if [ $FAILED -eq 0 ] && [ "$tests_failed" -eq 0 ]; then
            log_success "所有模块编译和测试通过！🎉"
            exit 0
        elif [ $FAILED -eq 0 ] && [ "$tests_failed" -gt 0 ]; then
            log_warning "编译通过，但有 $tests_failed 个模块测试失败"
            exit 1
        else
            log_error "有 $FAILED 个编译失败, $tests_failed 个测试失败"
            exit 1
        fi
    else
        if [ $FAILED -eq 0 ]; then
            log_success "所有模块编译通过！🎉"
            exit 0
        else
            log_error "有 $FAILED 个模块编译失败"
            exit 1
        fi
    fi
}

# 运行
main "$@"
