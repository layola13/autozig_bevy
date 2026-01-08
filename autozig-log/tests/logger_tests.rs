//! Integration tests for autozig-log

use autozig_log::*;

#[test]
fn test_log_level_ordering() {
    assert!(LogLevel::Trace < LogLevel::Debug);
    assert!(LogLevel::Debug < LogLevel::Info);
    assert!(LogLevel::Info < LogLevel::Warn);
    assert!(LogLevel::Warn < LogLevel::Error);
}

#[test]
fn test_log_level_conversion() {
    assert_eq!(LogLevel::from_str("trace"), Some(LogLevel::Trace));
    assert_eq!(LogLevel::from_str("DEBUG"), Some(LogLevel::Debug));
    assert_eq!(LogLevel::from_str("info"), Some(LogLevel::Info));
    assert_eq!(LogLevel::from_str("WARN"), Some(LogLevel::Warn));
    assert_eq!(LogLevel::from_str("error"), Some(LogLevel::Error));
    assert_eq!(LogLevel::from_str("ERR"), Some(LogLevel::Error));
    assert_eq!(LogLevel::from_str("invalid"), None);
}

#[test]
fn test_log_level_display() {
    assert_eq!(format!("{}", LogLevel::Trace), "TRACE");
    assert_eq!(format!("{}", LogLevel::Debug), "DEBUG");
    assert_eq!(format!("{}", LogLevel::Info), "INFO");
    assert_eq!(format!("{}", LogLevel::Warn), "WARN");
    assert_eq!(format!("{}", LogLevel::Error), "ERROR");
}

#[test]
fn test_min_level_setting() {
    // Set to Warn
    set_min_level(LogLevel::Warn);
    assert_eq!(min_level(), LogLevel::Warn);
    
    // Set to Debug
    set_min_level(LogLevel::Debug);
    assert_eq!(min_level(), LogLevel::Debug);
    
    // Set to Error
    set_min_level(LogLevel::Error);
    assert_eq!(min_level(), LogLevel::Error);
    
    // Reset to Info for other tests
    set_min_level(LogLevel::Info);
}

#[test]
fn test_is_enabled() {
    // Ensure clean state
    set_min_level(LogLevel::Error);
    set_min_level(LogLevel::Info);
    
    // Verify current level
    assert_eq!(min_level(), LogLevel::Info);
    
    assert!(!is_enabled(LogLevel::Trace));
    assert!(!is_enabled(LogLevel::Debug));
    assert!(is_enabled(LogLevel::Info));
    assert!(is_enabled(LogLevel::Warn));
    assert!(is_enabled(LogLevel::Error));
    
    set_min_level(LogLevel::Trace);
    assert!(is_enabled(LogLevel::Trace));
    assert!(is_enabled(LogLevel::Debug));
    
    // Reset
    set_min_level(LogLevel::Info);
}

#[test]
fn test_init_and_shutdown() {
    init();
    shutdown();
}

#[test]
fn test_console_available() {
    set_console_available(true);
    set_console_available(false);
    set_console_available(true);
}

#[test]
fn test_log_basic() {
    init();
    set_min_level(LogLevel::Trace);
    set_console_available(false); // Disable console output for tests
    
    log(LogLevel::Info, "test_module", "Test message");
    log(LogLevel::Warn, "test_module", "Warning message");
    log(LogLevel::Error, "test_module", "Error message");
    
    shutdown();
}

#[test]
fn test_log_macros() {
    init();
    set_min_level(LogLevel::Trace);
    set_console_available(false);
    
    trace!("This is a trace message");
    debug!("This is a debug message");
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");
    
    // Test with formatting
    let value = 42;
    info!("Value is: {}", value);
    warn!("Multiple values: {} and {}", value, "text");
    
    shutdown();
}

#[test]
fn test_log_filtering() {
    init();
    set_console_available(false);
    
    // Set minimum level to Warn
    set_min_level(LogLevel::Warn);
    
    // These should not be logged (below minimum level)
    trace!("Should not appear");
    debug!("Should not appear");
    info!("Should not appear");
    
    // These should be logged
    warn!("Should appear");
    error!("Should appear");
    
    shutdown();
}

#[test]
fn test_log_with_long_messages() {
    init();
    set_min_level(LogLevel::Info);
    set_console_available(false);
    
    let long_message = "A".repeat(1000);
    info!("Long message: {}", long_message);
    
    shutdown();
}

#[test]
fn test_log_with_special_characters() {
    init();
    set_min_level(LogLevel::Info);
    set_console_available(false);
    
    info!("Message with newline\nand tab\there");
    info!("Unicode characters: 你好世界 🌍");
    info!("Special chars: !@#$%^&*()");
    
    shutdown();
}

#[test]
fn test_concurrent_logging() {
    use std::thread;
    
    init();
    set_min_level(LogLevel::Info);
    set_console_available(false);
    
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                for j in 0..10 {
                    info!("Thread {} message {}", i, j);
                }
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    shutdown();
}

#[test]
fn test_log_level_as_str() {
    assert_eq!(LogLevel::Trace.as_str(), "TRACE");
    assert_eq!(LogLevel::Debug.as_str(), "DEBUG");
    assert_eq!(LogLevel::Info.as_str(), "INFO");
    assert_eq!(LogLevel::Warn.as_str(), "WARN");
    assert_eq!(LogLevel::Error.as_str(), "ERROR");
}