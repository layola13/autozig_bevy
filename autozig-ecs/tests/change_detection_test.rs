use autozig_ecs::prelude::*;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
struct Velocity {
    x: f32,
    y: f32,
}

#[test]
fn test_added_filter() {
    let mut world = World::new();
    world.register_component::<Position>();
    
    // 1. Spawn entity
    let _entity = world.spawn((Position { x: 1.0, y: 2.0 },));
    
    // 2. Query for Added<Position>
    let mut query = world.query_filtered::<&Position, Added<Position>>();
    let results: Vec<&Position> = query.iter::<&Position, Added<Position>>(&world).collect();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].x, 1.0);
    
    // 3. Clear trackers
    world.clear_trackers();
    
    // 4. Query for Added<Position> should be empty
    let results: Vec<&Position> = query.iter::<&Position, Added<Position>>(&world).collect();
    assert_eq!(results.len(), 0);
}

#[test]
fn test_changed_filter() {
    let mut world = World::new();
    world.register_component::<Position>();
    
    // 1. Spawn entity
    let _entity = world.spawn((Position { x: 1.0, y: 2.0 },));
    
    // 2. Query for Changed<Position> (Added implies Changed)
    let mut query = world.query_filtered::<&Position, Changed<Position>>();
    let results: Vec<&Position> = query.iter::<&Position, Changed<Position>>(&world).collect();
    assert_eq!(results.len(), 1);
    
    // 3. Clear trackers
    world.clear_trackers();
    
    // 4. Query for Changed<Position> should be empty
    let results: Vec<&Position> = query.iter::<&Position, Changed<Position>>(&world).collect();
    assert_eq!(results.len(), 0);
    
    // 5. Mutate component
    {
        let mut query_mut = world.query::<&mut Position>();
        for mut pos in query_mut.iter_mut::<&mut Position, ()>(&mut world) {
            pos.x += 1.0;
        }
    }
    
    // 6. Query for Changed<Position> should find it
    let results: Vec<&Position> = query.iter::<&Position, Changed<Position>>(&world).collect();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].x, 2.0);
    
    // 7. Clear trackers
    world.clear_trackers();
    
    // 8. Query for Changed<Position> should be empty again
    let results: Vec<&Position> = query.iter::<&Position, Changed<Position>>(&world).collect();
    assert_eq!(results.len(), 0);
}

#[test]
fn test_added_vs_changed() {
    let mut world = World::new();
    world.register_component::<Position>();
    
    // 1. Spawn entity
    world.spawn((Position { x: 1.0, y: 2.0 },));
    world.clear_trackers();
    
    // 2. Spawn ANOTHER entity
    world.spawn((Position { x: 3.0, y: 4.0 },));
    
    // 3. Mutate first entity
    {
        let mut query_mut = world.query::<&mut Position>();
        let mut it = query_mut.iter_mut::<&mut Position, ()>(&mut world);
        let mut first = it.next().unwrap();
        first.x += 10.0;
    }
    
    // 4. Check Added<Position>
    let mut query_added = world.query_filtered::<&Position, Added<Position>>();
    let added: Vec<&Position> = query_added.iter::<&Position, Added<Position>>(&world).collect();
    assert_eq!(added.len(), 1);
    assert_eq!(added[0].x, 3.0); // The newly spawned one
    
    // 5. Check Changed<Position>
    let mut query_changed = world.query_filtered::<&Position, Changed<Position>>();
    let changed: Vec<&Position> = query_changed.iter::<&Position, Changed<Position>>(&world).collect();
    assert_eq!(changed.len(), 2); // Both the added one and the mutated one
}

#[test]
fn test_or_filter() {
    let mut world = World::new();
    world.register_component::<Position>();
    world.register_component::<Velocity>();
    
    // 1. Spawn entities
    world.spawn((Position { x: 1.0, y: 2.0 },));
    world.spawn((Velocity { x: 1.0, y: 2.0 },));
    world.clear_trackers();
    
    // 2. Mutate Position only
    {
        let mut q = world.query::<&mut Position>();
        for mut p in q.iter_mut::<&mut Position, ()>(&mut world) { 
            p.x += 1.0; 
        }
    }
    
    // 3. Query for Changed<Position> OR Changed<Velocity>
    let mut query = world.query_filtered::<Entity, Or<(Changed<Position>, Changed<Velocity>)>>();
    let results: Vec<Entity> = query.iter::<Entity, Or<(Changed<Position>, Changed<Velocity>)>>(&world).collect();
    assert_eq!(results.len(), 1);
    
    // 4. Mutate Velocity too
    {
        let mut q = world.query::<&mut Velocity>();
        for mut v in q.iter_mut::<&mut Velocity, ()>(&mut world) { 
            v.x += 1.0; 
        }
    }
    
    let results: Vec<Entity> = query.iter::<Entity, Or<(Changed<Position>, Changed<Velocity>)>>(&world).collect();
    assert_eq!(results.len(), 2);
}
