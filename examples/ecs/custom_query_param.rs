use autozig_ecs::prelude::*;

#[derive(Component, Debug)]
pub struct ComponentA(pub usize);

#[derive(Component, Debug)]
pub struct ComponentB(pub usize);

#[derive(Component, Debug)]
pub struct ComponentC(pub usize);

// Define a custom query parameter using the QueryData derive macro
#[derive(QueryData)]
#[query_data(mutable)]
pub struct CustomQuery {
    pub a: &'static ComponentA,
    pub b: &'static mut ComponentB,
    // Optional component
    pub c: Option<&'static ComponentC>,
    // Entity is also valid
    pub entity: Entity,
}

fn main() {
    let mut app = App::new();
    
    use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};
    
    // Explicitly typed systems to aid inference
    let setup_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>)>, _> = setup.into_system();
    let print_entities_sys: ParamFunctionSystem<FunctionMarker<((), Query<'static, Entity>)>, _> = print_all_entities.into_system();
    let print_custom_sys: ParamFunctionSystem<FunctionMarker<((), Query<'static, CustomQuery>)>, _> = print_custom_query.into_system();

    app.add_systems(Startup, setup_sys);
    app.add_systems(Update, (print_entities_sys, print_custom_sys).chain());
    
    app.run();
}

fn print_all_entities(query: Query<Entity>) {
    for entity in query.iter() {
        println!("All Entities: {:?}", entity);
    }
}

fn setup(mut commands: Commands) {
    // Entity with A and B
    commands.spawn((ComponentA(10), ComponentB(20)));
    
    // Entity with A, B, and C
    commands.spawn((ComponentA(30), ComponentB(40), ComponentC(50)));
    
    // Entity with only A (should not match)
    commands.spawn(ComponentA(60));
}

fn print_custom_query(query: Query<CustomQuery>) {
    println!("Iterating CustomQuery...");
    for mut item in query.iter() {
        println!(
            "Entity {:?}: A={:?}, B={:?}, C={:?}", 
            item.entity, item.a.0, item.b.0, item.c.map(|c| c.0)
        );
        
        // Modify mutable component
        item.b.0 += 1;
    }
}
