//! Basic logging example for autozig-log

use autozig_log::*;

fn main() {
    // Initialize logger
    init();
    
    // Set minimum log level to Trace to see all messages
    set_min_level(LogLevel::Trace);
    
    // Set console available (for non-WASM environments, this uses stderr)
    set_console_available(true);
    
    println!("=== AutoZig Log Example ===\n");
    
    // Basic logging at different levels
    trace!("This is a trace message - very detailed");
    debug!("This is a debug message - for debugging");
    info!("This is an info message - general information");
    warn!("This is a warning message - something to watch");
    error!("This is an error message - something went wrong");
    
    println!("\n=== Testing with formatting ===\n");
    
    // Logging with formatting
    let user_name = "Alice";
    let user_age = 30;
    info!("User {} is {} years old", user_name, user_age);
    
    let items = vec!["apple", "banana", "cherry"];
    debug!("Shopping list has {} items: {:?}", items.len(), items);
    
    println!("\n=== Testing log filtering ===\n");
    
    // Change minimum level to Warn - only warnings and errors will show
    set_min_level(LogLevel::Warn);
    info!("This info message will NOT appear");
    debug!("This debug message will NOT appear");
    warn!("This warning WILL appear");
    error!("This error WILL appear");
    
    println!("\n=== Testing with different module paths ===\n");
    
    // Reset to Info level
    set_min_level(LogLevel::Info);
    
    // Direct log calls with custom module names
    log(LogLevel::Info, "custom::module", "Direct log call with custom module");
    log(LogLevel::Warn, "network::http", "Connection timeout after 30s");
    log(LogLevel::Error, "database::query", "Failed to execute query: table not found");
    
    println!("\n=== Testing with special characters ===\n");
    
    info!("Unicode works: 你好世界 🌍 🚀");
    info!("Newline test:\nLine 1\nLine 2\nLine 3");
    info!("Tab test: Col1\tCol2\tCol3");
    
    println!("\n=== Performance test ===\n");
    
    use std::time::Instant;
    
    let start = Instant::now();
    for i in 0..1000 {
        trace!("Performance test iteration {}", i);
    }
    let elapsed = start.elapsed();
    
    info!("Logged 1000 trace messages in {:?}", elapsed);
    
    // Note: Most won't show unless min level is Trace
    set_min_level(LogLevel::Trace);
    let start = Instant::now();
    for i in 0..100 {
        trace!("Visible iteration {}", i);
    }
    let elapsed = start.elapsed();
    info!("Logged 100 visible trace messages in {:?}", elapsed);
    
    println!("\n=== Testing log level checking ===\n");
    
    set_min_level(LogLevel::Info);
    println!("Current min level: {:?}", min_level());
    println!("Is Trace enabled? {}", is_enabled(LogLevel::Trace));
    println!("Is Debug enabled? {}", is_enabled(LogLevel::Debug));
    println!("Is Info enabled? {}", is_enabled(LogLevel::Info));
    println!("Is Warn enabled? {}", is_enabled(LogLevel::Warn));
    println!("Is Error enabled? {}", is_enabled(LogLevel::Error));
    
    println!("\n=== Cleanup ===\n");
    
    info!("Shutting down logger");
    shutdown();
    
    println!("Example complete!");
}