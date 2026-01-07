// 基础片段着色器
// 输入: 插值后的颜色
// 输出: 最终颜色

struct FragmentInput {
    @location(0) color: vec4<f32>,
}

@fragment
fn main(input: FragmentInput) -> @location(0) vec4<f32> {
    return input.color;
}