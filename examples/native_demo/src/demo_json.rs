//! 模块 5: JSON 示例
//! 演示 autozig_json 的高性能 JSON 解析功能

use autozig_json::*;

pub fn run_json_demo() {
    println!("\n{}", "=".repeat(60));
    println!("模块 5: JSON 示例");
    println!("{}", "=".repeat(60));
    
    // ========== 基本解析 ==========
    println!("\n[1] 基本 JSON 解析...");
    
    // Null
    let null_val = from_str("null").unwrap();
    println!("  null: {:?}", null_val);
    
    // Boolean
    let true_val = from_str("true").unwrap();
    let false_val = from_str("false").unwrap();
    println!("  true: {:?}", true_val);
    println!("  false: {:?}", false_val);
    
    // Number
    let num_val = from_str("42").unwrap();
    println!("  number: {:?} = {}", num_val, num_val.as_f64().unwrap());
    
    let float_val = from_str("3.14159").unwrap();
    println!("  float: {:?} = {:.5}", float_val, float_val.as_f64().unwrap());
    
    // String
    let str_val = from_str(r#""Hello, AutoZig!""#).unwrap();
    println!("  string: {:?} = {}", str_val, str_val.as_str().unwrap());
    
    // ========== 数组解析 ==========
    println!("\n[2] 数组解析...");
    let arr_json = r#"[1, 2, 3, 4, 5]"#;
    let arr_val = from_str(arr_json).unwrap();
    
    if let Some(arr) = arr_val.as_array() {
        println!("  数组长度: {}", arr.len());
        print!("  元素: [");
        for (i, elem) in arr.iter().enumerate() {
            if i > 0 { print!(", "); }
            print!("{}", elem.as_f64().unwrap() as i32);
        }
        println!("]");
    }
    
    // ========== 对象解析 ==========
    println!("\n[3] 对象解析...");
    let obj_json = r#"{"name": "AutoZig", "version": 1, "stable": true}"#;
    let obj_val = from_str(obj_json).unwrap();
    
    println!("  name: {}", obj_val["name"].as_str().unwrap());
    println!("  version: {}", obj_val["version"].as_f64().unwrap() as u32);
    println!("  stable: {}", obj_val["stable"].as_bool().unwrap());
    
    // ========== 嵌套结构 ==========
    println!("\n[4] 嵌套结构解析...");
    let nested_json = r#"
    {
        "project": "AutoZig-Bevy",
        "stats": {
            "stars": 100,
            "forks": 20
        },
        "contributors": [
            {"name": "Alice", "commits": 150},
            {"name": "Bob", "commits": 80}
        ]
    }
    "#;
    
    let nested_val = from_str(nested_json).unwrap();
    println!("  project: {}", nested_val["project"].as_str().unwrap());
    println!("  stats.stars: {}", nested_val["stats"]["stars"].as_f64().unwrap() as u32);
    println!("  contributors[0].name: {}", nested_val["contributors"][0]["name"].as_str().unwrap());
    println!("  contributors[1].commits: {}", nested_val["contributors"][1]["commits"].as_f64().unwrap() as u32);
    
    // ========== json! 宏 ==========
    println!("\n[5] json! 宏构建...");
    let config = json!({
        "app": "Native Demo",
        "window": {
            "width": 1280,
            "height": 720,
            "fullscreen": false
        },
        "features": ["ecs", "math", "state", "time", "json"]
    });
    
    println!("  构建的 JSON:");
    println!("  app: {}", config["app"].as_str().unwrap());
    println!("  window.width: {}", config["window"]["width"].as_f64().unwrap() as u32);
    println!("  features 数量: {}", config["features"].as_array().unwrap().len());
    
    // ========== 序列化 ==========
    println!("\n[6] JSON 序列化...");
    let data = json!({
        "status": "success",
        "code": 200,
        "data": {
            "message": "Operation completed"
        }
    });
    
    let json_str = to_string(&data);
    println!("  紧凑格式:");
    println!("  {}", json_str);
    
    let json_pretty = to_string_pretty(&data);
    println!("\n  美化格式:");
    for line in json_pretty.lines() {
        println!("  {}", line);
    }
    
    // ========== 性能特性 ==========
    println!("\n[7] AutoZig JSON 特性...");
    println!("  ✓ SIMD 优化: Zig 后端使用 SIMD 加速");
    println!("  ✓ Tape-based: 最快的 JSON 解析架构");
    println!("  ✓ 零依赖: 无 serde, 无 syn, 无 quote");
    println!("  ✓ 快速编译: 手写 proc-macro");
    println!("  ✓ 类型安全: 支持 AutoDeserialize 派生宏");
    
    // ========== 实际应用 ==========
    println!("\n[8] 实际应用场景...");
    println!("  • 配置文件解析 (game_config.json)");
    println!("  • 资产元数据 (asset_manifest.json)");
    println!("  • 网络通信 (API 请求/响应)");
    println!("  • 游戏存档 (save_data.json)");
    println!("  • 关卡数据 (level_01.json)");
    
    println!("\n模块 5 完成 ✓\n");
}