use autozig_app::{App, MainScheduleOrder};
use autozig_window::WindowPlugin;
use autozig_render::RenderPlugin;
use autozig_camera::Camera3d;
use autozig_transform::{Transform, GlobalTransform};

// Need to access APP_PTR from autozig-render
use autozig_render::APP_PTR;

#[no_mangle]
pub extern "C" fn setup() {
    unsafe {
        if APP_PTR.is_null() { return; }
        
        // Spawn Camera
        let world_ptr = autozig_app::App::get_world_from_ptr(APP_PTR);
        let mut world = autozig_ecs::world::World::from_raw(world_ptr as *mut autozig_ecs::world::WorldOpaque);
        
        // Debug register here
        world.register_component::<Camera3d>();
        world.register_component::<Transform>();
        world.register_component::<GlobalTransform>();
        
        // Camera at (0, 0, 5) looking at (0, 0, 0)
        let mut camera = Camera3d::new(45.0f32.to_radians(), 1280.0/720.0);
        let eye = [0.0, 2.0, 5.0];
        let target = [0.0, 0.0, 0.0];
        let up = [0.0, 1.0, 0.0];
        camera.look_at(eye, target, up);

        world.spawn((
            camera,
            Transform::from_translation(eye),
            GlobalTransform::identity()
        ));
        
        println!("Camera spawned!");
        
        std::mem::forget(world);
    }
}

static mut ANGLE: f32 = 0.0;

#[no_mangle]
pub extern "C" fn rotate_camera() {
    unsafe {
        if APP_PTR.is_null() { return; }
        
        let world_ptr = autozig_app::App::get_world_from_ptr(APP_PTR);
        let mut world = autozig_ecs::world::World::from_raw(world_ptr as *mut autozig_ecs::world::WorldOpaque);
        world.update_archetypes();

        ANGLE += 0.01;
        let radius = 5.0;
        let x = radius * ANGLE.sin();
        let z = radius * ANGLE.cos();
        
        let mut query = world.query::<(&mut Camera3d, &mut Transform)>();
        for (mut camera, mut transform) in query.iter::<(&mut Camera3d, &mut Transform), ()>(&world) {
            transform.translation = [x, 2.0, z];
            
            // Re-calculate view matrix
            let eye = transform.translation;
            let target = [0.0, 0.0, 0.0];
            let up = [0.0, 1.0, 0.0];
            
            camera.look_at(eye, target, up);
        }
        
        // Ensure to forget world!
        std::mem::forget(world);
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugin(WindowPlugin::default());
    // Winit runner is needed for window loop!
    // autozig-render's RenderPlugin does NOT add winit runner?
    // autozig-winit provides WinitPlugin.
    // WindowPlugin does not install runner.
    // So we need WinitPlugin as well.
    app.add_plugin(autozig_winit::WinitPlugin::default());
    app.add_plugin(RenderPlugin);
    
    app.add_systems(MainScheduleOrder::Startup, setup);
    app.add_systems(MainScheduleOrder::Update, rotate_camera);
    
    // Explicitly register components
    app.world.register_component::<Camera3d>();
    app.world.register_component::<Transform>();
    app.world.register_component::<GlobalTransform>();
    
    app.run();
}
