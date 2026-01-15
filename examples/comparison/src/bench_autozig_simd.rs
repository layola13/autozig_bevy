//! SIMD Benchmark - Tests pure Zig SIMD iteration performance
//! Uses the movement_update_simd8 kernel from simd_iter.zig

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

// FFI declarations for SIMD kernel and table access
extern "C" {
    fn world_get_table_for_archetype(world_ptr: *mut u8, archetype_id: u32) -> *mut TableOpaque;
    fn table_entity_count(table: *const TableOpaque) -> usize;
    fn table_get_column_data_ptr(table: *const TableOpaque, component_id: u32) -> *mut u8;
    
    // SIMD kernel from simd_iter.zig
    fn movement_update_simd8(
        positions: *mut f32,
        velocities: *const f32,
        count: usize,
        dt: f32,
    );
    
    fn movement_update_simd8_prefetch(
        positions: *mut f32,
        velocities: *const f32,
        count: usize,
        dt: f32,
    );
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
    
    let world_ptr = app.world_mut().as_raw_ptr();

    // Warm up
    for _ in 0..10 {
        simd_movement_system(app.world_mut(), world_ptr, pos_id, vel_id, false);
    }

    println!("Benchmarking AutoZig ECS (SIMD) for {} frames...", FRAMES);

    // Benchmark without prefetch
    let start = std::time::Instant::now();
    for _ in 0..FRAMES {
        simd_movement_system(app.world_mut(), world_ptr, pos_id, vel_id, false);
    }
    let duration = start.elapsed();

    println!("AutoZig ECS SIMD Result (no prefetch):");
    println!("  Total Time: {:.2?}", duration);
    println!("  Avg Frame:  {:.2?}", duration / FRAMES as u32);
    println!("  Entities:   {}", ENTITY_COUNT);

    // Benchmark with prefetch
    let start = std::time::Instant::now();
    for _ in 0..FRAMES {
        simd_movement_system(app.world_mut(), world_ptr, pos_id, vel_id, true);
    }
    let duration = start.elapsed();

    println!("\nAutoZig ECS SIMD Result (with prefetch):");
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

/// SIMD movement system - uses pure Zig SIMD kernel
fn simd_movement_system(world: &World, world_ptr: *mut u8, pos_id: ComponentId, vel_id: ComponentId, use_prefetch: bool) {
    let dt = 0.016f32;
    let archetypes = world.archetypes();
    
    for archetype in archetypes.iter() {
        if !archetype.components().contains(&pos_id) || !archetype.components().contains(&vel_id) {
            continue;
        }
        
        let table_ptr = unsafe { world_get_table_for_archetype(world_ptr, archetype.id().0) };
        if table_ptr.is_null() { continue; }
        
        let count = unsafe { table_entity_count(table_ptr) };
        if count == 0 { continue; }
        
        let pos_ptr = unsafe { table_get_column_data_ptr(table_ptr, pos_id.index() as u32) };
        let vel_ptr = unsafe { table_get_column_data_ptr(table_ptr, vel_id.index() as u32) };
        
        if !pos_ptr.is_null() && !vel_ptr.is_null() {
            unsafe {
                if use_prefetch {
                    movement_update_simd8_prefetch(
                        pos_ptr as *mut f32,
                        vel_ptr as *const f32,
                        count,
                        dt,
                    );
                } else {
                    movement_update_simd8(
                        pos_ptr as *mut f32,
                        vel_ptr as *const f32,
                        count,
                        dt,
                    );
                }
            }
        }
    }
}
