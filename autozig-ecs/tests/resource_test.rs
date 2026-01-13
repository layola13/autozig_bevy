use autozig_ecs::world::World;
use autozig_ecs::resource::{Resource, FromWorld};

#[derive(Default)]
struct TestResource {
    value: i32,
}

// impl Resource for TestResource {} - Blanket impl handles this

struct Counter(i32);

// impl Resource for Counter {} - Blanket impl handles this

impl FromWorld for Counter {
    fn from_world(_world: &mut World) -> Self {
        Self(100)
    }
}

struct DependentResource {
    double_counter: i32,
}

// impl Resource for DependentResource {} - Blanket impl handles this

impl FromWorld for DependentResource {
    fn from_world(world: &mut World) -> Self {
        // Need to ensure Counter exists before accessing it?
        // In Bevy, from_world can access existing resources.
        // If Counter is not present, this might panic if we use unwrap, or we should use logic.
        // For test, we assume Counter is init first or we init it here.
        // But world is passed as &mut World.
        // Accessing resource might require splitting borrows if we are not careful for internals,
        // but public API `resource()` takes `&self`.
        // `from_world` takes `&mut World`.
        // `world.resource()` requires `&World`. `&mut World` can be downgraded.
        let counter = world.resource::<Counter>();
        Self {
            double_counter: counter.0 * 2,
        }
    }
}

#[test]
fn test_insert_and_get_resource() {
    autozig_ecs::entity::init();

    let mut world = World::new();
    world.insert_resource(TestResource { value: 42 });

    let res = world.resource::<TestResource>();
    assert_eq!(res.value, 42);

    let mut res_mut = world.resource_mut::<TestResource>();
    res_mut.value += 1;
}

#[test]
fn test_init_resource() {
    autozig_ecs::entity::init();

    let mut world = World::new();
    
    // Init resource (should create with default value 100)
    world.init_resource::<Counter>();

    let counter = world.resource::<Counter>();
    assert_eq!(counter.0, 100);
}

#[test]
fn test_init_resource_dependency() {
    autozig_ecs::entity::init();

    let mut world = World::new();
    world.init_resource::<Counter>();
    
    // DependentResource requires Counter to exist
    world.init_resource::<DependentResource>();
    
    let dependent = world.resource::<DependentResource>();
    assert_eq!(dependent.double_counter, 200);
}

#[test]
fn test_remove_resource() {
    autozig_ecs::entity::init();

    let mut world = World::new();
    world.insert_resource(TestResource { value: 123 });
    
    assert!(world.contains_resource::<TestResource>());
    
    let removed = world.remove_resource::<TestResource>();
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().value, 123);
    
    assert!(!world.contains_resource::<TestResource>());
    
    // Remove again should return None
    let removed_again = world.remove_resource::<TestResource>();
    assert!(removed_again.is_none());
}
