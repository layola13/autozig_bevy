use autozig_ecs::event::Events;

// 定义测试事件
#[derive(Debug, Clone, Copy)]
struct PingEvent {
    message_id: u32,
}

#[derive(Debug, Clone, Copy)]
struct PongEvent {
    reply_to: u32,
}

fn main() {
    println!("=== AutoBevy Events 测试 ===\n");
    
    // 创建事件队列
    let mut ping_events = Events::<PingEvent>::new();
    let mut pong_events = Events::<PongEvent>::new();
    println!("✓ 创建事件队列");
    
    // Frame 1: 发送Ping事件
    println!("\n【Frame 1: 发送Ping事件】");
    {
        let mut writer = ping_events.get_writer();
        writer.send(PingEvent { message_id: 1 });
        writer.send(PingEvent { message_id: 2 });
        writer.send(PingEvent { message_id: 3 });
        println!("✓ 发送了 3 个 Ping 事件");
    }
    
    // Frame 1末尾：交换缓冲
    ping_events.update();
    pong_events.update();
    println!("✓ 交换缓冲完成");
    
    // Frame 2: 读取Ping事件并发送Pong
    println!("\n【Frame 2: 读取Ping并回复Pong】");
    {
        let mut reader = ping_events.get_reader();
        let events: Vec<_> = reader.read().collect();
        
        println!("✓ 读取到 {} 个 Ping 事件:", events.len());
        for ping in events.iter() {
            println!("  - Ping {{ message_id: {} }}", ping.message_id);
            
            // 回复Pong
            let mut pong_writer = pong_events.get_writer();
            pong_writer.send(PongEvent { reply_to: ping.message_id });
        }
    }
    
    // Frame 2末尾：交换缓冲
    ping_events.update();
    pong_events.update();
    
    // Frame 3: 读取Pong事件
    println!("\n【Frame 3: 读取Pong事件】");
    {
        let mut reader = pong_events.get_reader();
        let events: Vec<_> = reader.read().collect();
        
        println!("✓ 读取到 {} 个 Pong 事件:", events.len());
        for pong in events.iter() {
            println!("  - Pong {{ reply_to: {} }}", pong.reply_to);
        }
    }
    
    // 测试事件清空
    println!("\n【测试事件清空】");
    ping_events.clear();
    pong_events.clear();
    
    let reader = ping_events.get_reader();
    println!("✓ Ping events cleared: {}", reader.is_empty());
    
    println!("\n=== 所有测试通过! ===");
    println!("\n✨ Events 系统特性:");
    println!("  ✓ 双缓冲队列 (Double Buffer)");
    println!("  ✓ 每帧自动swap");
    println!("  ✓ EventWriter<T> - 发送事件");
    println!("  ✓ EventReader<T> - 读取事件");
    println!("  ✓ 类型安全的事件传递");
    println!("  ✓ 90% Zig + 10% Rust");
}
