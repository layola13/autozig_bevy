//! UUID和时间戳工具使用示例

use autozig_utils::prelude::*;

fn main() {
    println!("=== UUID Demo ===\n");
    uuid_demo();
    
    println!("\n=== Time Demo ===\n");
    time_demo();
    
    println!("\n=== Concurrency Demo ===\n");
    concurrency_demo();
}

fn uuid_demo() {
    // 生成UUID
    println!("生成UUID:");
    let uuid1 = Uuid::new();
    let uuid2 = Uuid::new();
    let uuid3 = Uuid::new();
    
    println!("  UUID 1: {}", uuid1);
    println!("  UUID 2: {}", uuid2);
    println!("  UUID 3: {}", uuid3);
    
    // UUID转换
    println!("\nUUID转换:");
    let uuid_str = uuid1.to_string();
    println!("  字符串: {}", uuid_str);
    println!("  u128:   {}", uuid1.as_u128());
    println!("  字节:   {:?}", uuid1.as_bytes());
    
    // 从字符串解析
    println!("\n从字符串解析:");
    if let Some(parsed) = Uuid::from_str(&uuid_str) {
        println!("  解析成功: {}", parsed);
        println!("  相等性检查: {}", parsed == uuid1);
    }
    
    // nil UUID
    println!("\nnil UUID:");
    let nil = Uuid::nil();
    println!("  {}", nil);
    println!("  is_nil? {}", nil.is_nil());
    
    // u128转换
    println!("\nu128转换:");
    let value: u128 = 123456789012345678901234567890;
    let from_u128 = Uuid::from_u128(value);
    println!("  从 u128: {}", from_u128);
    println!("  转回 u128: {}", from_u128.as_u128());
}

fn time_demo() {
    // 时间戳
    println!("当前时间戳:");
    let now = Instant::now();
    println!("  微秒: {}", now.as_micros());
    println!("  毫秒: {}", now.as_millis());
    println!("  秒:   {}", now.as_secs());
    println!("  秒(f64): {:.3}", now.as_secs_f64());
    
    // Duration创建
    println!("\nDuration创建:");
    let dur1 = Duration::from_secs(5);
    let dur2 = Duration::from_millis(3500);
    let dur3 = Duration::from_secs_f64(2.5);
    
    println!("  5秒:      {}", dur1);
    println!("  3500毫秒: {}", dur2);
    println!("  2.5秒:    {}", dur3);
    
    // Duration算术
    println!("\nDuration算术:");
    let sum = dur1 + dur2;
    let diff = dur1 - dur3;
    println!("  5s + 3.5s = {}", sum);
    println!("  5s - 2.5s = {}", diff);
    println!("  2.5s * 3  = {}", dur3.mul_i64(3));
    
    // 计时器
    println!("\n计时器:");
    let mut timer = Timer::new();
    
    // 模拟工作
    let mut sum = 0u64;
    for i in 0..1000000 {
        sum = sum.wrapping_add(i);
    }
    
    let elapsed = timer.elapsed();
    println!("  计算耗时: {}", elapsed);
    println!("  结果: {}", sum);
    
    // 重置计时器
    println!("\n重置计时器");
    let old_time = timer.reset();
    println!("  上次计时: {}", old_time);
    
    // 再次计时
    for i in 0..500000 {
        sum = sum.wrapping_add(i);
    }
    println!("  新计时: {}", timer.elapsed());
}

fn concurrency_demo() {
    // 原子计数器
    println!("原子计数器:");
    let mut counter = AtomicCounter::new(0);
    
    println!("  初始值: {}", counter.load());
    
    counter.store(10);
    println!("  存储10: {}", counter.load());
    
    let old = counter.increment();
    println!("  递增(旧值={}): {}", old, counter.load());
    
    let old = counter.fetch_add(5);
    println!("  加5(旧值={}): {}", old, counter.load());
    
    // 原子布尔
    println!("\n原子布尔:");
    let mut flag = AtomicBool::new(false);
    
    println!("  初始值: {}", flag.load());
    
    flag.store(true);
    println!("  存储true: {}", flag.load());
    
    let old = flag.swap(false);
    println!("  交换为false(旧值={}): {}", old, flag.load());
    
    // 自旋锁
    println!("\n自旋锁:");
    let mut lock = SpinLock::new();
    
    println!("  锁定前: is_locked={}", lock.is_locked());
    
    lock.lock();
    println!("  锁定后: is_locked={}", lock.is_locked());
    
    if !lock.try_lock() {
        println!("  try_lock失败(已锁定)");
    }
    
    lock.unlock();
    println!("  解锁后: is_locked={}", lock.is_locked());
    
    // OnceFlag
    println!("\nOnceFlag:");
    let mut once = OnceFlag::new();
    let mut init_count = 0;
    
    println!("  初始状态: is_initialized={}", once.is_initialized());
    
    once.call_once(|| {
        init_count += 1;
        println!("  第一次调用: 执行初始化");
    });
    
    once.call_once(|| {
        init_count += 1;
        println!("  第二次调用: 不应该执行");
    });
    
    println!("  初始化次数: {}", init_count);
    println!("  最终状态: is_initialized={}", once.is_initialized());
}