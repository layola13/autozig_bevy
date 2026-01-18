//! Tests for the schedule system

use autozig_app::{App, MainScheduleOrder, SystemSet};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

// Test counter for tracking system execution
static TEST_COUNTER: AtomicU32 = AtomicU32::new(0);

extern "C" fn reset_counter_c() {
    TEST_COUNTER.store(0, Ordering::SeqCst);
}
fn reset_counter() { reset_counter_c(); }

extern "C" fn increment_counter_c() {
    TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
}
fn increment_counter() { increment_counter_c(); }

extern "C" fn first_system_c() {
    TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
}
fn first_system() { first_system_c(); }

extern "C" fn pre_startup_system_c() {
    TEST_COUNTER.fetch_add(10, Ordering::SeqCst);
}
fn pre_startup_system() { pre_startup_system_c(); }

extern "C" fn startup_system_c() {
    TEST_COUNTER.fetch_add(100, Ordering::SeqCst);
}
fn startup_system() { startup_system_c(); }

extern "C" fn post_startup_system_c() {
    TEST_COUNTER.fetch_add(1000, Ordering::SeqCst);
}
fn post_startup_system() { post_startup_system_c(); }

extern "C" fn pre_update_system_c() {
    TEST_COUNTER.fetch_add(10000, Ordering::SeqCst);
}
fn pre_update_system() { pre_update_system_c(); }

extern "C" fn update_system_c() {
    TEST_COUNTER.fetch_add(100000, Ordering::SeqCst);
}
fn update_system() { update_system_c(); }

extern "C" fn post_update_system_c() {
    TEST_COUNTER.fetch_add(1000000, Ordering::SeqCst);
}
fn post_update_system() { post_update_system_c(); }

extern "C" fn last_system_c() {
    TEST_COUNTER.fetch_add(10000000, Ordering::SeqCst);
}
fn last_system() { last_system_c(); }

#[test]
fn test_schedule_order_enum() {
    // Test enum values
    assert_eq!(MainScheduleOrder::First as u8, 0);
    assert_eq!(MainScheduleOrder::PreStartup as u8, 1);
    assert_eq!(MainScheduleOrder::Startup as u8, 2);
    assert_eq!(MainScheduleOrder::PostStartup as u8, 3);
    assert_eq!(MainScheduleOrder::PreUpdate as u8, 4);
    assert_eq!(MainScheduleOrder::Update as u8, 5);
    assert_eq!(MainScheduleOrder::PostUpdate as u8, 6);
    assert_eq!(MainScheduleOrder::Last as u8, 7);
}

#[test]
fn test_schedule_is_startup() {
    // Startup schedules
    assert!(MainScheduleOrder::PreStartup.is_startup());
    assert!(MainScheduleOrder::Startup.is_startup());
    assert!(MainScheduleOrder::PostStartup.is_startup());
    
    // Non-startup schedules
    assert!(!MainScheduleOrder::First.is_startup());
    assert!(!MainScheduleOrder::PreUpdate.is_startup());
    assert!(!MainScheduleOrder::Update.is_startup());
    assert!(!MainScheduleOrder::PostUpdate.is_startup());
    assert!(!MainScheduleOrder::Last.is_startup());
}

#[test]
fn test_schedule_as_str() {
    assert_eq!(MainScheduleOrder::First.as_str(), "First");
    assert_eq!(MainScheduleOrder::PreStartup.as_str(), "PreStartup");
    assert_eq!(MainScheduleOrder::Startup.as_str(), "Startup");
    assert_eq!(MainScheduleOrder::PostStartup.as_str(), "PostStartup");
    assert_eq!(MainScheduleOrder::PreUpdate.as_str(), "PreUpdate");
    assert_eq!(MainScheduleOrder::Update.as_str(), "Update");
    assert_eq!(MainScheduleOrder::PostUpdate.as_str(), "PostUpdate");
    assert_eq!(MainScheduleOrder::Last.as_str(), "Last");
}

#[test]
fn test_schedule_all_schedules() {
    let schedules = MainScheduleOrder::all_schedules();
    assert_eq!(schedules.len(), 8);
    assert_eq!(schedules[0], MainScheduleOrder::First);
    assert_eq!(schedules[1], MainScheduleOrder::PreStartup);
    assert_eq!(schedules[2], MainScheduleOrder::Startup);
    assert_eq!(schedules[3], MainScheduleOrder::PostStartup);
    assert_eq!(schedules[4], MainScheduleOrder::PreUpdate);
    assert_eq!(schedules[5], MainScheduleOrder::Update);
    assert_eq!(schedules[6], MainScheduleOrder::PostUpdate);
    assert_eq!(schedules[7], MainScheduleOrder::Last);
}

#[test]
fn test_add_systems_api() {
    let mut app = App::new();
    
    // Test that add_systems returns &mut Self for chaining
    app.add_systems(MainScheduleOrder::Update, increment_counter)
       .add_systems(MainScheduleOrder::PreUpdate, increment_counter)
       .add_systems(MainScheduleOrder::PostUpdate, increment_counter);
}

#[test]
fn test_configure_sets_api() {
    let mut app = App::new();
    
    let set1 = SystemSet::new(1);
    let set2 = SystemSet::new(2);
    
    // Test that configure_sets returns &mut Self for chaining
    app.configure_sets(MainScheduleOrder::Update, set1)
       .configure_sets(MainScheduleOrder::PreUpdate, set2);
}

#[test]
fn test_init_resource_api() {
    #[derive(Default)]
    struct TestResource {
        value: i32,
    }
    
    let mut app = App::new();
    
    // Test that init_resource returns &mut Self for chaining
    app.init_resource::<TestResource>()
       .init_resource::<TestResource>(); // Should not panic on second call
}

#[test]
fn test_system_set_creation() {
    let set1 = SystemSet::new(1);
    let set2 = SystemSet::new(2);
    let set3 = SystemSet::new(1);
    
    assert_eq!(set1.id, 1);
    assert_eq!(set2.id, 2);
    assert_eq!(set1, set3);
    assert_ne!(set1, set2);
}

#[test]
fn test_schedule_execution_order() {
    reset_counter();
    TEST_COUNTER.store(0, Ordering::SeqCst);
    
    let mut app = App::new();
    
    // Add systems to all schedules
    app.add_systems(MainScheduleOrder::First, first_system)
       .add_systems(MainScheduleOrder::PreStartup, pre_startup_system)
       .add_systems(MainScheduleOrder::Startup, startup_system)
       .add_systems(MainScheduleOrder::PostStartup, post_startup_system)
       .add_systems(MainScheduleOrder::PreUpdate, pre_update_system)
       .add_systems(MainScheduleOrder::Update, update_system)
       .add_systems(MainScheduleOrder::PostUpdate, post_update_system)
       .add_systems(MainScheduleOrder::Last, last_system);
    
    // Run one update (should execute all schedules)
    app.update();
    
    let counter = TEST_COUNTER.load(Ordering::SeqCst);
    
    // Verify all systems ran
    // Expected: 1 + 10 + 100 + 1000 + 10000 + 100000 + 1000000 + 10000000 = 11111111
    assert_eq!(counter, 11111111, "All systems should have run in order");
}

#[test]
fn test_startup_runs_once() {
    TEST_COUNTER.store(0, Ordering::SeqCst);
    
    let mut app = App::new();
    
    // Add startup systems
    app.add_systems(MainScheduleOrder::PreStartup, increment_counter)
       .add_systems(MainScheduleOrder::Startup, increment_counter)
       .add_systems(MainScheduleOrder::PostStartup, increment_counter);
    
    // First update - startup systems should run
    app.update();
    let first_count = TEST_COUNTER.load(Ordering::SeqCst);
    assert_eq!(first_count, 3, "Startup systems should run on first update");
    
    // Second update - startup systems should NOT run again
    app.update();
    let second_count = TEST_COUNTER.load(Ordering::SeqCst);
    assert_eq!(second_count, 3, "Startup systems should not run on second update");
    
    // Third update - still should not run
    app.update();
    let third_count = TEST_COUNTER.load(Ordering::SeqCst);
    assert_eq!(third_count, 3, "Startup systems should not run on third update");
}

#[test]
fn test_update_runs_every_frame() {
    TEST_COUNTER.store(0, Ordering::SeqCst);
    
    let mut app = App::new();
    
    // Add update systems
    app.add_systems(MainScheduleOrder::PreUpdate, increment_counter)
       .add_systems(MainScheduleOrder::Update, increment_counter)
       .add_systems(MainScheduleOrder::PostUpdate, increment_counter);
    
    // First update
    app.update();
    assert_eq!(TEST_COUNTER.load(Ordering::SeqCst), 3);
    
    // Second update - should run again
    app.update();
    assert_eq!(TEST_COUNTER.load(Ordering::SeqCst), 6);
    
    // Third update - should run again
    app.update();
    assert_eq!(TEST_COUNTER.load(Ordering::SeqCst), 9);
}

#[test]
fn test_first_and_last_run_every_frame() {
    TEST_COUNTER.store(0, Ordering::SeqCst);
    
    let mut app = App::new();
    
    // Add First and Last systems
    app.add_systems(MainScheduleOrder::First, increment_counter)
       .add_systems(MainScheduleOrder::Last, increment_counter);
    
    // First update
    app.update();
    assert_eq!(TEST_COUNTER.load(Ordering::SeqCst), 2);
    
    // Second update
    app.update();
    assert_eq!(TEST_COUNTER.load(Ordering::SeqCst), 4);
    
    // Third update
    app.update();
    assert_eq!(TEST_COUNTER.load(Ordering::SeqCst), 6);
}

#[test]
fn test_mixed_startup_and_update() {
    TEST_COUNTER.store(0, Ordering::SeqCst);
    
    let mut app = App::new();
    
    extern "C" fn startup_inc_c() {
        TEST_COUNTER.fetch_add(1000, Ordering::SeqCst);
    }
    fn startup_inc() { startup_inc_c(); }
    
    extern "C" fn update_inc_c() {
        TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    }
    fn update_inc() { update_inc_c(); }
    
    // Add both startup and update systems
    app.add_systems(MainScheduleOrder::Startup, startup_inc)
       .add_systems(MainScheduleOrder::Update, update_inc);
    
    // First update: both should run (1000 + 1 = 1001)
    app.update();
    assert_eq!(TEST_COUNTER.load(Ordering::SeqCst), 1001);
    
    // Second update: only Update should run (1001 + 1 = 1002)
    app.update();
    assert_eq!(TEST_COUNTER.load(Ordering::SeqCst), 1002);
    
    // Third update: only Update should run (1002 + 1 = 1003)
    app.update();
    assert_eq!(TEST_COUNTER.load(Ordering::SeqCst), 1003);
}

#[test]
fn test_multiple_systems_same_schedule() {
    TEST_COUNTER.store(0, Ordering::SeqCst);
    
    let mut app = App::new();
    
    extern "C" fn add_1_c() {
        TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    }
    fn add_1() { add_1_c(); }
    
    extern "C" fn add_2_c() {
        TEST_COUNTER.fetch_add(2, Ordering::SeqCst);
    }
    fn add_2() { add_2_c(); }
    
    extern "C" fn add_4_c() {
        TEST_COUNTER.fetch_add(4, Ordering::SeqCst);
    }
    fn add_4() { add_4_c(); }
    
    // Add multiple systems to the same schedule
    app.add_systems(MainScheduleOrder::Update, add_1)
       .add_systems(MainScheduleOrder::Update, add_2)
       .add_systems(MainScheduleOrder::Update, add_4);
    
    // All three systems should run
    app.update();
    assert_eq!(TEST_COUNTER.load(Ordering::SeqCst), 7); // 1 + 2 + 4 = 7
    
    app.update();
    assert_eq!(TEST_COUNTER.load(Ordering::SeqCst), 14); // 7 + 7 = 14
}

#[test]
fn test_empty_app_update() {
    let mut app = App::new();
    
    // Should not panic with no systems
    app.update();
    app.update();
    app.update();
}

#[test]
fn test_app_chaining() {
    let mut app = App::new();
    
    // Test method chaining
    app.add_systems(MainScheduleOrder::Update, increment_counter)
       .configure_sets(MainScheduleOrder::Update, SystemSet::new(1))
       .init_resource::<TestResourceForChaining>()
       .add_systems(MainScheduleOrder::PreUpdate, increment_counter)
       .update();
    
    #[derive(Default)]
    struct TestResourceForChaining {
        _value: i32,
    }
}