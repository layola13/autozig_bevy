#!/bin/bash
set -e

# List of all ECS examples
examples=(
    "change_detection"
    "component_hooks"
    "custom_query_param"
    "startup_system"
    "system_param"
    "iter_combinations"
    "fixed_timestep"
    "removal_detection"
    "run_conditions"
    "system_piping"
    "hierarchy"
    "one_shot_systems"
    "error_handling"
    "dynamic"
    "fallible_params"
    "observers"
    "parallel_query"
    "generic_system"
    "ecs_guide"
    "custom_schedule"
    "state_scoped"
    "entity_disabling"
    "immutable_components"
    "system_closure"
    "observer_propagation"
    "nondeterministic_system_order"
    "system_stepping"
    "send_and_receive_messages"
    "hotpatching_systems"
)

echo "Compiling all examples..."
cargo build -p autozig-ecs-examples

echo ""
echo "============================================="
echo " Starting execution of ALL ECS examples"
echo "============================================="
echo ""

for example in "${examples[@]}"; do
    echo "---------------------------------------------"
    echo " Running example: $example"
    echo "---------------------------------------------"
    
    cargo run -p autozig-ecs-examples --bin "$example"
    
    echo ""
    read -p "Press [Enter] to continue to the next example, or Ctrl+C to stop..."
    echo ""
done

echo "All examples executed."
