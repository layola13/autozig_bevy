# AutoZig Bevy ECS API Restoration Plan

**Goal:** Achieve 1:1 public API parity with `bevy_ecs`.
**Current Status:** ~30% complete. ~1700 items missing.

## Strategy

The restoration will be executed in **6 batches**, prioritized by dependency order and criticality. Each batch focuses on a specific subsystem of the ECS.

### Batch 1: Core Fundamentals (Entities & World)
**Focus:** The bedrock of ECS. Entity lifecycle, ID management, and World storage.
- **Modules**: `entity/*`, `world/*`
- **Key Missing APIs**:
    - Entity allocation/sparsing (`Entities`, `AllocEntitiesIterator`)
    - Entity mapping/cloning (`EntityHashMap`, `EntityMapper`)
    - World internals (`UnsafeWorldCell`, `WorldId`)
    - Entity location cache (`EntityLocation`)
- **Estimated complexity**: High (unsafe code, core logic).

### Batch 2: Components & Bundles
**Focus:** Data definition and storage layout.
- **Modules**: `component/*`, `bundle/*`, `storage/*`
- **Key Missing APIs**:
    - Component lifecycle hooks (`hooks`, `ComponentHooks`)
    - Bundle traits & info (`DynamicBundle`, `BundleInfo`)
    - Storage types (`Table`, `SparseSet`, `ComponentSparseSet`)
    - Required components (`RequiredComponents`)
- **Estimated complexity**: Medium.

### Batch 3: Change Detection & Querying
**Focus:** Retrieving and filtering data efficiently.
- **Modules**: `query/*`, `change_detection/*`, `removal_detection/*`
- **Key Missing APIs**:
    - Advanced query filters (`Or`, `With`, `Without` complex combinations)
    - Change detection internals (`Ticks`, `ComponentTicks`)
    - Query fetching traits (`Fetch`, `WorldQuery`)
    - Query cursors and iterators.
- **Estimated complexity**: High (generics, lifetimes, unsafe fetch logic).

### Batch 4: Systems & Scheduling
**Focus:** Execution logic and ordering.
- **Modules**: `system/*`, `schedule/*`, `executor/*`
- **Key Missing APIs**:
    - System piping (`PipeSystem`, `AdapterSystem`)
    - Schedule conditions (`Condition`, `common_conditions`)
    - Executor types (`MultiThreadedExecutor`, `SingleThreadedExecutor`)
    - System param combinators (`ParamSet`)
- **Estimated complexity**: High (async, threading, complex traits).

### Batch 5: Events, Observers & Messaging
**Focus:** Inter-system communication.
- **Modules**: `event/*`, `observer/*`, `message/*`
- **Key Missing APIs**:
    - Trigger & Observer system (`Observer`, `Trigger`)
    - Event cursors (`EventCursor`)
    - Message passing internals (`MessageQueue`)
- **Estimated complexity**: Medium.

### Batch 6: Reflection & Hierarchy (Polish)
**Focus:** Editor support and scene graph.
- **Modules**: `reflect/*`, `hierarchy.rs`
- **Key Missing APIs**:
    - Reflection traits for ECS types (`ReflectComponent`, `ReflectResource`)
    - Hierarchy helpers (`Parent`, `Children` helpers)
    - Diagnostics and error handling types.
- **Estimated complexity**: Low to Medium.

## Execution Guidelines

1.  **Reference:** Use `bevy_ecs` source code as the source of truth.
2.  **Naming:** Keep names identical to Bevy.
3.  **Safety:** Verify unsafe blocks carefully; AutoZig may need to wrap them safely where possible or expose them as `unsafe`.
4.  **Testing:** Add unit tests for new types to verify layout/behavior matches Bevy.

## Tracking

Use the `ECS_API_GAP_ANALYSIS.md` document to check off items as they are implemented.
