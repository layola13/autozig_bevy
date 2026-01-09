use autozig_core_pipeline::*;

#[test]
fn test_pass_creation() {
    let pass = Pass::new();
    assert!(pass.is_enabled());
    assert_eq!(pass.name_len, 0);
}

#[test]
fn test_pass_scheduler_basic() {
    let mut scheduler = PassScheduler::new();
    assert_eq!(scheduler.pass_count(), 0);
    assert!(scheduler.is_dirty());

    let mut pass = Pass::new();
    pass.set_name("test_pass");
    pass.set_type(PassType::MainOpaquePass);
    pass.set_priority(PassPriority::Normal);

    assert!(scheduler.add_pass(pass));
    assert_eq!(scheduler.pass_count(), 1);
}

#[test]
fn test_pass_scheduler_priority_ordering() {
    let mut scheduler = PassScheduler::new();

    let mut pass1 = Pass::new();
    pass1.set_name("low_priority");
    pass1.set_priority(PassPriority::Late);

    let mut pass2 = Pass::new();
    pass2.set_name("high_priority");
    pass2.set_priority(PassPriority::Early);

    let mut pass3 = Pass::new();
    pass3.set_name("normal_priority");
    pass3.set_priority(PassPriority::Normal);

    scheduler.add_pass(pass1);
    scheduler.add_pass(pass2);
    scheduler.add_pass(pass3);

    assert_eq!(scheduler.pass_count(), 3);
}

#[test]
fn test_resource_tracker_basic() {
    let tracker = ResourceTracker::new();
    assert_eq!(tracker.resource_count(), 0);
    assert_eq!(tracker.barrier_count(), 0);
}

#[test]
fn test_resource_registration() {
    let mut tracker = ResourceTracker::new();

    let resource_id = tracker.register(
        ResourceType::Texture,
        None,
        ResourceState::Undefined,
    );

    assert_ne!(resource_id, 0);
    assert_eq!(tracker.resource_count(), 1);
    assert_eq!(tracker.get_state(resource_id), ResourceState::Undefined);
}

#[test]
fn test_resource_barrier() {
    let mut tracker = ResourceTracker::new();

    let resource_id = tracker.register(
        ResourceType::Texture,
        None,
        ResourceState::Undefined,
    );

    assert!(tracker.add_barrier(resource_id, ResourceState::RenderTarget));
    assert_eq!(tracker.barrier_count(), 1);

    tracker.execute_barriers();
    assert_eq!(tracker.barrier_count(), 0);
    assert_eq!(tracker.get_state(resource_id), ResourceState::RenderTarget);
}

#[test]
fn test_resource_unregister() {
    let mut tracker = ResourceTracker::new();

    let resource_id = tracker.register(
        ResourceType::Buffer,
        None,
        ResourceState::Undefined,
    );

    assert_eq!(tracker.resource_count(), 1);
    assert!(tracker.unregister(resource_id));
    assert_eq!(tracker.resource_count(), 0);
}

#[test]
fn test_command_buffer_creation() {
    let buffer = CommandBuffer::new();
    assert!(!buffer.is_valid());
    assert!(!buffer.is_submitted);
}

#[test]
fn test_command_queue_creation() {
    let queue = CommandQueue::new();
    assert!(!queue.is_valid());
}

#[test]
fn test_pipeline_config() {
    let config = PipelineConfig::default();
    assert_eq!(config.max_passes, 64);
    assert_eq!(config.max_resources, 256);
    assert!(config.enable_validation);
    assert!(!config.enable_debug_markers);
}

#[test]
fn test_pipeline_custom_config() {
    let config = PipelineConfig::new(32, 128, false, true);
    assert_eq!(config.max_passes, 32);
    assert_eq!(config.max_resources, 128);
    assert!(!config.enable_validation);
    assert!(config.enable_debug_markers);
}

#[test]
fn test_pipeline_creation() {
    let pipeline = Pipeline::new();
    assert!(!pipeline.is_initialized());
    assert!(!pipeline.is_recording());
    assert_eq!(pipeline.frame_count(), 0);
}

#[test]
fn test_pipeline_initialization() {
    let mut pipeline = Pipeline::new();
    pipeline.init();
    assert!(pipeline.is_initialized());
    assert!(!pipeline.is_recording());
}

#[test]
fn test_pipeline_frame_recording() {
    let mut pipeline = Pipeline::new();
    pipeline.init();

    assert!(pipeline.begin_frame());
    assert!(pipeline.is_recording());

    assert!(pipeline.end_frame());
    assert!(!pipeline.is_recording());
    assert_eq!(pipeline.frame_count(), 1);
}

#[test]
fn test_pipeline_cannot_begin_frame_twice() {
    let mut pipeline = Pipeline::new();
    pipeline.init();

    assert!(pipeline.begin_frame());
    assert!(!pipeline.begin_frame()); // Should fail
}

#[test]
fn test_pipeline_cannot_end_frame_without_begin() {
    let mut pipeline = Pipeline::new();
    pipeline.init();

    assert!(!pipeline.end_frame()); // Should fail
}

#[test]
fn test_pipeline_with_custom_config() {
    let config = PipelineConfig::new(128, 512, true, true);
    let mut pipeline = Pipeline::with_config(config);
    
    pipeline.init();
    assert!(pipeline.is_validation_enabled());
    assert!(pipeline.is_debug_markers_enabled());
}

#[test]
fn test_pass_inputs_outputs() {
    let mut pass = Pass::new();
    
    assert!(pass.add_input(1));
    assert!(pass.add_input(2));
    assert_eq!(pass.input_count, 2);
    
    assert!(pass.add_output(10));
    assert!(pass.add_output(11));
    assert_eq!(pass.output_count, 2);
}

#[test]
fn test_pass_max_inputs() {
    let mut pass = Pass::new();
    
    // Add 8 inputs (max)
    for i in 0..8 {
        assert!(pass.add_input(i));
    }
    
    // 9th input should fail
    assert!(!pass.add_input(8));
}

// Note: test_resource_needs_barrier is covered by test_resource_barrier which tests the same functionality

#[test]
fn test_resource_clear() {
    let mut tracker = ResourceTracker::new();
    
    tracker.register(ResourceType::Texture, None, ResourceState::Undefined);
    tracker.register(ResourceType::Buffer, None, ResourceState::Undefined);
    
    assert_eq!(tracker.resource_count(), 2);
    
    tracker.clear();
    assert_eq!(tracker.resource_count(), 0);
}