use autozig_app::{App, Plugin, MinimalPlugins};
use autozig_ecs::prelude::*;
use autozig_camera::{Camera3dBundle, CameraPlugin};
use autozig_pbr::{PbrBundle};
use autozig_light::{PointLightBundle, LightPlugin};
use autozig_scene::{SceneBundle, ScenePlugin};
use autozig_asset::AssetServer;
use autozig_transform::TransformPlugin;
use autozig_window::WindowPlugin;
use autozig_winit::WinitPlugin;
use autozig_render::RenderPlugin;
use autozig_mesh::{Mesh, MeshPrimitives};
use autozig_pbr::{StandardMaterial};
use autozig_asset::Assets;
use autozig_math::Vec3;
use autozig_ecs::resource::ResMut; // Fix: Import from resource module
use autozig_color::Color;
use autozig_transform::Transform; // Fix: Import Transform directly from crate root or prelude

fn main() {
    println!("Starting Basic 3D Scene Example...");
    let mut app = App::new();
    
    // Add base plugins
    app.add_plugins(MinimalPlugins);
    app.add_plugin(TransformPlugin::default());
    app.add_plugin(WindowPlugin::default());
    app.add_plugin(WinitPlugin::default());
    app.add_plugin(RenderPlugin::default());
    
    // Add 3D plugins
    app.add_plugin(CameraPlugin::default());
    app.add_plugin(LightPlugin::default());
    app.add_plugin(ScenePlugin::default());
    
    // Register AssetServer manually (since AssetPlugin is not in use or stubbed)
    app.insert_resource(AssetServer::default());
    
    // Initialize Asset Storage
    // In a real app, AssetPlugin would do this.
    app.insert_resource(Assets::<Mesh>::new());
    app.insert_resource(Assets::<StandardMaterial>::new());

    app.add_systems(Startup, setup);
    app.add_systems(Update, debug_camera_matrix);
    
    // Use return_after_run to avoid blocking in CI/tests if needed, but for manual verification blocking is fine.
    // But for automation, running once is good.
    // However, minimal plugins might lack loop? No, default storage.
    println!("Building implementations and systems...");
    // Run the app (starts Winit event loop and creates window)
    app.run();
    println!("App run finished.");
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    println!("Spawned Camera3dBundle");
    // Continued setup...

    // Spawn Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    println!("Spawned PointLightBundle");

    // Spawn Cube (PbrBundle)
    let mesh_handle = meshes.add(MeshPrimitives::cube(1.0));
    let material_handle = materials.add(StandardMaterial::from(Color::rgb(0.8, 0.7, 0.6)));

    commands.spawn(PbrBundle {
        mesh: autozig_mesh::Mesh3d(mesh_handle),
        material: autozig_pbr::MeshMaterial3d(material_handle),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
    println!("Spawned PbrBundle (Cube)");

    // Spawn Scene
    // commands.spawn(SceneBundle::default());
    // println!("Spawned SceneBundle");
    
    println!("Setup complete!");
}

fn debug_camera_matrix(query: autozig_ecs::prelude::Query<&autozig_camera::Camera3d>) {
    for camera in query.iter() {
        // println!("Camera View-Proj Matrix: {:?}", camera.view_projection_matrix);
        // Reduce spam - print only first element to check non-zero
        if camera.view_projection_matrix[0] != 0.0 || camera.view_projection_matrix[15] != 0.0 {
             // println!("Valid Matrix Detected!"); 
        } else {
             println!("WARNING: Camera View-Proj Matrix is ALL ZEROS or Invalid!");
             println!("Debug Info:");
             println!("  FOV: {}", camera.projection.fov);
             println!("  Aspect: {}", camera.projection.aspect_ratio);
             println!("  Proj[0] (f/aspect): {}", camera.projection_matrix[0]);
             println!("  View[0] (Right.x): {}", camera.view_matrix[0]);
             println!("  View[15] (1.0): {}", camera.view_matrix[15]);
        }
    }
}
