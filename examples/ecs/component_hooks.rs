use autozig_ecs::{
    prelude::*,
    component::{hooks::HookContext},
    event::Events,
    resource::Resource,
};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

// Define KeyCode locally since we don't depend on autozig-input
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum KeyCode {
    KeyA,
    KeyB,
    KeyC,
}

#[derive(Component, Debug, Clone, Copy)]
struct MyComponent(KeyCode);

#[derive(Default, Debug)]
struct MyComponentIndex(HashMap<KeyCode, Entity>);

impl Resource for MyComponentIndex {}

impl Deref for MyComponentIndex {
    type Target = HashMap<KeyCode, Entity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MyComponentIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Resource)]
struct MyMessage;

#[derive(Resource, Default)]
struct MockInput(Vec<KeyCode>);

fn register_hooks(world: &mut World) {
    // In order to register component hooks the component must:
    // - not be currently in use by any entities in the world
    // - not already have a hook of that kind registered
    // This is to prevent overriding hooks defined in plugins and other crates as well as keeping things fast
    world
        .register_component_hooks::<MyComponent>()
        // There are 4 component lifecycle hooks: `on_add`, `on_insert`, `on_replace` and `on_remove`
        // A hook has 2 arguments:
        // - a `DeferredWorld`, this allows access to resource and component data as well as `Commands`
        // - a `HookContext`, this provides access to the following contextual information:
        //   - the entity that triggered the hook
        //   - the component id of the triggering component, this is mostly used for dynamic components
        //   - the location of the code that caused the hook to trigger
        //
        // `on_add` will trigger when a component is inserted onto an entity without it
        .on_add(
            |mut world,
             HookContext {
                 entity,
                 component_id,
                 caller,
                 ..
             }| {
                // Be safe with unwrap or use get
                let value_opt = world.as_world().get::<MyComponent>(entity).map(|c| c.0);
                if let Some(value) = value_opt {
                    println!(
                        "{component_id:?} added to {entity:?} with value {value:?}{}",
                        if let Some(loc) = caller { format!("due to {loc}") } else { "".to_string() }
                    );
                    // Or access resources
                    world.resource_mut::<MyComponentIndex>().insert(value, entity);
                    
                    // Or send messages
                    world.resource_mut::<Events<MyMessage>>().send(MyMessage);
                }
            },
        )
        // `on_insert` will trigger when a component is inserted onto an entity,
        // regardless of whether or not it already had it and after `on_add` if it ran
        .on_insert(|mut world, _| {
            println!("Current Index: {:?}", world.resource_mut::<MyComponentIndex>().0);
        })
        // `on_replace` will trigger when a component is inserted onto an entity that already had it,
        // and runs before the value is replaced.
        // Also triggers when a component is removed from an entity, and runs before `on_remove`
        .on_replace(|mut world, context| {
             let value_opt = world.as_world().get::<MyComponent>(context.entity).map(|c| c.0);
             if let Some(value) = value_opt {
                 world.resource_mut::<MyComponentIndex>().remove(&value);
             }
        })
        // `on_remove` will trigger when a component is removed from an entity,
        // since it runs before the component is removed you can still access the component data
        .on_remove(
            |mut world,
             HookContext {
                 entity,
                 component_id,
                 caller,
                 ..
             }| {
                if let Some(comp_ref) = world.as_world().get::<MyComponent>(entity) {
                    let value = comp_ref.0;
                    println!(
                        "{component_id:?} removed from {entity:?} with value {value:?}{}",
                        if let Some(loc) = caller { format!("due to {loc}") } else { "".to_string() }
                    );
                    
                    world.resource_mut::<MyComponentIndex>().remove(&value);
                }

                // Despawn to avoid zombie entities
                // println!("Triggering despawn for {:?}", entity);
                world.commands().entity(entity).despawn();
            },
        );
}

fn trigger_hooks(
    mut commands: Commands,
    index: Res<MyComponentIndex>,
) {
    let key = KeyCode::KeyA;
    if index.is_empty() {
        println!("Spawning entity with {:?}", key);
        commands.spawn(MyComponent(key));
    } else {
        println!("Removing component from existing entities");
        for (_k, entity) in index.iter() {
             commands.entity(*entity).remove::<MyComponent>();
        }
    }
}

fn main() {
    let mut app = App::new();
    
    app.init_resource::<MyComponentIndex>();
    app.init_resource::<Events<MyMessage>>();
    app.init_resource::<MockInput>();
    
    register_hooks(app.world_mut());
    
    // Use proper explicit wrapper syntax for SystemParamFunction inference
    app.add_systems(
        Update, 
        IntoSystem::<((), Commands<'static>, Res<'static, MyComponentIndex>)>::into_system(trigger_hooks)
    );
    
    println!("Starting Component Hooks Example...");
    
    // Manual loop
    for _ in 0..5 {
        app.update();
        std::thread::sleep(std::time::Duration::from_secs_f32(0.5));
    }
}
