//! Raw Pointer Benchmark - Maximum performance without Mut wrapper overhead
//! This benchmark directly accesses component data via raw pointers to measure
//! the theoretical performance ceiling of AutoZig ECS.

use autozig_ecs::prelude::*;
use autozig_ecs::storage::table::TableOpaque;
use rand::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
#[repr(C)]
struct Position { x: f32, y: f32 }

#[derive(Component, Clone, Copy, Debug)]
#[repr(C)]
struct Velocity { x: f32, y: f32 }

const ENTITY_COUNT: usize = 100_000;
const FRAMES: usize = 1000;

#[no_mangle]
pub extern "C" fn __zig_probe_stack() {}

// FFI declarations for direct table access
extern "C" {
    fn world_get_table_for_archetype(world_ptr: *mut u8, archetype_id: u32) -> *mut TableOpaque;
    fn table_entity_count(table: *const TableOpaque) -> usize;
    fn table_get_column_data_ptr(table: *const TableOpaque, component_id: u32) -> *mut u8;
}

fn main() {
    let mut app = App::new();
    
    // Register components
    app.world_mut().register_component::<Position>();
    app.world_mut().register_component::<Velocity>();

    println!("Spawning {} entities...", ENTITY_COUNT);
    setup(app.world_mut());
    app.world_mut().update_archetypes();

    let pos_id = app.world_mut().component_id::<Position>().expect("Position not registered");
    let vel_id = app.world_mut().component_id::<Velocity>().expect("Velocity not registered");
    
    // Get world inner pointer once
    let world_ptr = app.world_mut().as_raw_ptr();

    println!("Benchmarking AutoZig ECS (RAW POINTERS) for {} frames...", FRAMES);

    let start = std::time::Instant::now();
    for _ in 0..FRAMES {
        raw_movement_system(app.world_mut(), world_ptr, pos_id, vel_id);
    }
    let duration = start.elapsed();

    println!("AutoZig ECS RAW Result:");
    println!("  Total Time: {:.2?}", duration);
    println!("  Avg Frame:  {:.2?}", duration / FRAMES as u32);
    println!("  Entities:   {}", ENTITY_COUNT);
}

fn setup(world: &mut World) {
    let mut rng = rand::thread_rng();
    for _ in 0..ENTITY_COUNT {
        world.spawn((
            Position { x: rng.gen(), y: rng.gen() },
            Velocity { x: rng.gen(), y: rng.gen() }
        ));
    }
}

/// Raw movement system - bypasses all Bevy abstractions for maximum speed
fn raw_movement_system(world: &World, world_ptr: *mut u8, pos_id: ComponentId, vel_id: ComponentId) {
    let dt = 0.016f32;
    
    // Get archetypes from world
    let archetypes = world.archetypes();
    
    for archetype in archetypes.iter() {
        // Check if archetype has both Position and Velocity
        if !archetype.components().contains(&pos_id) || !archetype.components().contains(&vel_id) {
            continue;
        }
        
        // Get table for this archetype via FFI
        let table_ptr = unsafe { world_get_table_for_archetype(world_ptr, archetype.id().0) };
        if table_ptr.is_null() {
            continue;
        }
        
        let count = unsafe { table_entity_count(table_ptr) };
        if count == 0 { continue; }
        
        // Get raw pointers to component data
        let pos_ptr = unsafe { table_get_column_data_ptr(table_ptr, pos_id.index() as u32) };
        let vel_ptr = unsafe { table_get_column_data_ptr(table_ptr, vel_id.index() as u32) };
        
        if !pos_ptr.is_null() && !vel_ptr.is_null() {
            // CRITICAL: Direct pointer iteration - no Mut, no set_changed, no wrapper
            let positions = pos_ptr as *mut Position;
            let velocities = vel_ptr as *const Velocity;
            
            for i in 0..count {
                unsafe {
                    let pos = &mut *positions.add(i);
                    let vel = &*velocities.add(i);
                    pos.x += vel.x * dt;
                    pos.y += vel.y * dt;
                }
            }
        }
    }
}
