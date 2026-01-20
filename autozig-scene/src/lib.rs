use autozig_app::{App, Plugin};
use autozig_asset::{Asset, Handle};
use autozig_ecs::prelude::Bundle;
use autozig_transform::{Transform, GlobalTransform};
use autozig_camera::{Visibility, InheritedVisibility, ViewVisibility};

#[derive(Debug, Clone)]
pub struct Scene {
    pub world: (), 
}

impl Asset for Scene {
    fn type_uuid() -> autozig_asset::Uuid {
        autozig_asset::Uuid::from_u128(0x717237e8c324483e8753235222216666)
    }
}

#[derive(Bundle, Clone, Default)]
pub struct SceneBundle {
    pub scene: Handle<Scene>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

#[derive(Default)]
pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, _app: &mut App) {
        // Placeholder for asset initialization
    }
    
    fn name(&self) -> &str {
        "ScenePlugin"
    }
}
