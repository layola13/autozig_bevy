# AutoZig vs Bevy API 对比报告

> 自动生成于: 2026-01-09 11:02:45

## 模块汇总

| 模块 | AutoZig | Bevy | 匹配 | 完成度 |
|------|---------|------|------|--------|
| app | 8 | 60 | 4 | 6% |
| ecs | 38 | 501 | 24 | 4% |
| math | 65 | 195 | 34 | 17% |
| render | 22 | 312 | 8 | 2% |
| transform | 4 | 11 | 1 | 9% |
| mesh | 13 | 69 | 2 | 2% |
| pbr | 4 | 323 | 1 | 0% |
| light | 12 | 40 | 5 | 12% |
| color | 4 | 24 | 4 | 16% |
| input | 14 | 48 | 14 | 29% |
| window | 6 | 54 | 4 | 7% |
| time | 4 | 12 | 4 | 33% |
| state | 14 | 28 | 10 | 35% |
| asset | 12 | 158 | 8 | 5% |
| sprite | 7 | 16 | 2 | 12% |
| ui | 23 | 104 | 17 | 16% |
| text | 17 | 51 | 3 | 5% |
| reflect | 10 | 156 | 5 | 3% |
| tasks | 2 | 26 | 2 | 7% |
| ptr | 11 | 13 | 11 | 84% |
| utils | 10 | 6 | 1 | 16% |
| diagnostic | 11 | 15 | 7 | 46% |
| log | 1 | 1 | 0 | 0% |
| image | 10 | 46 | 2 | 4% |
| camera | 7 | 60 | 7 | 11% |
| **总计** | **329** | **2329** | **180** | **7%** |

---

## 详细 API 对比

### app

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `AccessibilityPlugin` | 🔴 缺失 |
| struct | `AnimationSystems` | 🔴 缺失 |
| struct | `App` | ✅ |
| struct | `AudioPlugins` | 🔴 缺失 |
| struct | `CapsuleCollisionPlugin` | 🔴 缺失 |
| struct | `First` | 🔴 缺失 |
| struct | `FixedFirst` | 🔴 缺失 |
| struct | `FixedLast` | 🔴 缺失 |
| struct | `FixedMain` | 🔴 缺失 |
| struct | `FixedMainScheduleOrder` | 🔴 缺失 |
| struct | `FixedPostUpdate` | 🔴 缺失 |
| struct | `FixedPreUpdate` | 🔴 缺失 |
| struct | `FixedUpdate` | 🔴 缺失 |
| struct | `Foo` | 🔴 缺失 |
| struct | `ForcePlugin` | 🔴 缺失 |
| struct | `HierarchyPropagatePlugin` | 🔴 缺失 |
| struct | `HotPatchPlugin` | 🔴 缺失 |
| struct | `Inherited` | 🔴 缺失 |
| struct | `InternalPlugin` | 🔴 缺失 |
| struct | `Last` | 🔴 缺失 |
| struct | `LogPlugin` | 🔴 缺失 |
| struct | `Main` | 🔴 缺失 |
| struct | `MainScheduleOrder` | 🔴 缺失 |
| struct | `MainSchedulePlugin` | 🔴 缺失 |
| struct | `NoopPluginGroup` | 🔴 缺失 |
| struct | `PanicHandlerPlugin` | 🔴 缺失 |
| struct | `PhysicsPlugins` | 🔴 缺失 |
| struct | `PluginGroupBuilder` | 🔴 缺失 |
| struct | `PluginGroupMarker` | 🔴 缺失 |
| struct | `PluginMarker` | 🔴 缺失 |
| struct | `PluginsTupleMarker` | 🔴 缺失 |
| struct | `PostStartup` | 🔴 缺失 |
| struct | `PostUpdate` | 🔴 缺失 |
| struct | `PreStartup` | 🔴 缺失 |
| struct | `PreUpdate` | 🔴 缺失 |
| struct | `Propagate` | 🔴 缺失 |
| struct | `PropagateOver` | 🔴 缺失 |
| struct | `PropagateSet` | 🔴 缺失 |
| struct | `PropagateStop` | 🔴 缺失 |
| struct | `RunFixedMainLoop` | 🔴 缺失 |
| struct | `ScheduleRunnerPlugin` | 🔴 缺失 |
| struct | `SpawnScene` | 🔴 缺失 |
| struct | `Startup` | 🔴 缺失 |
| struct | `SubApp` | ✅ |
| struct | `SubApps` | 🔴 缺失 |
| struct | `TaskPoolOptions` | 🔴 缺失 |
| struct | `TaskPoolPlugin` | 🔴 缺失 |
| struct | `TaskPoolThreadAssignmentPolicy` | 🔴 缺失 |
| struct | `TerminalCtrlCHandlerPlugin` | 🔴 缺失 |
| struct | `TickratePlugin` | 🔴 缺失 |
| struct | `Update` | 🔴 缺失 |
| struct | `VelocityPlugin` | 🔴 缺失 |
| struct | `WebCompatibilityPlugin` | 🔴 缺失 |
| enum | `AppExit` | ✅ |
| enum | `PluginsState` | 🔴 缺失 |
| enum | `RunFixedMainLoopSystems` | 🔴 缺失 |
| enum | `RunMode` | 🔴 缺失 |
| trait | `Plugin` | ✅ |
| trait | `PluginGroup` | 🔴 缺失 |
| trait | `Plugins` | 🔴 缺失 |

### ecs

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `A` | ✅ |
| struct | `Access` | 🔴 缺失 |
| struct | `AccessConflictError` | 🔴 缺失 |
| struct | `AdapterSystem` | 🔴 缺失 |
| struct | `Add` | 🔴 缺失 |
| struct | `Added` | 🔴 缺失 |
| struct | `AddedFetch` | 🔴 缺失 |
| struct | `Alarm` | 🔴 缺失 |
| struct | `AllocEntitiesIterator` | 🔴 缺失 |
| struct | `Allow` | 🔴 缺失 |
| struct | `AmbiguousSystemConflictsWarning` | 🔴 缺失 |
| struct | `AncestorIter` | 🔴 缺失 |
| struct | `AndMarker` | 🔴 缺失 |
| struct | `AnonymousSet` | 🔴 缺失 |
| struct | `AnyOf` | 🔴 缺失 |
| struct | `AppFunctionRegistry` | 🔴 缺失 |
| struct | `AppTypeRegistry` | 🔴 缺失 |
| struct | `ApplyDeferred` | 🔴 缺失 |
| struct | `Archetype` | 🔴 缺失 |
| struct | `ArchetypeEntity` | 🔴 缺失 |
| struct | `ArchetypeGeneration` | 🔴 缺失 |
| struct | `ArchetypeId` | 🔴 缺失 |
| struct | `ArchetypeRecord` | 🔴 缺失 |
| struct | `ArchetypeRow` | 🔴 缺失 |
| struct | `Archetypes` | 🔴 缺失 |
| struct | `AutoInsertApplyDeferredPass` | 🔴 缺失 |
| struct | `B` | ✅ |
| struct | `BatchingStrategy` | 🔴 缺失 |
| struct | `BevyError` | 🔴 缺失 |
| struct | `BundleId` | 🔴 缺失 |
| struct | `BundleInfo` | 🔴 缺失 |
| struct | `Bundles` | 🔴 缺失 |
| struct | `C` | ✅ |
| struct | `CachedComponentObservers` | 🔴 缺失 |
| struct | `CachedObservers` | 🔴 缺失 |
| struct | `CachedSystemId` | 🔴 缺失 |
| struct | `Changed` | 🔴 缺失 |
| struct | `ChangedFetch` | 🔴 缺失 |
| struct | `CheckChangeTicks` | 🔴 缺失 |
| struct | `ChildOf` | 🔴 缺失 |
| struct | `Children` | 🔴 缺失 |
| struct | `Client` | 🔴 缺失 |
| struct | `ClientFetch` | 🔴 缺失 |
| struct | `Collide` | 🔴 缺失 |
| struct | `Column` | 🔴 缺失 |
| struct | `CombinatorSystem` | 🔴 缺失 |
| struct | `CommandQueue` | 🔴 缺失 |
| struct | `Commands` | ✅ |
| struct | `CompactNodeIdAndDirection` | 🔴 缺失 |
| struct | `CompactNodeIdPair` | 🔴 缺失 |
| struct | `ComponentCloneCtx` | 🔴 缺失 |
| struct | `ComponentDescriptor` | 🔴 缺失 |
| struct | `ComponentHooks` | 🔴 缺失 |
| struct | `ComponentId` | 🔴 缺失 |
| struct | `ComponentIdFor` | 🔴 缺失 |
| struct | `ComponentIds` | 🔴 缺失 |
| struct | `ComponentInfo` | 🔴 缺失 |
| struct | `ComponentRelationshipAccessor` | 🔴 缺失 |
| struct | `ComponentSparseSet` | 🔴 缺失 |
| struct | `ComponentTickCells` | 🔴 缺失 |
| struct | `ComponentTicks` | 🔴 缺失 |
| struct | `Components` | 🔴 缺失 |
| struct | `ComponentsQueuedRegistrator` | 🔴 缺失 |
| struct | `ComponentsRegistrator` | 🔴 缺失 |
| struct | `ConditionWithAccess` | 🔴 缺失 |
| struct | `ConflictingSystems` | 🔴 缺失 |
| struct | `ConstGenericParam` | 🔴 缺失 |
| struct | `Criminal` | 🔴 缺失 |
| struct | `CustomParam` | 🔴 缺失 |
| struct | `D` | ✅ |
| struct | `Dag` | 🔴 缺失 |
| struct | `DagAnalysis` | 🔴 缺失 |
| struct | `DagCrossDependencyError` | 🔴 缺失 |
| struct | `DagGroups` | 🔴 缺失 |
| struct | `DagOverlappingGroupError` | 🔴 缺失 |
| struct | `DagRedundancyError` | 🔴 缺失 |
| struct | `DefaultCloneBehaviorSpecialization` | 🔴 缺失 |
| struct | `DefaultErrorHandler` | 🔴 缺失 |
| struct | `DefaultQueryFilters` | 🔴 缺失 |
| struct | `Deferred` | 🔴 缺失 |
| struct | `DeferredWorld` | 🔴 缺失 |
| struct | `DerivedNonReleaseMutable` | 🔴 缺失 |
| struct | `DerivedNonReleaseRead` | 🔴 缺失 |
| struct | `DerivedReleaseMutable` | 🔴 缺失 |
| struct | `DerivedReleaseRead` | 🔴 缺失 |
| struct | `DescendantDepthFirstIter` | 🔴 缺失 |
| struct | `DescendantIter` | 🔴 缺失 |
| struct | `Despawn` | 🔴 缺失 |
| struct | `Disabled` | 🔴 缺失 |
| struct | `Drain` | 🔴 缺失 |
| struct | `DynParamBuilder` | 🔴 缺失 |
| struct | `DynSystemParam` | 🔴 缺失 |
| struct | `DynSystemParamState` | 🔴 缺失 |
| struct | `Edges` | 🔴 缺失 |
| struct | `EncapsulatedParam` | 🔴 缺失 |
| struct | `Entities` | 🔴 缺失 |
| struct | `Entity` | ✅ |
| struct | `EntityAllocator` | 🔴 缺失 |
| struct | `EntityCloner` | 🔴 缺失 |
| struct | `EntityClonerBuilder` | 🔴 缺失 |
| struct | ... | (更多省略) |
| enum | `A` | 🔴 缺失 |
| enum | `AccessConflicts` | 🔴 缺失 |
| enum | `Chain` | 🔴 缺失 |
| enum | `CoffeeMachineState` | 🔴 缺失 |
| enum | `ComponentAccessKind` | 🔴 缺失 |
| enum | `ComponentCloneBehavior` | 🔴 缺失 |
| enum | `ComponentEntry` | 🔴 缺失 |
| enum | `DiGraphToposortError` | 🔴 缺失 |
| enum | `Direction` | 🔴 缺失 |
| enum | `EcsAccessLevel` | 🔴 缺失 |
| enum | `EcsAccessType` | 🔴 缺失 |
| enum | `EnemyState` | 🔴 缺失 |
| enum | `EntityClonerFilter` | 🔴 缺失 |
| enum | `EntityCommandError` | 🔴 缺失 |
| enum | `EntityComponentError` | 🔴 缺失 |
| enum | `EntityMutableFetchError` | 🔴 缺失 |
| enum | `EntityNotSpawnedError` | 🔴 缺失 |
| enum | `ErrorContext` | 🔴 缺失 |
| enum | `ExecutorKind` | 🔴 缺失 |
| enum | `FilterableId` | 🔴 缺失 |
| enum | `GetComponentReflectError` | 🔴 缺失 |
| enum | `GetEntityMutByIdError` | 🔴 缺失 |
| enum | `InsertMode` | 🔴 缺失 |
| enum | `LogLevel` | 🔴 缺失 |
| enum | `NodeId` | 🔴 缺失 |
| enum | `PlayerState` | 🔴 缺失 |
| enum | `QueryAccessError` | 🔴 缺失 |
| enum | `QueryEntityError` | 🔴 缺失 |
| enum | `QuerySingleError` | 🔴 缺失 |
| enum | `RegisteredSystemError` | 🔴 缺失 |
| enum | `RelationshipAccessor` | 🔴 缺失 |
| enum | `RelationshipHookMode` | 🔴 缺失 |
| enum | `RequiredComponentsError` | 🔴 缺失 |
| enum | `ResourceAccessLevel` | 🔴 缺失 |
| enum | `ResourceFetchError` | 🔴 缺失 |
| enum | `RunSystemError` | 🔴 缺失 |
| enum | `ScheduleBuildError` | 🔴 缺失 |
| enum | `ScheduleBuildWarning` | 🔴 缺失 |
| enum | `ScheduleCleanupPolicy` | 🔴 缺失 |
| enum | `ScheduleConfigs` | 🔴 缺失 |
| enum | `ScheduleError` | 🔴 缺失 |
| enum | `ShouldUpdateMessages` | 🔴 缺失 |
| enum | `SoilState` | 🔴 缺失 |
| enum | `SpawnError` | 🔴 缺失 |
| enum | `StorageType` | 🔴 缺失 |
| enum | `TeaKettleState` | 🔴 缺失 |
| enum | `TryFromFilteredError` | 🔴 缺失 |
| enum | `WeatherState` | 🔴 缺失 |
| trait | `Adapt` | 🔴 缺失 |
| trait | `ArchetypeFilter` | 🔴 缺失 |
| trait | `ArchetypeQueryData` | 🔴 缺失 |
| trait | `ClientState` | 🔴 缺失 |
| trait | `CloneByFilter` | 🔴 缺失 |
| trait | `Combine` | 🔴 缺失 |
| trait | `Command` | 🔴 缺失 |
| trait | `CommandWithEntity` | 🔴 缺失 |
| trait | `Component` | ✅ |
| trait | `ComponentMutability` | 🔴 缺失 |
| trait | `ContainsEntity` | 🔴 缺失 |
| trait | `DebugCheckedUnwrap` | 🔴 缺失 |
| trait | `DefaultCloneBehaviorBase` | 🔴 缺失 |
| trait | `DefaultCloneBehaviorViaClone` | 🔴 缺失 |
| trait | `DetectChanges` | 🔴 缺失 |
| trait | `DetectChangesMut` | 🔴 缺失 |
| trait | `DynEq` | 🔴 缺失 |
| trait | `DynHash` | 🔴 缺失 |
| trait | `DynamicBundle` | 🔴 缺失 |
| trait | `EntityCommand` | 🔴 缺失 |
| trait | `EntityEvent` | 🔴 缺失 |
| trait | `EntityMapper` | 🔴 缺失 |
| trait | `EntitySet` | 🔴 缺失 |
| trait | `Event` | 🔴 缺失 |
| trait | `ExclusiveSystemParam` | 🔴 缺失 |
| trait | `ExclusiveSystemParamFunction` | 🔴 缺失 |
| trait | `FilterableIds` | 🔴 缺失 |
| trait | `FnRet` | 🔴 缺失 |
| trait | `FromEntitySetIterator` | 🔴 缺失 |
| trait | `FromInput` | 🔴 缺失 |
| trait | `FromWorld` | 🔴 缺失 |
| trait | `GraphNodeId` | 🔴 缺失 |
| trait | `HandleError` | 🔴 缺失 |
| trait | `Internable` | 🔴 缺失 |
| trait | `IntoObserverSystem` | 🔴 缺失 |
| trait | `IntoResult` | 🔴 缺失 |
| trait | `IntoScheduleConfigs` | 🔴 缺失 |
| trait | `IntoSystem` | ✅ |
| trait | `IntoSystemSet` | 🔴 缺失 |
| trait | `MapEntities` | 🔴 缺失 |
| trait | `Marker` | 🔴 缺失 |
| trait | `Message` | 🔴 缺失 |
| trait | `NoBundleEffect` | 🔴 缺失 |
| trait | `ObserverSystem` | 🔴 缺失 |
| trait | `OrderedRelationshipSourceCollection` | 🔴 缺失 |
| trait | `ReflectCommandExt` | 🔴 缺失 |
| trait | `Relationship` | 🔴 缺失 |
| trait | `RelationshipCloneBehaviorBase` | 🔴 缺失 |
| trait | `RelationshipCloneBehaviorViaClone` | 🔴 缺失 |
| trait | `RelationshipCloneBehaviorViaReflect` | 🔴 缺失 |
| trait | `RelationshipSourceCollection` | 🔴 缺失 |
| trait | `RelationshipTarget` | 🔴 缺失 |
| trait | `RelationshipTargetCloneBehaviorHierarchy` | 🔴 缺失 |
| trait | `RelationshipTargetCloneBehaviorViaClone` | 🔴 缺失 |
| trait | `RelationshipTargetCloneBehaviorViaReflect` | 🔴 缺失 |
| trait | `ReleaseStateQueryData` | 🔴 缺失 |
| trait | `Resource` | 🔴 缺失 |
| trait | `RunSystemOnce` | 🔴 缺失 |
| trait | `Schedulable` | 🔴 缺失 |
| trait | `ScheduleBuildPass` | 🔴 缺失 |
| trait | `Seal` | 🔴 缺失 |
| trait | `SetEntityEventTarget` | 🔴 缺失 |
| trait | `SparseSetIndex` | 🔴 缺失 |
| trait | `SpawnRelated` | 🔴 缺失 |
| trait | `SpawnableList` | 🔴 缺失 |
| trait | `System` | ✅ |
| trait | `SystemBuffer` | 🔴 缺失 |
| trait | `SystemCondition` | 🔴 缺失 |
| trait | `SystemInput` | 🔴 缺失 |
| trait | `SystemParamFunction` | 🔴 缺失 |
| trait | `Traversal` | 🔴 缺失 |

### math

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `Aabb2d` | ✅ |
| struct | `Aabb3d` | ✅ |
| struct | `AabbCast2d` | 🔴 缺失 |
| struct | `AabbCast3d` | 🔴 缺失 |
| struct | `Affine3` | ✅ |
| struct | `Annulus` | 🔴 缺失 |
| struct | `Arc2d` | 🔴 缺失 |
| struct | `AspectRatio` | ✅ |
| struct | `BackInCurve` | 🔴 缺失 |
| struct | `BackInOutCurve` | 🔴 缺失 |
| struct | `BackOutCurve` | 🔴 缺失 |
| struct | `BounceInCurve` | 🔴 缺失 |
| struct | `BounceInOutCurve` | 🔴 缺失 |
| struct | `BounceOutCurve` | 🔴 缺失 |
| struct | `BoundaryOf` | 🔴 缺失 |
| struct | `BoundingCircle` | 🔴 缺失 |
| struct | `BoundingCircleCast` | 🔴 缺失 |
| struct | `BoundingSphere` | 🔴 缺失 |
| struct | `BoundingSphereCast` | 🔴 缺失 |
| struct | `Capsule2d` | ✅ |
| struct | `Capsule3d` | ✅ |
| struct | `ChainCurve` | 🔴 缺失 |
| struct | `ChunkedUnevenCore` | 🔴 缺失 |
| struct | `Circle` | ✅ |
| struct | `CircularInCurve` | 🔴 缺失 |
| struct | `CircularInOutCurve` | 🔴 缺失 |
| struct | `CircularOutCurve` | 🔴 缺失 |
| struct | `CircularSector` | 🔴 缺失 |
| struct | `CircularSegment` | 🔴 缺失 |
| struct | `Cone` | 🔴 缺失 |
| struct | `ConicalFrustum` | 🔴 缺失 |
| struct | `ConstantCurve` | 🔴 缺失 |
| struct | `ContinuationCurve` | 🔴 缺失 |
| struct | `ConvexPolygon` | 🔴 缺失 |
| struct | `CubicBSpline` | 🔴 缺失 |
| struct | `CubicBezier` | ✅ |
| struct | `CubicBezierError` | 🔴 缺失 |
| struct | `CubicCardinalSpline` | 🔴 缺失 |
| struct | `CubicCurve` | 🔴 缺失 |
| struct | `CubicHermite` | ✅ |
| struct | `CubicInCurve` | 🔴 缺失 |
| struct | `CubicInOutCurve` | 🔴 缺失 |
| struct | `CubicNurbs` | 🔴 缺失 |
| struct | `CubicOutCurve` | 🔴 缺失 |
| struct | `CubicSegment` | 🔴 缺失 |
| struct | `Cuboid` | ✅ |
| struct | `CurveReparamCurve` | 🔴 缺失 |
| struct | `Cylinder` | ✅ |
| struct | `Dir2` | ✅ |
| struct | `Dir3` | ✅ |
| struct | `Dir3A` | ✅ |
| struct | `Dir4` | 🔴 缺失 |
| struct | `EasingCurve` | 🔴 缺失 |
| struct | `ElasticCurve` | 🔴 缺失 |
| struct | `ElasticInCurve` | 🔴 缺失 |
| struct | `ElasticInOutCurve` | 🔴 缺失 |
| struct | `ElasticOutCurve` | 🔴 缺失 |
| struct | `Ellipse` | 🔴 缺失 |
| struct | `EvenCore` | 🔴 缺失 |
| struct | `ExponentialInCurve` | 🔴 缺失 |
| struct | `ExponentialInOutCurve` | 🔴 缺失 |
| struct | `ExponentialOutCurve` | 🔴 缺失 |
| struct | `Extrusion` | 🔴 缺失 |
| struct | `FloatOrd` | ✅ |
| struct | `ForeverCurve` | 🔴 缺失 |
| struct | `FunctionCurve` | 🔴 缺失 |
| struct | `GraphCurve` | 🔴 缺失 |
| struct | `IRect` | ✅ |
| struct | `InfinitePlane3d` | ✅ |
| struct | `InsufficientDataError` | 🔴 缺失 |
| struct | `InteriorOf` | 🔴 缺失 |
| struct | `Interval` | 🔴 缺失 |
| struct | `InvalidIntervalError` | 🔴 缺失 |
| struct | `Isometry2d` | ✅ |
| struct | `Isometry3d` | ✅ |
| struct | `Line2d` | 🔴 缺失 |
| struct | `Line3d` | 🔴 缺失 |
| struct | `LinearCurve` | 🔴 缺失 |
| struct | `LinearReparamCurve` | 🔴 缺失 |
| struct | `LinearSpline` | 🔴 缺失 |
| struct | `MapCurve` | 🔴 缺失 |
| struct | `MismatchedUnitsError` | 🔴 缺失 |
| struct | `PingPongCurve` | 🔴 缺失 |
| struct | `Plane2d` | ✅ |
| struct | `Plane3d` | ✅ |
| struct | `Polygon` | 🔴 缺失 |
| struct | `Polyline2d` | 🔴 缺失 |
| struct | `Polyline3d` | 🔴 缺失 |
| struct | `QuadraticInCurve` | 🔴 缺失 |
| struct | `QuadraticInOutCurve` | 🔴 缺失 |
| struct | `QuadraticOutCurve` | 🔴 缺失 |
| struct | `QuarticInCurve` | 🔴 缺失 |
| struct | `QuarticInOutCurve` | 🔴 缺失 |
| struct | `QuarticOutCurve` | 🔴 缺失 |
| struct | `QuinticInCurve` | 🔴 缺失 |
| struct | `QuinticInOutCurve` | 🔴 缺失 |
| struct | `QuinticOutCurve` | 🔴 缺失 |
| struct | `RationalCurve` | 🔴 缺失 |
| struct | `RationalSegment` | 🔴 缺失 |
| struct | `Ray2d` | ✅ |
| struct | ... | (更多省略) |
| enum | `AspectRatioError` | 🔴 缺失 |
| enum | `ChainError` | 🔴 缺失 |
| enum | `ChunkedUnevenCoreError` | 🔴 缺失 |
| enum | `CompassOctant` | ✅ |
| enum | `CompassQuadrant` | ✅ |
| enum | `ConvexPolygonError` | 🔴 缺失 |
| enum | `CubicNurbsError` | 🔴 缺失 |
| enum | `EaseFunction` | ✅ |
| enum | `EvenCoreError` | 🔴 缺失 |
| enum | `InterpolationDatum` | 🔴 缺失 |
| enum | `InvalidDirectionError` | ✅ |
| enum | `JumpAt` | 🔴 缺失 |
| enum | `LinearReparamError` | 🔴 缺失 |
| enum | `PingPongError` | 🔴 缺失 |
| enum | `RepeatError` | 🔴 缺失 |
| enum | `ResamplingError` | 🔴 缺失 |
| enum | `ReverseError` | 🔴 缺失 |
| enum | `TorusKind` | 🔴 缺失 |
| enum | `UnevenCoreError` | 🔴 缺失 |
| enum | `WindingOrder` | 🔴 缺失 |
| trait | `Bounded2d` | 🔴 缺失 |
| trait | `Bounded3d` | 🔴 缺失 |
| trait | `BoundedExtrusion` | 🔴 缺失 |
| trait | `BoundingVolume` | 🔴 缺失 |
| trait | `CubicGenerator` | 🔴 缺失 |
| trait | `Curve` | 🔴 缺失 |
| trait | `CurveExt` | 🔴 缺失 |
| trait | `CurveResampleExt` | 🔴 缺失 |
| trait | `CurveWithDerivative` | 🔴 缺失 |
| trait | `CurveWithTwoDerivatives` | 🔴 缺失 |
| trait | `CyclicCubicGenerator` | 🔴 缺失 |
| trait | `Ease` | 🔴 缺失 |
| trait | `FloatPow` | ✅ |
| trait | `FromRng` | 🔴 缺失 |
| trait | `HasTangent` | 🔴 缺失 |
| trait | `Inset` | 🔴 缺失 |
| trait | `IntersectsVolume` | 🔴 缺失 |
| trait | `IterableCurve` | 🔴 缺失 |
| trait | `Measured2d` | 🔴 缺失 |
| trait | `Measured3d` | 🔴 缺失 |
| trait | `NormedVectorSpace` | 🔴 缺失 |
| trait | `Primitive2d` | 🔴 缺失 |
| trait | `Primitive3d` | 🔴 缺失 |
| trait | `RationalGenerator` | 🔴 缺失 |
| trait | `SampleDerivative` | 🔴 缺失 |
| trait | `SampleTwoDerivatives` | 🔴 缺失 |
| trait | `ScalarField` | 🔴 缺失 |
| trait | `ShapeSample` | 🔴 缺失 |
| trait | `StableInterpolate` | 🔴 缺失 |
| trait | `ToRing` | 🔴 缺失 |
| trait | `TryStableInterpolate` | 🔴 缺失 |
| trait | `VectorSpace` | 🔴 缺失 |

### render

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `AdditionalVulkanFeatures` | 🔴 缺失 |
| struct | `AssetExtractionSystems` | 🔴 缺失 |
| struct | `BatchedInstanceBuffer` | 🔴 缺失 |
| struct | `BatchedInstanceBuffers` | 🔴 缺失 |
| struct | `BatchedUniformBuffer` | 🔴 缺失 |
| struct | `BatchingPlugin` | 🔴 缺失 |
| struct | `BindGroup` | 🔴 缺失 |
| struct | `BindGroupEntries` | 🔴 缺失 |
| struct | `BindGroupLayout` | 🔴 缺失 |
| struct | `BindGroupLayoutDescriptor` | 🔴 缺失 |
| struct | `BindGroupLayoutEntries` | 🔴 缺失 |
| struct | `BindGroupLayoutEntryBuilder` | 🔴 缺失 |
| struct | `BindingNumber` | 🔴 缺失 |
| struct | `BindingResources` | 🔴 缺失 |
| struct | `BindlessBufferDescriptor` | 🔴 缺失 |
| struct | `BindlessDescriptor` | 🔴 缺失 |
| struct | `BindlessIndex` | 🔴 缺失 |
| struct | `BindlessIndexTableDescriptor` | 🔴 缺失 |
| struct | `BinnedRenderPhase` | 🔴 缺失 |
| struct | `BinnedRenderPhaseBatch` | 🔴 缺失 |
| struct | `BinnedRenderPhaseBatchSet` | 🔴 缺失 |
| struct | `BinnedRenderPhasePlugin` | 🔴 缺失 |
| struct | `Buffer` | 🔴 缺失 |
| struct | `BufferSlice` | 🔴 缺失 |
| struct | `BufferVec` | 🔴 缺失 |
| struct | `CachedBinKey` | 🔴 缺失 |
| struct | `CachedBinnedEntity` | 🔴 缺失 |
| struct | `CachedComputePipelineId` | 🔴 缺失 |
| struct | `CachedPipeline` | 🔴 缺失 |
| struct | `CachedRenderPipelineId` | 🔴 缺失 |
| struct | `CachedTexture` | 🔴 缺失 |
| struct | `CameraDriverLabel` | 🔴 缺失 |
| struct | `CameraDriverNode` | 🔴 缺失 |
| struct | `CameraPlugin` | 🔴 缺失 |
| struct | `CameraRenderGraph` | 🔴 缺失 |
| struct | `Captured` | 🔴 缺失 |
| struct | `CapturedScreenshots` | 🔴 缺失 |
| struct | `Capturing` | 🔴 缺失 |
| struct | `ColorAttachment` | ✅ |
| struct | `ColorGrading` | 🔴 缺失 |
| struct | `ColorGradingGlobal` | 🔴 缺失 |
| struct | `ColorGradingSection` | 🔴 缺失 |
| struct | `ColorGradingUniform` | 🔴 缺失 |
| struct | `ComponentUniforms` | 🔴 缺失 |
| struct | `ComputePipeline` | 🔴 缺失 |
| struct | `ComputePipelineDescriptor` | 🔴 缺失 |
| struct | `DefaultImageSampler` | 🔴 缺失 |
| struct | `DepthAttachment` | 🔴 缺失 |
| struct | `DiagnosticsRecorder` | 🔴 缺失 |
| struct | `DrawFunctionId` | 🔴 缺失 |
| struct | `DrawFunctions` | 🔴 缺失 |
| struct | `DrawFunctionsInternal` | 🔴 缺失 |
| struct | `DynamicBindGroupEntries` | 🔴 缺失 |
| struct | `DynamicBindGroupLayoutEntries` | 🔴 缺失 |
| struct | `DynamicStorageBuffer` | 🔴 缺失 |
| struct | `DynamicUniformBuffer` | 🔴 缺失 |
| struct | `DynamicUniformBufferWriter` | 🔴 缺失 |
| struct | `DynamicUniformIndex` | 🔴 缺失 |
| struct | `Edges` | 🔴 缺失 |
| struct | `EmptyNode` | 🔴 缺失 |
| struct | `ErasedRenderAssetDiagnosticPlugin` | 🔴 缺失 |
| struct | `ErasedRenderAssetPlugin` | 🔴 缺失 |
| struct | `ErasedRenderAssets` | 🔴 缺失 |
| struct | `Extract` | 🔴 缺失 |
| struct | `ExtractComponentPlugin` | 🔴 缺失 |
| struct | `ExtractInstancesPlugin` | 🔴 缺失 |
| struct | `ExtractResourcePlugin` | 🔴 缺失 |
| struct | `ExtractSchedule` | 🔴 缺失 |
| struct | `ExtractState` | 🔴 缺失 |
| struct | `ExtractedAssets` | 🔴 缺失 |
| struct | `ExtractedCamera` | 🔴 缺失 |
| struct | `ExtractedInstances` | 🔴 缺失 |
| struct | `ExtractedView` | 🔴 缺失 |
| struct | `ExtractedWindow` | 🔴 缺失 |
| struct | `ExtractedWindows` | 🔴 缺失 |
| struct | `FallbackImage` | 🔴 缺失 |
| struct | `FallbackImageCubemap` | 🔴 缺失 |
| struct | `FallbackImageFormatMsaaCache` | 🔴 缺失 |
| struct | `FallbackImageMsaa` | 🔴 缺失 |
| struct | `FallbackImageZero` | 🔴 缺失 |
| struct | `FragmentState` | 🔴 缺失 |
| struct | `GlobalsBuffer` | 🔴 缺失 |
| struct | `GlobalsPlugin` | 🔴 缺失 |
| struct | `GlobalsUniform` | 🔴 缺失 |
| struct | `GpuArrayBufferIndex` | 🔴 缺失 |
| struct | `GpuComponentArrayBufferPlugin` | 🔴 缺失 |
| struct | `GpuImage` | 🔴 缺失 |
| struct | `GpuOcclusionCullingWorkItemBuffers` | 🔴 缺失 |
| struct | `GpuPreprocessingSupport` | 🔴 缺失 |
| struct | `GpuReadbackPlugin` | 🔴 缺失 |
| struct | `GpuShaderStorageBuffer` | 🔴 缺失 |
| struct | `GraphInput` | 🔴 缺失 |
| struct | `GraphInputNode` | 🔴 缺失 |
| struct | `Hdr` | 🔴 缺失 |
| struct | `IndirectBatchSet` | 🔴 缺失 |
| struct | `IndirectParametersBuffers` | 🔴 缺失 |
| struct | `IndirectParametersCpuMetadata` | 🔴 缺失 |
| struct | `IndirectParametersGpuMetadata` | 🔴 缺失 |
| struct | `IndirectParametersIndexed` | 🔴 缺失 |
| struct | `IndirectParametersNonIndexed` | 🔴 缺失 |
| struct | ... | (更多省略) |
| enum | `AlphaMode` | 🔴 缺失 |
| enum | `AsBindGroupError` | 🔴 缺失 |
| enum | `AssetExtractionError` | 🔴 缺失 |
| enum | `BindlessResourceType` | 🔴 缺失 |
| enum | `BindlessSlabResourceLimit` | 🔴 缺失 |
| enum | `BinnedRenderPhaseBatchSets` | 🔴 缺失 |
| enum | `BinnedRenderPhaseType` | 🔴 缺失 |
| enum | `CachedPipelineState` | 🔴 缺失 |
| enum | `DrawError` | 🔴 缺失 |
| enum | `Edge` | 🔴 缺失 |
| enum | `EdgeExistence` | 🔴 缺失 |
| enum | `GpuArrayBuffer` | 🔴 缺失 |
| enum | `GpuPreprocessingMode` | 🔴 缺失 |
| enum | `InputSlotError` | 🔴 缺失 |
| enum | `MissingRenderTargetInfoError` | 🔴 缺失 |
| enum | `Msaa` | 🔴 缺失 |
| enum | `NodeRunError` | 🔴 缺失 |
| enum | `OutputSlotError` | 🔴 缺失 |
| enum | `OwnedBindingResource` | 🔴 缺失 |
| enum | `PassKind` | 🔴 缺失 |
| enum | `PhaseItemExtraIndex` | 🔴 缺失 |
| enum | `Pipeline` | 🔴 缺失 |
| enum | `PipelineDescriptor` | 🔴 缺失 |
| enum | `PrepareAssetError` | 🔴 缺失 |
| enum | `PreprocessWorkItemBuffers` | 🔴 缺失 |
| enum | `Readback` | 🔴 缺失 |
| enum | `RenderCommandResult` | 🔴 缺失 |
| enum | `RenderCreation` | 🔴 缺失 |
| enum | `RenderGraphError` | 🔴 缺失 |
| enum | `RenderGraphRunnerError` | 🔴 缺失 |
| enum | `RenderMeshBufferInfo` | 🔴 缺失 |
| enum | `RenderSystems` | 🔴 缺失 |
| enum | `RunSubGraphError` | 🔴 缺失 |
| enum | `SlotLabel` | 🔴 缺失 |
| enum | `SlotType` | 🔴 缺失 |
| enum | `SlotValue` | 🔴 缺失 |
| enum | `SpecializedMeshPipelineError` | 🔴 缺失 |
| enum | `WgpuSettingsPriority` | 🔴 缺失 |
| enum | `WriteBufferRangeError` | 🔴 缺失 |
| trait | `AddRenderCommand` | 🔴 缺失 |
| trait | `AsBindGroup` | 🔴 缺失 |
| trait | `AsBindGroupShaderType` | 🔴 缺失 |
| trait | `BinnedPhaseItem` | 🔴 缺失 |
| trait | `CachedRenderPipelinePhaseItem` | 🔴 缺失 |
| trait | `Draw` | 🔴 缺失 |
| trait | `ErasedRenderAsset` | 🔴 缺失 |
| trait | `ErasedRenderAssetDependency` | 🔴 缺失 |
| trait | `ExtractComponent` | 🔴 缺失 |
| trait | `ExtractInstance` | 🔴 缺失 |
| trait | `ExtractResource` | 🔴 缺失 |
| trait | `GetBatchData` | 🔴 缺失 |
| trait | `GetFullBatchData` | 🔴 缺失 |
| trait | `GpuArrayBufferable` | 🔴 缺失 |
| trait | `IntoBindGroupLayoutEntryBuilder` | 🔴 缺失 |
| trait | `IntoBindGroupLayoutEntryBuilderArray` | 🔴 缺失 |
| trait | `IntoBinding` | 🔴 缺失 |
| trait | `IntoBindingArray` | 🔴 缺失 |
| trait | `IntoIndexedBindGroupLayoutEntryBuilderArray` | 🔴 缺失 |
| trait | `IntoIndexedBindingArray` | 🔴 缺失 |
| trait | `IntoRenderNodeArray` | 🔴 缺失 |
| trait | `Node` | 🔴 缺失 |
| trait | `NormalizedRenderTargetExt` | 🔴 缺失 |
| trait | `Pass` | 🔴 缺失 |
| trait | `PhaseItem` | 🔴 缺失 |
| trait | `PhaseItemBatchSetKey` | 🔴 缺失 |
| trait | `RecordDiagnostics` | 🔴 缺失 |
| trait | `RenderAsset` | 🔴 缺失 |
| trait | `RenderAssetDependency` | 🔴 缺失 |
| trait | `RenderCommand` | 🔴 缺失 |
| trait | `RenderGraphExt` | 🔴 缺失 |
| trait | `SortedPhaseItem` | 🔴 缺失 |
| trait | `Specializable` | 🔴 缺失 |
| trait | `SpecializedComputePipeline` | 🔴 缺失 |
| trait | `SpecializedMeshPipeline` | 🔴 缺失 |
| trait | `SpecializedRenderPipeline` | 🔴 缺失 |
| trait | `Specializer` | 🔴 缺失 |
| trait | `SpecializerKey` | 🔴 缺失 |
| trait | `ViewNode` | 🔴 缺失 |
| trait | `WritePipelineStatistics` | 🔴 缺失 |
| trait | `WriteTimestamp` | 🔴 缺失 |

### transform

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `GlobalTransform` | 🔴 缺失 |
| struct | `StaticTransformOptimizations` | 🔴 缺失 |
| struct | `Transform` | ✅ |
| struct | `TransformHelper` | 🔴 缺失 |
| struct | `TransformPlugin` | 🔴 缺失 |
| struct | `TransformTreeChanged` | 🔴 缺失 |
| struct | `WorkQueue` | 🔴 缺失 |
| enum | `ComputeGlobalTransformError` | 🔴 缺失 |
| enum | `TransformSystems` | 🔴 缺失 |
| trait | `BuildChildrenTransformExt` | 🔴 缺失 |
| trait | `TransformPoint` | 🔴 缺失 |

### mesh

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `AnnulusMeshBuilder` | 🔴 缺失 |
| struct | `BaseMeshPipelineKey` | 🔴 缺失 |
| struct | `Capsule2dMeshBuilder` | 🔴 缺失 |
| struct | `Capsule3dMeshBuilder` | 🔴 缺失 |
| struct | `CircleMeshBuilder` | 🔴 缺失 |
| struct | `CircularSectorMeshBuilder` | 🔴 缺失 |
| struct | `CircularSegmentMeshBuilder` | 🔴 缺失 |
| struct | `ConeMeshBuilder` | 🔴 缺失 |
| struct | `ConicalFrustumMeshBuilder` | 🔴 缺失 |
| struct | `ConvexPolygonMeshBuilder` | 🔴 缺失 |
| struct | `CuboidMeshBuilder` | 🔴 缺失 |
| struct | `CylinderMeshBuilder` | 🔴 缺失 |
| struct | `EllipseMeshBuilder` | 🔴 缺失 |
| struct | `ExtrusionBuilder` | 🔴 缺失 |
| struct | `FromVertexAttributeError` | 🔴 缺失 |
| struct | `InheritWeightSystems` | 🔴 缺失 |
| struct | `Mesh` | ✅ |
| struct | `Mesh2d` | 🔴 缺失 |
| struct | `Mesh3d` | 🔴 缺失 |
| struct | `MeshDeserializer` | 🔴 缺失 |
| struct | `MeshMorphWeights` | 🔴 缺失 |
| struct | `MeshPlugin` | 🔴 缺失 |
| struct | `MeshTag` | 🔴 缺失 |
| struct | `MeshVertexAttribute` | 🔴 缺失 |
| struct | `MeshVertexAttributeId` | 🔴 缺失 |
| struct | `MeshVertexBufferLayout` | 🔴 缺失 |
| struct | `MeshVertexBufferLayoutRef` | 🔴 缺失 |
| struct | `MeshVertexBufferLayouts` | 🔴 缺失 |
| struct | `MissingVertexAttributeError` | 🔴 缺失 |
| struct | `MorphAttributes` | 🔴 缺失 |
| struct | `MorphTargetImage` | 🔴 缺失 |
| struct | `MorphWeights` | 🔴 缺失 |
| struct | `PlaneMeshBuilder` | 🔴 缺失 |
| struct | `Polyline2dMeshBuilder` | 🔴 缺失 |
| struct | `Polyline3dMeshBuilder` | 🔴 缺失 |
| struct | `RectangleMeshBuilder` | 🔴 缺失 |
| struct | `RegularPolygonMeshBuilder` | 🔴 缺失 |
| struct | `RhombusMeshBuilder` | 🔴 缺失 |
| struct | `RingMeshBuilder` | 🔴 缺失 |
| struct | `Segment2dMeshBuilder` | 🔴 缺失 |
| struct | `Segment3dMeshBuilder` | 🔴 缺失 |
| struct | `SerializedMesh` | 🔴 缺失 |
| struct | `SkinnedMesh` | 🔴 缺失 |
| struct | `SkinnedMeshInverseBindposes` | 🔴 缺失 |
| struct | `SphereMeshBuilder` | 🔴 缺失 |
| struct | `TetrahedronMeshBuilder` | 🔴 缺失 |
| struct | `TorusMeshBuilder` | 🔴 缺失 |
| struct | `Triangle2dMeshBuilder` | 🔴 缺失 |
| struct | `Triangle3dMeshBuilder` | 🔴 缺失 |
| struct | `VertexAttributeDescriptor` | 🔴 缺失 |
| struct | `VertexBufferLayout` | ✅ |
| enum | `CapsuleUvProfile` | 🔴 缺失 |
| enum | `CircularMeshUvMode` | 🔴 缺失 |
| enum | `ConeAnchor` | 🔴 缺失 |
| enum | `CylinderAnchor` | 🔴 缺失 |
| enum | `GenerateTangentsError` | 🔴 缺失 |
| enum | `IcosphereError` | 🔴 缺失 |
| enum | `Indices` | 🔴 缺失 |
| enum | `MeshAccessError` | 🔴 缺失 |
| enum | `MeshMergeError` | 🔴 缺失 |
| enum | `MeshTrianglesError` | 🔴 缺失 |
| enum | `MeshWindingInvertError` | 🔴 缺失 |
| enum | `MorphBuildError` | 🔴 缺失 |
| enum | `PerimeterSegment` | 🔴 缺失 |
| enum | `SphereKind` | 🔴 缺失 |
| enum | `VertexAttributeValues` | 🔴 缺失 |
| trait | `Extrudable` | 🔴 缺失 |
| trait | `MeshBuilder` | 🔴 缺失 |
| trait | `Meshable` | 🔴 缺失 |

### pbr

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `Atmosphere` | 🔴 缺失 |
| struct | `AtmosphereBuffer` | 🔴 缺失 |
| struct | `AtmosphereEnvironmentMap` | 🔴 缺失 |
| struct | `AtmospherePlugin` | 🔴 缺失 |
| struct | `AtmosphereProbeLayouts` | 🔴 缺失 |
| struct | `AtmosphereProbePipeline` | 🔴 缺失 |
| struct | `AtmosphereProbeTextures` | 🔴 缺失 |
| struct | `AtmosphereSampler` | 🔴 缺失 |
| struct | `AtmosphereSettings` | 🔴 缺失 |
| struct | `AtmosphereTextures` | 🔴 缺失 |
| struct | `AtmosphereTransform` | 🔴 缺失 |
| struct | `AtmosphereTransforms` | 🔴 缺失 |
| struct | `AtmosphereTransformsOffset` | 🔴 缺失 |
| struct | `Bluenoise` | 🔴 缺失 |
| struct | `BuildIndirectParametersBindGroups` | 🔴 缺失 |
| struct | `BuildIndirectParametersPipeline` | 🔴 缺失 |
| struct | `BuildIndirectParametersPipelineKey` | 🔴 缺失 |
| struct | `BvhNode` | 🔴 缺失 |
| struct | `ClearIndirectParametersMetadataNode` | 🔴 缺失 |
| struct | `ClusteredDecalPlugin` | 🔴 缺失 |
| struct | `CustomMaterial` | 🔴 缺失 |
| struct | `DecalsBuffer` | 🔴 缺失 |
| struct | `DefaultOpaqueRendererMethod` | 🔴 缺失 |
| struct | `DeferredAlphaMaskDrawFunction` | 🔴 缺失 |
| struct | `DeferredFragmentShader` | 🔴 缺失 |
| struct | `DeferredLightingLayout` | 🔴 缺失 |
| struct | `DeferredLightingPipeline` | 🔴 缺失 |
| struct | `DeferredOpaqueDrawFunction` | 🔴 缺失 |
| struct | `DeferredOpaquePass3dPbrLightingNode` | 🔴 缺失 |
| struct | `DeferredPbrLightingPlugin` | 🔴 缺失 |
| struct | `DeferredVertexShader` | 🔴 缺失 |
| struct | `DistanceFog` | 🔴 缺失 |
| struct | `DownsamplingConfig` | 🔴 缺失 |
| struct | `DownsamplingNode` | 🔴 缺失 |
| struct | `DrawMesh` | 🔴 缺失 |
| struct | `EarlyGpuPreprocessNode` | 🔴 缺失 |
| struct | `EarlyPrepassBuildIndirectParametersNode` | 🔴 缺失 |
| struct | `EarlyShadowPassNode` | 🔴 缺失 |
| struct | `EarthlikeAtmosphere` | 🔴 缺失 |
| struct | `EntitiesNeedingSpecialization` | 🔴 缺失 |
| struct | `EntitySpecializationTickPair` | 🔴 缺失 |
| struct | `EntitySpecializationTicks` | 🔴 缺失 |
| struct | `EnvironmentMapGenerationPlugin` | 🔴 缺失 |
| struct | `EnvironmentMapIds` | 🔴 缺失 |
| struct | `EnvironmentMapUniform` | 🔴 缺失 |
| struct | `EnvironmentMapUniformBuffer` | 🔴 缺失 |
| struct | `EnvironmentMapViewLightProbeInfo` | 🔴 缺失 |
| struct | `ErasedMaterialKey` | 🔴 缺失 |
| struct | `ErasedMaterialKeyVTable` | 🔴 缺失 |
| struct | `ErasedMaterialPipelineKey` | 🔴 缺失 |
| struct | `ExtendedMaterial` | 🔴 缺失 |
| struct | `ExtractedAtmosphere` | 🔴 缺失 |
| struct | `ExtractedClusterConfig` | 🔴 缺失 |
| struct | `ExtractedClusterableObjects` | 🔴 缺失 |
| struct | `ExtractedDirectionalLight` | 🔴 缺失 |
| struct | `ExtractedPointLight` | 🔴 缺失 |
| struct | `ExtractedWireframeColor` | 🔴 缺失 |
| struct | `FallbackBindlessResources` | 🔴 缺失 |
| struct | `FilteringConstants` | 🔴 缺失 |
| struct | `FilteringNode` | 🔴 缺失 |
| struct | `FogAssets` | 🔴 缺失 |
| struct | `FogMeta` | 🔴 缺失 |
| struct | `FogPlugin` | 🔴 缺失 |
| struct | `ForwardDecal` | 🔴 缺失 |
| struct | `ForwardDecalMaterialExt` | 🔴 缺失 |
| struct | `ForwardDecalMaterialExtUniform` | 🔴 缺失 |
| struct | `ForwardDecalPlugin` | 🔴 缺失 |
| struct | `GeneratorBindGroupLayouts` | 🔴 缺失 |
| struct | `GeneratorBindGroups` | 🔴 缺失 |
| struct | `GeneratorPipelines` | 🔴 缺失 |
| struct | `GeneratorSamplers` | 🔴 缺失 |
| struct | `GlobalClusterableObjectMeta` | 🔴 缺失 |
| struct | `GpuAtmosphere` | 🔴 缺失 |
| struct | `GpuAtmosphereSettings` | 🔴 缺失 |
| struct | `GpuClusterableObject` | 🔴 缺失 |
| struct | `GpuClusterableObjectsStorage` | 🔴 缺失 |
| struct | `GpuClusterableObjectsUniform` | 🔴 缺失 |
| struct | `GpuDirectionalCascade` | 🔴 缺失 |
| struct | `GpuDirectionalLight` | 🔴 缺失 |
| struct | `GpuFog` | 🔴 缺失 |
| struct | `GpuLights` | 🔴 缺失 |
| struct | `GpuMeshPreprocessPlugin` | 🔴 缺失 |
| struct | `GpuScatteringMedium` | 🔴 缺失 |
| struct | `InstanceManager` | 🔴 缺失 |
| struct | `IntermediateTextures` | 🔴 缺失 |
| struct | `LateGpuPreprocessNode` | 🔴 缺失 |
| struct | `LatePrepassBuildIndirectParametersNode` | 🔴 缺失 |
| struct | `LateShadowPassNode` | 🔴 缺失 |
| struct | `LightKeyCache` | 🔴 缺失 |
| struct | `LightMeta` | 🔴 缺失 |
| struct | `LightProbePlugin` | 🔴 缺失 |
| struct | `LightProbesBuffer` | 🔴 缺失 |
| struct | `LightProbesUniform` | 🔴 缺失 |
| struct | `LightSpecializationTicks` | 🔴 缺失 |
| struct | `LightViewEntities` | 🔴 缺失 |
| struct | `Lightmap` | 🔴 缺失 |
| struct | `LightmapPlugin` | 🔴 缺失 |
| struct | `LightmapSlab` | 🔴 缺失 |
| struct | `LightmapSlabIndex` | 🔴 缺失 |
| struct | `LightmapSlotIndex` | 🔴 缺失 |
| struct | ... | (更多省略) |
| enum | `AtmosphereMode` | 🔴 缺失 |
| enum | `AtmosphereNode` | 🔴 缺失 |
| enum | `Falloff` | 🔴 缺失 |
| enum | `FogFalloff` | 🔴 缺失 |
| enum | `GeneratorNode` | 🔴 缺失 |
| enum | `GpuClusterableObjects` | 🔴 缺失 |
| enum | `LightEntity` | 🔴 缺失 |
| enum | `MaterialBindGroupAllocator` | 🔴 缺失 |
| enum | `MeshBindGroups` | 🔴 缺失 |
| enum | `MeshToMeshletMeshConversionError` | 🔴 缺失 |
| enum | `MeshletMeshSaveOrLoadError` | 🔴 缺失 |
| enum | `NodeMeshlet` | 🔴 缺失 |
| enum | `NodePbr` | 🔴 缺失 |
| enum | `OpaqueRendererMethod` | 🔴 缺失 |
| enum | `ParallaxMappingMethod` | 🔴 缺失 |
| enum | `PhaseFunction` | 🔴 缺失 |
| enum | `PhasePreprocessBindGroups` | 🔴 缺失 |
| enum | `RenderMeshInstanceGpuQueue` | 🔴 缺失 |
| enum | `RenderMeshInstances` | 🔴 缺失 |
| enum | `RenderPhaseType` | 🔴 缺失 |
| enum | `ScreenSpaceAmbientOcclusionQualityLevel` | 🔴 缺失 |
| enum | `UvChannel` | 🔴 缺失 |
| trait | `LightProbeComponent` | 🔴 缺失 |
| trait | `Material` | 🔴 缺失 |
| trait | `MaterialExtension` | 🔴 缺失 |
| trait | `PersistentGpuBufferable` | 🔴 缺失 |

### light

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `AmbientLight` | ✅ |
| struct | `AtmosphereEnvironmentMapLight` | 🔴 缺失 |
| struct | `Cascade` | ✅ |
| struct | `CascadeShadowConfig` | 🔴 缺失 |
| struct | `CascadeShadowConfigBuilder` | 🔴 缺失 |
| struct | `Cascades` | 🔴 缺失 |
| struct | `ClusterVisibilityClass` | 🔴 缺失 |
| struct | `ClusterZConfig` | 🔴 缺失 |
| struct | `ClusterableObjectCounts` | 🔴 缺失 |
| struct | `ClusteredDecal` | 🔴 缺失 |
| struct | `Clusters` | 🔴 缺失 |
| struct | `DirectionalLight` | ✅ |
| struct | `DirectionalLightShadowMap` | 🔴 缺失 |
| struct | `DirectionalLightTexture` | 🔴 缺失 |
| struct | `EnvironmentMapLight` | 🔴 缺失 |
| struct | `FogVolume` | 🔴 缺失 |
| struct | `GeneratedEnvironmentMapLight` | 🔴 缺失 |
| struct | `GlobalAmbientLight` | 🔴 缺失 |
| struct | `GlobalClusterSettings` | 🔴 缺失 |
| struct | `GlobalVisibleClusterableObjects` | 🔴 缺失 |
| struct | `IrradianceVolume` | 🔴 缺失 |
| struct | `LightPlugin` | 🔴 缺失 |
| struct | `LightProbe` | 🔴 缺失 |
| struct | `NotShadowCaster` | 🔴 缺失 |
| struct | `NotShadowReceiver` | 🔴 缺失 |
| struct | `PointLight` | ✅ |
| struct | `PointLightShadowMap` | 🔴 缺失 |
| struct | `PointLightTexture` | 🔴 缺失 |
| struct | `SpotLight` | ✅ |
| struct | `SpotLightTexture` | 🔴 缺失 |
| struct | `SunDisk` | 🔴 缺失 |
| struct | `TransmittedShadowReceiver` | 🔴 缺失 |
| struct | `VisibleClusterableObjects` | 🔴 缺失 |
| struct | `VolumetricFog` | 🔴 缺失 |
| struct | `VolumetricLight` | 🔴 缺失 |
| enum | `ClusterConfig` | 🔴 缺失 |
| enum | `ClusterFarZMode` | 🔴 缺失 |
| enum | `ClusterableObjectType` | 🔴 缺失 |
| enum | `ShadowFilteringMethod` | 🔴 缺失 |
| enum | `SimulationLightSystems` | 🔴 缺失 |

### color

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `ColorCurve` | 🔴 缺失 |
| struct | `Hsla` | ✅ |
| struct | `Hsva` | ✅ |
| struct | `Hwba` | 🔴 缺失 |
| struct | `Laba` | 🔴 缺失 |
| struct | `Lcha` | 🔴 缺失 |
| struct | `LinearRgba` | ✅ |
| struct | `Oklaba` | 🔴 缺失 |
| struct | `Oklcha` | 🔴 缺失 |
| struct | `Srgba` | 🔴 缺失 |
| struct | `TestColor` | 🔴 缺失 |
| struct | `Xyza` | 🔴 缺失 |
| enum | `Color` | 🔴 缺失 |
| enum | `HexColorError` | 🔴 缺失 |
| trait | `Alpha` | 🔴 缺失 |
| trait | `ColorRange` | 🔴 缺失 |
| trait | `ColorToComponents` | 🔴 缺失 |
| trait | `ColorToPacked` | 🔴 缺失 |
| trait | `EuclideanDistance` | 🔴 缺失 |
| trait | `Gray` | 🔴 缺失 |
| trait | `Hue` | 🔴 缺失 |
| trait | `Luminance` | 🔴 缺失 |
| trait | `Mix` | 🔴 缺失 |
| trait | `Saturation` | 🔴 缺失 |

### input

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `AccumulatedMouseMotion` | 🔴 缺失 |
| struct | `AccumulatedMouseScroll` | 🔴 缺失 |
| struct | `Axis` | 🔴 缺失 |
| struct | `AxisSettings` | 🔴 缺失 |
| struct | `ButtonAxisSettings` | 🔴 缺失 |
| struct | `ButtonInput` | 🔴 缺失 |
| struct | `ButtonSettings` | 🔴 缺失 |
| struct | `DoubleTapGesture` | 🔴 缺失 |
| struct | `Gamepad` | ✅ |
| struct | `GamepadAxisChangedEvent` | 🔴 缺失 |
| struct | `GamepadButtonChangedEvent` | 🔴 缺失 |
| struct | `GamepadButtonStateChangedEvent` | 🔴 缺失 |
| struct | `GamepadConnectionEvent` | 🔴 缺失 |
| struct | `GamepadRumbleIntensity` | 🔴 缺失 |
| struct | `GamepadSettings` | 🔴 缺失 |
| struct | `InputPlugin` | 🔴 缺失 |
| struct | `InputSystems` | 🔴 缺失 |
| struct | `KeyboardFocusLost` | 🔴 缺失 |
| struct | `KeyboardInput` | ✅ |
| struct | `MouseButtonInput` | ✅ |
| struct | `MouseMotion` | ✅ |
| struct | `MouseWheel` | ✅ |
| struct | `PanGesture` | 🔴 缺失 |
| struct | `PinchGesture` | 🔴 缺失 |
| struct | `RawGamepadAxisChangedEvent` | 🔴 缺失 |
| struct | `RawGamepadButtonChangedEvent` | 🔴 缺失 |
| struct | `RotationGesture` | 🔴 缺失 |
| struct | `Touch` | ✅ |
| struct | `TouchInput` | ✅ |
| struct | `Touches` | 🔴 缺失 |
| enum | `AxisSettingsError` | 🔴 缺失 |
| enum | `ButtonSettingsError` | 🔴 缺失 |
| enum | `ButtonState` | 🔴 缺失 |
| enum | `ForceTouch` | 🔴 缺失 |
| enum | `GamepadAxis` | ✅ |
| enum | `GamepadButton` | ✅ |
| enum | `GamepadConnection` | 🔴 缺失 |
| enum | `GamepadEvent` | 🔴 缺失 |
| enum | `GamepadInput` | 🔴 缺失 |
| enum | `GamepadRumbleRequest` | 🔴 缺失 |
| enum | `Key` | ✅ |
| enum | `KeyCode` | ✅ |
| enum | `MouseButton` | ✅ |
| enum | `MouseScrollUnit` | ✅ |
| enum | `NativeKey` | 🔴 缺失 |
| enum | `NativeKeyCode` | 🔴 缺失 |
| enum | `RawGamepadEvent` | 🔴 缺失 |
| enum | `TouchPhase` | ✅ |

### window

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `ClosingWindow` | 🔴 缺失 |
| struct | `CursorEntered` | 🔴 缺失 |
| struct | `CursorLeft` | 🔴 缺失 |
| struct | `CursorMoved` | 🔴 缺失 |
| struct | `CursorOptions` | 🔴 缺失 |
| struct | `CustomCursorImage` | 🔴 缺失 |
| struct | `CustomCursorUrl` | 🔴 缺失 |
| struct | `EnabledButtons` | 🔴 缺失 |
| struct | `InternalWindowState` | 🔴 缺失 |
| struct | `Monitor` | 🔴 缺失 |
| struct | `NormalizedWindowRef` | 🔴 缺失 |
| struct | `PrimaryMonitor` | 🔴 缺失 |
| struct | `PrimaryWindow` | 🔴 缺失 |
| struct | `RawHandleWrapper` | 🔴 缺失 |
| struct | `RawHandleWrapperHolder` | 🔴 缺失 |
| struct | `RequestRedraw` | 🔴 缺失 |
| struct | `ThreadLockedRawWindowHandleWrapper` | 🔴 缺失 |
| struct | `VideoMode` | 🔴 缺失 |
| struct | `Window` | ✅ |
| struct | `WindowBackendScaleFactorChanged` | 🔴 缺失 |
| struct | `WindowCloseRequested` | 🔴 缺失 |
| struct | `WindowClosed` | 🔴 缺失 |
| struct | `WindowClosing` | 🔴 缺失 |
| struct | `WindowCreated` | 🔴 缺失 |
| struct | `WindowDestroyed` | 🔴 缺失 |
| struct | `WindowFocused` | 🔴 缺失 |
| struct | `WindowMoved` | 🔴 缺失 |
| struct | `WindowOccluded` | 🔴 缺失 |
| struct | `WindowPlugin` | 🔴 缺失 |
| struct | `WindowResizeConstraints` | 🔴 缺失 |
| struct | `WindowResized` | 🔴 缺失 |
| struct | `WindowResolution` | 🔴 缺失 |
| struct | `WindowScaleFactorChanged` | 🔴 缺失 |
| struct | `WindowThemeChanged` | 🔴 缺失 |
| struct | `WindowWrapper` | 🔴 缺失 |
| enum | `AppLifecycle` | 🔴 缺失 |
| enum | `CompositeAlphaMode` | 🔴 缺失 |
| enum | `CursorGrabMode` | 🔴 缺失 |
| enum | `CursorIcon` | ✅ |
| enum | `CustomCursor` | 🔴 缺失 |
| enum | `ExitCondition` | 🔴 缺失 |
| enum | `FileDragAndDrop` | 🔴 缺失 |
| enum | `Ime` | 🔴 缺失 |
| enum | `MonitorSelection` | 🔴 缺失 |
| enum | `PresentMode` | 🔴 缺失 |
| enum | `ScreenEdge` | 🔴 缺失 |
| enum | `SystemCursorIcon` | 🔴 缺失 |
| enum | `VideoModeSelection` | 🔴 缺失 |
| enum | `WindowEvent` | ✅ |
| enum | `WindowLevel` | 🔴 缺失 |
| enum | `WindowMode` | ✅ |
| enum | `WindowPosition` | 🔴 缺失 |
| enum | `WindowRef` | 🔴 缺失 |
| enum | `WindowTheme` | 🔴 缺失 |

### time

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `Fixed` | 🔴 缺失 |
| struct | `Real` | 🔴 缺失 |
| struct | `Stopwatch` | ✅ |
| struct | `Time` | ✅ |
| struct | `TimePlugin` | 🔴 缺失 |
| struct | `TimeReceiver` | 🔴 缺失 |
| struct | `TimeSender` | 🔴 缺失 |
| struct | `TimeSystems` | 🔴 缺失 |
| struct | `Timer` | ✅ |
| struct | `Virtual` | 🔴 缺失 |
| enum | `TimeUpdateStrategy` | 🔴 缺失 |
| enum | `TimerMode` | ✅ |

### state

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `DespawnOnEnter` | ✅ |
| struct | `DespawnOnExit` | ✅ |
| struct | `EnterSchedules` | 🔴 缺失 |
| struct | `ExitSchedules` | 🔴 缺失 |
| struct | `OnEnter` | ✅ |
| struct | `OnExit` | ✅ |
| struct | `OnTransition` | ✅ |
| struct | `PreviousState` | 🔴 缺失 |
| struct | `ReflectFreelyMutableState` | 🔴 缺失 |
| struct | `ReflectFreelyMutableStateFns` | 🔴 缺失 |
| struct | `ReflectState` | 🔴 缺失 |
| struct | `ReflectStateFns` | 🔴 缺失 |
| struct | `State` | ✅ |
| struct | `StateTransition` | ✅ |
| struct | `StateTransitionEvent` | ✅ |
| struct | `StatesPlugin` | 🔴 缺失 |
| struct | `TransitionSchedules` | 🔴 缺失 |
| enum | `NextState` | 🔴 缺失 |
| enum | `StateTransitionSystems` | 🔴 缺失 |
| trait | `AppExtStates` | 🔴 缺失 |
| trait | `CommandsStatesExt` | 🔴 缺失 |
| trait | `ComputedStates` | 🔴 缺失 |
| trait | `FreelyMutableState` | 🔴 缺失 |
| trait | `StateScopedMessagesAppExt` | 🔴 缺失 |
| trait | `StateSet` | 🔴 缺失 |
| trait | `StateSetSealed` | 🔴 缺失 |
| trait | `States` | ✅ |
| trait | `SubStates` | 🔴 缺失 |

### asset

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `AddAsyncError` | 🔴 缺失 |
| struct | `AndroidAssetReader` | 🔴 缺失 |
| struct | `AssetChanged` | 🔴 缺失 |
| struct | `AssetChangedFetch` | 🔴 缺失 |
| struct | `AssetChangedState` | 🔴 缺失 |
| struct | `AssetEventSystems` | 🔴 缺失 |
| struct | `AssetHandleProvider` | 🔴 缺失 |
| struct | `AssetIndex` | 🔴 缺失 |
| struct | `AssetLoadFailedEvent` | 🔴 缺失 |
| struct | `AssetLoaderError` | 🔴 缺失 |
| struct | `AssetMeta` | 🔴 缺失 |
| struct | `AssetMetaMinimal` | 🔴 缺失 |
| struct | `AssetPath` | ✅ |
| struct | `AssetPlugin` | 🔴 缺失 |
| struct | `AssetProcessor` | 🔴 缺失 |
| struct | `AssetProcessorData` | 🔴 缺失 |
| struct | `AssetServer` | ✅ |
| struct | `AssetSource` | 🔴 缺失 |
| struct | `AssetSourceBuilder` | 🔴 缺失 |
| struct | `AssetSourceBuilders` | 🔴 缺失 |
| struct | `AssetSources` | 🔴 缺失 |
| struct | `AssetTrackingSystems` | 🔴 缺失 |
| struct | `Assets` | ✅ |
| struct | `AssetsMutIterator` | 🔴 缺失 |
| struct | `CoolText` | 🔴 缺失 |
| struct | `CoolTextLoader` | 🔴 缺失 |
| struct | `CoolTextRon` | 🔴 缺失 |
| struct | `Data` | 🔴 缺失 |
| struct | `Deferred` | 🔴 缺失 |
| struct | `Dir` | 🔴 缺失 |
| struct | `DirStream` | 🔴 缺失 |
| struct | `DynamicTyped` | 🔴 缺失 |
| struct | `EmbeddedAssetRegistry` | 🔴 缺失 |
| struct | `EmbeddedWatcher` | 🔴 缺失 |
| struct | `ErasedLoadedAsset` | 🔴 缺失 |
| struct | `FileAssetReader` | 🔴 缺失 |
| struct | `FileAssetWriter` | 🔴 缺失 |
| struct | `FileTransactionLogFactory` | 🔴 缺失 |
| struct | `FileWatcher` | 🔴 缺失 |
| struct | `GateOpener` | 🔴 缺失 |
| struct | `GatedReader` | 🔴 缺失 |
| struct | `HttpWasmAssetReader` | 🔴 缺失 |
| struct | `IdentityAssetTransformer` | 🔴 缺失 |
| struct | `Immediate` | 🔴 缺失 |
| struct | `LoadContext` | 🔴 缺失 |
| struct | `LoadTransformAndSave` | 🔴 缺失 |
| struct | `LoadTransformAndSaveSettings` | 🔴 缺失 |
| struct | `LoadedAsset` | 🔴 缺失 |
| struct | `LoadedFolder` | 🔴 缺失 |
| struct | `LoadedUntypedAsset` | 🔴 缺失 |
| struct | `Marker` | 🔴 缺失 |
| struct | `MemoryAssetReader` | 🔴 缺失 |
| struct | `MemoryAssetWriter` | 🔴 缺失 |
| struct | `MissingAssetLoaderForExtensionError` | 🔴 缺失 |
| struct | `MissingAssetLoaderForTypeIdError` | 🔴 缺失 |
| struct | `MissingAssetLoaderForTypeNameError` | 🔴 缺失 |
| struct | `MissingAssetSourceError` | 🔴 缺失 |
| struct | `MissingAssetWriterError` | 🔴 缺失 |
| struct | `MissingHandleProviderError` | 🔴 缺失 |
| struct | `MissingProcessedAssetReaderError` | 🔴 缺失 |
| struct | `MissingProcessedAssetWriterError` | 🔴 缺失 |
| struct | `NestedLoader` | 🔴 缺失 |
| struct | `ProcessContext` | 🔴 缺失 |
| struct | `ProcessDependencyInfo` | 🔴 缺失 |
| struct | `ProcessedInfo` | 🔴 缺失 |
| struct | `ProcessedInfoMinimal` | 🔴 缺失 |
| struct | `ProcessorAssetInfos` | 🔴 缺失 |
| struct | `ReaderRequiredFeatures` | 🔴 缺失 |
| struct | `ReflectAsset` | 🔴 缺失 |
| struct | `ReflectHandle` | 🔴 缺失 |
| struct | `RenderAssetUsages` | 🔴 缺失 |
| struct | `SavedAsset` | 🔴 缺失 |
| struct | `SliceReader` | 🔴 缺失 |
| struct | `StaticTyped` | 🔴 缺失 |
| struct | `StrongHandle` | 🔴 缺失 |
| struct | `StructTestAsset` | 🔴 缺失 |
| struct | `SubText` | 🔴 缺失 |
| struct | `TestAsset` | 🔴 缺失 |
| struct | `TransactionLockedReader` | 🔴 缺失 |
| struct | `TransformedAsset` | 🔴 缺失 |
| struct | `TransformedSubAsset` | 🔴 缺失 |
| struct | `TupleTestAsset` | 🔴 缺失 |
| struct | `UnknownTyped` | 🔴 缺失 |
| struct | `UnstableMemoryAssetReader` | 🔴 缺失 |
| struct | `UntypedAssetLoadFailedEvent` | 🔴 缺失 |
| struct | `VecReader` | 🔴 缺失 |
| struct | `WebAssetPlugin` | 🔴 缺失 |
| enum | `AssetAction` | 🔴 缺失 |
| enum | `AssetActionMinimal` | 🔴 缺失 |
| enum | `AssetEvent` | ✅ |
| enum | `AssetId` | 🔴 缺失 |
| enum | `AssetLoadError` | 🔴 缺失 |
| enum | `AssetMetaCheck` | 🔴 缺失 |
| enum | `AssetMode` | 🔴 缺失 |
| enum | `AssetReaderError` | 🔴 缺失 |
| enum | `AssetServerMode` | 🔴 缺失 |
| enum | `AssetSourceEvent` | 🔴 缺失 |
| enum | `AssetSourceId` | 🔴 缺失 |
| enum | `AssetWriterError` | 🔴 缺失 |
| enum | `CoolTextLoaderError` | 🔴 缺失 |
| enum | `DependencyLoadState` | 🔴 缺失 |
| enum | `DeserializeMetaError` | 🔴 缺失 |
| enum | `EnumTestAsset` | 🔴 缺失 |
| enum | `GetProcessorError` | 🔴 缺失 |
| enum | `Handle` | 🔴 缺失 |
| enum | `InitializeError` | 🔴 缺失 |
| enum | `InvalidGenerationError` | 🔴 缺失 |
| enum | `LoadDirectError` | 🔴 缺失 |
| enum | `LoadState` | ✅ |
| enum | `LogEntry` | 🔴 缺失 |
| enum | `LogEntryError` | 🔴 缺失 |
| enum | `ParseAssetPathError` | 🔴 缺失 |
| enum | `ProcessError` | 🔴 缺失 |
| enum | `ProcessResult` | 🔴 缺失 |
| enum | `ProcessStatus` | 🔴 缺失 |
| enum | `ProcessorState` | 🔴 缺失 |
| enum | `ReadAssetBytesError` | 🔴 缺失 |
| enum | `ReadLogError` | 🔴 缺失 |
| enum | `RecursiveDependencyLoadState` | 🔴 缺失 |
| enum | `SeekKind` | 🔴 缺失 |
| enum | `SetTransactionLogFactoryError` | 🔴 缺失 |
| enum | `UnapprovedPathMode` | 🔴 缺失 |
| enum | `UnsupportedReaderFeature` | 🔴 缺失 |
| enum | `UntypedAssetConversionError` | 🔴 缺失 |
| enum | `UntypedAssetId` | 🔴 缺失 |
| enum | `UntypedAssetIdConversionError` | 🔴 缺失 |
| enum | `UntypedHandle` | 🔴 缺失 |
| enum | `ValidateLogError` | 🔴 缺失 |
| enum | `Value` | 🔴 缺失 |
| enum | `WaitForAssetError` | 🔴 缺失 |
| enum | `WebAssetReader` | 🔴 缺失 |
| enum | `WriteDefaultMetaError` | 🔴 缺失 |
| trait | `AsAssetId` | 🔴 缺失 |
| trait | `Asset` | ✅ |
| trait | `AssetApp` | 🔴 缺失 |
| trait | `AssetLoader` | 🔴 缺失 |
| trait | `AssetMetaDyn` | 🔴 缺失 |
| trait | `AssetReader` | 🔴 缺失 |
| trait | `AssetReaderFuture` | 🔴 缺失 |
| trait | `AssetSaver` | 🔴 缺失 |
| trait | `AssetTransformer` | 🔴 缺失 |
| trait | `AssetWatcher` | 🔴 缺失 |
| trait | `AssetWriter` | 🔴 缺失 |
| trait | `DirectAssetAccessExt` | 🔴 缺失 |
| trait | `ErasedAssetLoader` | 🔴 缺失 |
| trait | `ErasedAssetReader` | 🔴 缺失 |
| trait | `ErasedAssetSaver` | 🔴 缺失 |
| trait | `ErasedAssetWriter` | 🔴 缺失 |
| trait | `ErasedProcessor` | 🔴 缺失 |
| trait | `GetAssetServer` | 🔴 缺失 |
| trait | `Mode` | 🔴 缺失 |
| trait | `Process` | 🔴 缺失 |
| trait | `ProcessorTransactionLog` | 🔴 缺失 |
| trait | `ProcessorTransactionLogFactory` | 🔴 缺失 |
| trait | `Reader` | 🔴 缺失 |
| trait | `Settings` | 🔴 缺失 |
| trait | `Typing` | 🔴 缺失 |
| trait | `VisitAssetDependencies` | 🔴 缺失 |

### sprite

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `Anchor` | 🔴 缺失 |
| struct | `BorderRect` | 🔴 缺失 |
| struct | `Sprite` | ✅ |
| struct | `SpritePickingCamera` | 🔴 缺失 |
| struct | `SpritePickingPlugin` | 🔴 缺失 |
| struct | `SpritePickingSettings` | 🔴 缺失 |
| struct | `SpritePlugin` | 🔴 缺失 |
| struct | `Text2d` | 🔴 缺失 |
| struct | `Text2dShadow` | 🔴 缺失 |
| struct | `TextureSlice` | 🔴 缺失 |
| struct | `TextureSlicer` | 🔴 缺失 |
| enum | `SliceScaleMode` | 🔴 缺失 |
| enum | `SpriteImageMode` | 🔴 缺失 |
| enum | `SpritePickingMode` | 🔴 缺失 |
| enum | `SpriteScalingMode` | 🔴 缺失 |
| enum | `SpriteSystems` | 🔴 缺失 |

### ui

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `AngularColorStop` | 🔴 缺失 |
| struct | `BackgroundColor` | ✅ |
| struct | `BackgroundGradient` | 🔴 缺失 |
| struct | `BorderColor` | ✅ |
| struct | `BorderGradient` | 🔴 缺失 |
| struct | `BorderRadius` | ✅ |
| struct | `BoxShadow` | 🔴 缺失 |
| struct | `Button` | 🔴 缺失 |
| struct | `CalculatedClip` | 🔴 缺失 |
| struct | `Checkable` | 🔴 缺失 |
| struct | `Checked` | 🔴 缺失 |
| struct | `ColorStop` | 🔴 缺失 |
| struct | `ComputedNode` | ✅ |
| struct | `ComputedUiRenderTargetInfo` | 🔴 缺失 |
| struct | `ComputedUiTargetCamera` | 🔴 缺失 |
| struct | `ConicGradient` | 🔴 缺失 |
| struct | `ContentSize` | 🔴 缺失 |
| struct | `DefaultUiCamera` | 🔴 缺失 |
| struct | `FixedMeasure` | 🔴 缺失 |
| struct | `GhostNode` | 🔴 缺失 |
| struct | `GlobalZIndex` | 🔴 缺失 |
| struct | `GridPlacement` | 🔴 缺失 |
| struct | `GridTrack` | 🔴 缺失 |
| struct | `IgnoreScroll` | 🔴 缺失 |
| struct | `ImageMeasure` | 🔴 缺失 |
| struct | `ImageNode` | 🔴 缺失 |
| struct | `ImageNodeSize` | 🔴 缺失 |
| struct | `InteractionDisabled` | 🔴 缺失 |
| struct | `IsDefaultUiCamera` | 🔴 缺失 |
| struct | `Label` | 🔴 缺失 |
| struct | `LayoutConfig` | 🔴 缺失 |
| struct | `LayoutContext` | 🔴 缺失 |
| struct | `LayoutNode` | 🔴 缺失 |
| struct | `LinearGradient` | 🔴 缺失 |
| struct | `MeasureArgs` | 🔴 缺失 |
| struct | `Node` | ✅ |
| struct | `NodeQuery` | 🔴 缺失 |
| struct | `Outline` | 🔴 缺失 |
| struct | `Overflow` | 🔴 缺失 |
| struct | `OverflowClipMargin` | 🔴 缺失 |
| struct | `OverrideClip` | 🔴 缺失 |
| struct | `Pressed` | 🔴 缺失 |
| struct | `RadialGradient` | 🔴 缺失 |
| struct | `RelativeCursorPosition` | 🔴 缺失 |
| struct | `RepeatedGridTrack` | 🔴 缺失 |
| struct | `ResolvedBorderRadius` | 🔴 缺失 |
| struct | `ScrollPosition` | 🔴 缺失 |
| struct | `ShadowStyle` | 🔴 缺失 |
| struct | `State` | 🔴 缺失 |
| struct | `Text` | 🔴 缺失 |
| struct | `TextMeasure` | 🔴 缺失 |
| struct | `TextNodeFlags` | 🔴 缺失 |
| struct | `TextShadow` | 🔴 缺失 |
| struct | `UiChildren` | 🔴 缺失 |
| struct | `UiChildrenIter` | 🔴 缺失 |
| struct | `UiGlobalTransform` | 🔴 缺失 |
| struct | `UiPickingCamera` | 🔴 缺失 |
| struct | `UiPickingPlugin` | 🔴 缺失 |
| struct | `UiPickingSettings` | 🔴 缺失 |
| struct | `UiPlugin` | 🔴 缺失 |
| struct | `UiPosition` | 🔴 缺失 |
| struct | `UiRect` | ✅ |
| struct | `UiRootNodes` | 🔴 缺失 |
| struct | `UiScale` | 🔴 缺失 |
| struct | `UiStack` | 🔴 缺失 |
| struct | `UiSurface` | 🔴 缺失 |
| struct | `UiTargetCamera` | 🔴 缺失 |
| struct | `UiTransform` | 🔴 缺失 |
| struct | `Val2` | 🔴 缺失 |
| struct | `ViewportNode` | 🔴 缺失 |
| struct | `ZIndex` | 🔴 缺失 |
| enum | `AlignContent` | ✅ |
| enum | `AlignItems` | ✅ |
| enum | `AlignSelf` | ✅ |
| enum | `BoxSizing` | 🔴 缺失 |
| enum | `Display` | ✅ |
| enum | `FlexDirection` | ✅ |
| enum | `FlexWrap` | ✅ |
| enum | `FocusPolicy` | 🔴 缺失 |
| enum | `Gradient` | 🔴 缺失 |
| enum | `GridAutoFlow` | 🔴 缺失 |
| enum | `GridPlacementError` | 🔴 缺失 |
| enum | `GridTrackRepetition` | 🔴 缺失 |
| enum | `Interaction` | ✅ |
| enum | `InterpolationColorSpace` | 🔴 缺失 |
| enum | `JustifyContent` | ✅ |
| enum | `JustifyItems` | 🔴 缺失 |
| enum | `JustifySelf` | 🔴 缺失 |
| enum | `LayoutError` | 🔴 缺失 |
| enum | `MaxTrackSizingFunction` | 🔴 缺失 |
| enum | `MinTrackSizingFunction` | 🔴 缺失 |
| enum | `NodeImageMode` | 🔴 缺失 |
| enum | `NodeMeasure` | 🔴 缺失 |
| enum | `OverflowAxis` | 🔴 缺失 |
| enum | `OverflowClipBox` | 🔴 缺失 |
| enum | `PositionType` | ✅ |
| enum | `RadialGradientShape` | 🔴 缺失 |
| enum | `UiSystems` | 🔴 缺失 |
| enum | `Val` | 🔴 缺失 |
| enum | `ValArithmeticError` | 🔴 缺失 |
| enum | `ValParseError` | 🔴 缺失 |
| trait | `InColorSpace` | 🔴 缺失 |
| trait | `Measure` | 🔴 缺失 |
| trait | `ValNum` | 🔴 缺失 |

### text

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `ComputedTextBlock` | 🔴 缺失 |
| struct | `CosmicBuffer` | 🔴 缺失 |
| struct | `CosmicFontSystem` | 🔴 缺失 |
| struct | `Font` | ✅ |
| struct | `FontAtlas` | 🔴 缺失 |
| struct | `FontAtlasKey` | 🔴 缺失 |
| struct | `FontAtlasSet` | 🔴 缺失 |
| struct | `FontFaceInfo` | 🔴 缺失 |
| struct | `FontFeatureTag` | 🔴 缺失 |
| struct | `FontFeatures` | 🔴 缺失 |
| struct | `FontFeaturesBuilder` | 🔴 缺失 |
| struct | `FontLoader` | 🔴 缺失 |
| struct | `FontWeight` | 🔴 缺失 |
| struct | `FontWidth` | 🔴 缺失 |
| struct | `GlyphAtlasInfo` | 🔴 缺失 |
| struct | `GlyphAtlasLocation` | 🔴 缺失 |
| struct | `PositionedGlyph` | 🔴 缺失 |
| struct | `RunGeometry` | 🔴 缺失 |
| struct | `Strikethrough` | 🔴 缺失 |
| struct | `StrikethroughColor` | 🔴 缺失 |
| struct | `SwashCache` | 🔴 缺失 |
| struct | `Text2dUpdateSystems` | 🔴 缺失 |
| struct | `TextBackgroundColor` | 🔴 缺失 |
| struct | `TextBounds` | ✅ |
| struct | `TextColor` | 🔴 缺失 |
| struct | `TextEntity` | 🔴 缺失 |
| struct | `TextFont` | 🔴 缺失 |
| struct | `TextIterScratch` | 🔴 缺失 |
| struct | `TextLayout` | ✅ |
| struct | `TextLayoutInfo` | 🔴 缺失 |
| struct | `TextMeasureInfo` | 🔴 缺失 |
| struct | `TextPipeline` | 🔴 缺失 |
| struct | `TextPlugin` | 🔴 缺失 |
| struct | `TextReader` | 🔴 缺失 |
| struct | `TextSpan` | 🔴 缺失 |
| struct | `TextSpanIter` | 🔴 缺失 |
| struct | `TextWriter` | 🔴 缺失 |
| struct | `Underline` | 🔴 缺失 |
| struct | `UnderlineColor` | 🔴 缺失 |
| enum | `FontHinting` | 🔴 缺失 |
| enum | `FontLoaderError` | 🔴 缺失 |
| enum | `FontSmoothing` | 🔴 缺失 |
| enum | `FontSource` | 🔴 缺失 |
| enum | `FontStyle` | 🔴 缺失 |
| enum | `Justify` | 🔴 缺失 |
| enum | `LineBreak` | 🔴 缺失 |
| enum | `LineHeight` | 🔴 缺失 |
| enum | `TextError` | 🔴 缺失 |
| trait | `TextRoot` | 🔴 缺失 |
| trait | `TextSpanAccess` | 🔴 缺失 |
| trait | `TextSpanComponent` | 🔴 缺失 |

### reflect

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `AccessError` | 🔴 缺失 |
| struct | `Arg` | 🔴 缺失 |
| struct | `ArgCount` | 🔴 缺失 |
| struct | `ArgCountIter` | 🔴 缺失 |
| struct | `ArgCountOutOfBoundsError` | 🔴 缺失 |
| struct | `ArgInfo` | 🔴 缺失 |
| struct | `ArgList` | 🔴 缺失 |
| struct | `ArgumentSignature` | 🔴 缺失 |
| struct | `ArrayInfo` | 🔴 缺失 |
| struct | `ArrayIter` | 🔴 缺失 |
| struct | `AutomaticReflectRegistrations` | 🔴 缺失 |
| struct | `ConstParamInfo` | 🔴 缺失 |
| struct | `CustomAttributes` | 🔴 缺失 |
| struct | `DynamicArray` | 🔴 缺失 |
| struct | `DynamicEnum` | 🔴 缺失 |
| struct | `DynamicFunction` | 🔴 缺失 |
| struct | `DynamicFunctionMut` | 🔴 缺失 |
| struct | `DynamicList` | 🔴 缺失 |
| struct | `DynamicMap` | 🔴 缺失 |
| struct | `DynamicSet` | 🔴 缺失 |
| struct | `DynamicStruct` | 🔴 缺失 |
| struct | `DynamicTuple` | 🔴 缺失 |
| struct | `DynamicTupleStruct` | 🔴 缺失 |
| struct | `EnumInfo` | 🔴 缺失 |
| struct | `FieldIter` | 🔴 缺失 |
| struct | `FunctionInfo` | 🔴 缺失 |
| struct | `FunctionRegistry` | 🔴 缺失 |
| struct | `FunctionRegistryArc` | 🔴 缺失 |
| struct | `GenericTypeCell` | 🔴 缺失 |
| struct | `Generics` | 🔴 缺失 |
| struct | `ListInfo` | 🔴 缺失 |
| struct | `ListIter` | 🔴 缺失 |
| struct | `MapInfo` | 🔴 缺失 |
| struct | `MyType` | 🔴 缺失 |
| struct | `NamedField` | 🔴 缺失 |
| struct | `NonGenericTypeCell` | 🔴 缺失 |
| struct | `OffsetAccess` | 🔴 缺失 |
| struct | `OpaqueInfo` | 🔴 缺失 |
| struct | `ParseError` | 🔴 缺失 |
| struct | `ParsedPath` | 🔴 缺失 |
| struct | `PrettyPrintFunctionInfo` | 🔴 缺失 |
| struct | `PrettyPrintSignatureInfo` | 🔴 缺失 |
| struct | `ReflectDefault` | 🔴 缺失 |
| struct | `ReflectDeserialize` | 🔴 缺失 |
| struct | `ReflectDeserializeWithRegistry` | 🔴 缺失 |
| struct | `ReflectDeserializer` | 🔴 缺失 |
| struct | `ReflectFromPtr` | 🔴 缺失 |
| struct | `ReflectFromReflect` | 🔴 缺失 |
| struct | `ReflectKindMismatchError` | 🔴 缺失 |
| struct | `ReflectSerialize` | 🔴 缺失 |
| struct | `ReflectSerializeWithRegistry` | 🔴 缺失 |
| struct | `ReflectSerializer` | 🔴 缺失 |
| struct | `ReturnInfo` | 🔴 缺失 |
| struct | `SerializationData` | 🔴 缺失 |
| struct | `SetInfo` | 🔴 缺失 |
| struct | `Signature` | 🔴 缺失 |
| struct | `SignatureInfo` | 🔴 缺失 |
| struct | `SkippedField` | 🔴 缺失 |
| struct | `StructInfo` | 🔴 缺失 |
| struct | `StructVariantInfo` | 🔴 缺失 |
| struct | `TheirInner` | 🔴 缺失 |
| struct | `TheirOuter` | 🔴 缺失 |
| struct | `TheirType` | 🔴 缺失 |
| struct | `TupleFieldIter` | 🔴 缺失 |
| struct | `TupleInfo` | 🔴 缺失 |
| struct | `TupleStructFieldIter` | 🔴 缺失 |
| struct | `TupleStructInfo` | 🔴 缺失 |
| struct | `TupleVariantInfo` | 🔴 缺失 |
| struct | `Type` | ✅ |
| struct | `TypeParamInfo` | 🔴 缺失 |
| struct | `TypePathComponent` | 🔴 缺失 |
| struct | `TypePathTable` | 🔴 缺失 |
| struct | `TypeRegistration` | 🔴 缺失 |
| struct | `TypeRegistrationDeserializer` | 🔴 缺失 |
| struct | `TypeRegistry` | ✅ |
| struct | `TypeRegistryArc` | 🔴 缺失 |
| struct | `TypedReflectDeserializer` | 🔴 缺失 |
| struct | `TypedReflectSerializer` | 🔴 缺失 |
| struct | `UnitVariantInfo` | 🔴 缺失 |
| struct | `UnnamedField` | 🔴 缺失 |
| struct | `VariantFieldIter` | 🔴 缺失 |
| struct | `Vector2` | 🔴 缺失 |
| enum | `Access` | 🔴 缺失 |
| enum | `AccessErrorKind` | 🔴 缺失 |
| enum | `ApplyError` | 🔴 缺失 |
| enum | `ArgError` | 🔴 缺失 |
| enum | `ArgId` | 🔴 缺失 |
| enum | `ArgValue` | 🔴 缺失 |
| enum | `DynamicVariant` | 🔴 缺失 |
| enum | `FieldId` | 🔴 缺失 |
| enum | `FunctionError` | 🔴 缺失 |
| enum | `FunctionOverloadError` | 🔴 缺失 |
| enum | `FunctionRegistrationError` | 🔴 缺失 |
| enum | `GenericInfo` | 🔴 缺失 |
| enum | `Ownership` | 🔴 缺失 |
| enum | `ReflectCloneError` | 🔴 缺失 |
| enum | `ReflectKind` | 🔴 缺失 |
| enum | `ReflectMut` | 🔴 缺失 |
| enum | `ReflectOwned` | 🔴 缺失 |
| enum | `ReflectPathError` | 🔴 缺失 |
| enum | `ReflectRef` | 🔴 缺失 |
| enum | `Return` | 🔴 缺失 |
| enum | `Serializable` | 🔴 缺失 |
| enum | `TheirInner` | 🔴 缺失 |
| enum | `TheirOuter` | 🔴 缺失 |
| enum | `TheirType` | 🔴 缺失 |
| enum | `TypeInfo` | ✅ |
| enum | `TypeInfoError` | 🔴 缺失 |
| enum | `VariantField` | 🔴 缺失 |
| enum | `VariantInfo` | 🔴 缺失 |
| enum | `VariantInfoError` | 🔴 缺失 |
| enum | `VariantType` | 🔴 缺失 |
| trait | `Array` | 🔴 缺失 |
| trait | `DeserializeWithRegistry` | 🔴 缺失 |
| trait | `DynamicTypePath` | 🔴 缺失 |
| trait | `DynamicTyped` | 🔴 缺失 |
| trait | `Enum` | 🔴 缺失 |
| trait | `FromArg` | 🔴 缺失 |
| trait | `FromReflect` | 🔴 缺失 |
| trait | `FromType` | 🔴 缺失 |
| trait | `Function` | 🔴 缺失 |
| trait | `GetField` | 🔴 缺失 |
| trait | `GetOwnership` | 🔴 缺失 |
| trait | `GetPath` | 🔴 缺失 |
| trait | `GetTupleField` | 🔴 缺失 |
| trait | `GetTupleStructField` | 🔴 缺失 |
| trait | `GetTypeRegistration` | 🔴 缺失 |
| trait | `IntoFunction` | 🔴 缺失 |
| trait | `IntoFunctionMut` | 🔴 缺失 |
| trait | `IntoReturn` | 🔴 缺失 |
| trait | `Is` | 🔴 缺失 |
| trait | `List` | 🔴 缺失 |
| trait | `Map` | 🔴 缺失 |
| trait | `MaybeTyped` | 🔴 缺失 |
| trait | `MyTrait` | 🔴 缺失 |
| trait | `PartialReflect` | 🔴 缺失 |
| trait | `Reflect` | ✅ |
| trait | `ReflectDeserializerProcessor` | 🔴 缺失 |
| trait | `ReflectFn` | 🔴 缺失 |
| trait | `ReflectFnMut` | 🔴 缺失 |
| trait | `ReflectPath` | 🔴 缺失 |
| trait | `ReflectRemote` | 🔴 缺失 |
| trait | `ReflectSerializerProcessor` | 🔴 缺失 |
| trait | `Reflectable` | 🔴 缺失 |
| trait | `RegisterForReflection` | 🔴 缺失 |
| trait | `Sealed` | 🔴 缺失 |
| trait | `SerializeWithRegistry` | 🔴 缺失 |
| trait | `Set` | 🔴 缺失 |
| trait | `Struct` | ✅ |
| trait | `Tuple` | 🔴 缺失 |
| trait | `TupleStruct` | 🔴 缺失 |
| trait | `TypeData` | 🔴 缺失 |
| trait | `TypePath` | 🔴 缺失 |
| trait | `Typed` | 🔴 缺失 |
| trait | `TypedFunction` | 🔴 缺失 |
| trait | `TypedProperty` | 🔴 缺失 |

### tasks

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `Chain` | 🔴 缺失 |
| struct | `Cloned` | 🔴 缺失 |
| struct | `Copied` | 🔴 缺失 |
| struct | `Cycle` | 🔴 缺失 |
| struct | `Executor` | 🔴 缺失 |
| struct | `Filter` | 🔴 缺失 |
| struct | `FilterMap` | 🔴 缺失 |
| struct | `FlatMap` | 🔴 缺失 |
| struct | `Flatten` | 🔴 缺失 |
| struct | `Fuse` | 🔴 缺失 |
| struct | `Inspect` | 🔴 缺失 |
| struct | `LocalExecutor` | 🔴 缺失 |
| struct | `Map` | 🔴 缺失 |
| struct | `Scope` | 🔴 缺失 |
| struct | `Task` | ✅ |
| struct | `TaskPool` | ✅ |
| struct | `TaskPoolBuilder` | 🔴 缺失 |
| struct | `ThreadExecutor` | 🔴 缺失 |
| struct | `ThreadExecutorTicker` | 🔴 缺失 |
| trait | `ConditionalSend` | 🔴 缺失 |
| trait | `ConditionalSendFuture` | 🔴 缺失 |
| trait | `MaybeSend` | 🔴 缺失 |
| trait | `MaybeSync` | 🔴 缺失 |
| trait | `ParallelIterator` | 🔴 缺失 |
| trait | `ParallelSlice` | 🔴 缺失 |
| trait | `ParallelSliceMut` | 🔴 缺失 |

### ptr

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `Aligned` | ✅ |
| struct | `ConstNonNull` | 🔴 缺失 |
| struct | `MovingPtr` | ✅ |
| struct | `OwningPtr` | ✅ |
| struct | `Parent` | 🔴 缺失 |
| struct | `Ptr` | ✅ |
| struct | `PtrMut` | ✅ |
| struct | `ThinSlicePtr` | ✅ |
| struct | `Unaligned` | ✅ |
| trait | `IsAligned` | ✅ |
| trait | `Sealed` | ✅ |
| trait | `SealedUnsafeCell` | ✅ |
| trait | `UnsafeCellDeref` | ✅ |

### utils

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `DebugName` | 🔴 缺失 |
| struct | `OnDrop` | 🔴 缺失 |
| struct | `OnceFlag` | ✅ |
| struct | `Parallel` | 🔴 缺失 |
| trait | `PreHashMapExt` | 🔴 缺失 |
| trait | `TypeIdMapExt` | 🔴 缺失 |

### diagnostic

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `Diagnostic` | ✅ |
| struct | `DiagnosticMeasurement` | 🔴 缺失 |
| struct | `DiagnosticPath` | ✅ |
| struct | `Diagnostics` | ✅ |
| struct | `DiagnosticsPlugin` | 🔴 缺失 |
| struct | `DiagnosticsStore` | ✅ |
| struct | `EntityCountDiagnosticsPlugin` | ✅ |
| struct | `FrameCount` | 🔴 缺失 |
| struct | `FrameCountPlugin` | 🔴 缺失 |
| struct | `FrameTimeDiagnosticsPlugin` | ✅ |
| struct | `LogDiagnosticsPlugin` | ✅ |
| struct | `LogDiagnosticsState` | 🔴 缺失 |
| struct | `SystemInfo` | 🔴 缺失 |
| struct | `SystemInformationDiagnosticsPlugin` | 🔴 缺失 |
| trait | `RegisterDiagnostic` | 🔴 缺失 |

### log

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `LogPlugin` | 🔴 缺失 |

### image

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `CompressedImageFormatSupport` | 🔴 缺失 |
| struct | `CompressedImageFormats` | 🔴 缺失 |
| struct | `CompressedImageSaver` | 🔴 缺失 |
| struct | `DynamicTextureAtlasBuilder` | 🔴 缺失 |
| struct | `ExrTextureLoader` | 🔴 缺失 |
| struct | `ExrTextureLoaderSettings` | 🔴 缺失 |
| struct | `FileTextureError` | 🔴 缺失 |
| struct | `HdrTextureLoader` | 🔴 缺失 |
| struct | `HdrTextureLoaderSettings` | 🔴 缺失 |
| struct | `Image` | ✅ |
| struct | `ImageLoader` | ✅ |
| struct | `ImageLoaderSettings` | 🔴 缺失 |
| struct | `ImagePlugin` | 🔴 缺失 |
| struct | `ImageSamplerDescriptor` | 🔴 缺失 |
| struct | `SerializedImage` | 🔴 缺失 |
| struct | `TextureAtlas` | 🔴 缺失 |
| struct | `TextureAtlasBuilder` | 🔴 缺失 |
| struct | `TextureAtlasLayout` | 🔴 缺失 |
| struct | `TextureAtlasPlugin` | 🔴 缺失 |
| struct | `TextureAtlasSources` | 🔴 缺失 |
| enum | `CompressedImageSaverError` | 🔴 缺失 |
| enum | `DataFormat` | 🔴 缺失 |
| enum | `DynamicTextureAtlasBuilderError` | 🔴 缺失 |
| enum | `ExrTextureLoaderError` | 🔴 缺失 |
| enum | `HdrTextureLoaderError` | 🔴 缺失 |
| enum | `ImageAddressMode` | 🔴 缺失 |
| enum | `ImageArrayLayout` | 🔴 缺失 |
| enum | `ImageCompareFunction` | 🔴 缺失 |
| enum | `ImageFilterMode` | 🔴 缺失 |
| enum | `ImageFormat` | 🔴 缺失 |
| enum | `ImageFormatSetting` | 🔴 缺失 |
| enum | `ImageLoaderError` | 🔴 缺失 |
| enum | `ImageSampler` | 🔴 缺失 |
| enum | `ImageSamplerBorderColor` | 🔴 缺失 |
| enum | `ImageType` | 🔴 缺失 |
| enum | `IntoDynamicImageError` | 🔴 缺失 |
| enum | `TextureAccessError` | 🔴 缺失 |
| enum | `TextureAtlasBuilderError` | 🔴 缺失 |
| enum | `TextureError` | 🔴 缺失 |
| enum | `TextureReinterpretationError` | 🔴 缺失 |
| enum | `TranscodeFormat` | 🔴 缺失 |
| trait | `BevyDefault` | 🔴 缺失 |
| trait | `TextureFormatPixelInfo` | 🔴 缺失 |
| trait | `TextureSrgbViewFormats` | 🔴 缺失 |
| trait | `ToExtents` | 🔴 缺失 |
| trait | `Volume` | 🔴 缺失 |

### camera

| 类型 | API 名称 | 状态 |
|------|---------|------|
| struct | `Aabb` | 🔴 缺失 |
| struct | `Camera` | ✅ |
| struct | `Camera2d` | ✅ |
| struct | `Camera3d` | ✅ |
| struct | `Camera3dDepthTextureUsage` | 🔴 缺失 |
| struct | `CameraMainTextureUsages` | 🔴 缺失 |
| struct | `CameraPlugin` | 🔴 缺失 |
| struct | `CameraProjectionPlugin` | 🔴 缺失 |
| struct | `CameraUpdateSystems` | 🔴 缺失 |
| struct | `CascadesFrusta` | 🔴 缺失 |
| struct | `CascadesVisibleEntities` | 🔴 缺失 |
| struct | `ClearColor` | 🔴 缺失 |
| struct | `ComputedCameraValues` | 🔴 缺失 |
| struct | `CubeMapFace` | 🔴 缺失 |
| struct | `CubemapFrusta` | 🔴 缺失 |
| struct | `CubemapVisibleEntities` | 🔴 缺失 |
| struct | `CustomProjection` | 🔴 缺失 |
| struct | `Exposure` | 🔴 缺失 |
| struct | `Frustum` | ✅ |
| struct | `HalfSpace` | 🔴 缺失 |
| struct | `ImageRenderTarget` | 🔴 缺失 |
| struct | `InheritedVisibility` | 🔴 缺失 |
| struct | `MainPassResolutionOverride` | 🔴 缺失 |
| struct | `ManualTextureViewHandle` | 🔴 缺失 |
| struct | `NoAutoAabb` | 🔴 缺失 |
| struct | `NoCpuCulling` | 🔴 缺失 |
| struct | `NoFrustumCulling` | 🔴 缺失 |
| struct | `OrthographicProjection` | ✅ |
| struct | `PerspectiveProjection` | ✅ |
| struct | `PhysicalCameraParameters` | 🔴 缺失 |
| struct | `RenderLayers` | 🔴 缺失 |
| struct | `RenderTargetInfo` | 🔴 缺失 |
| struct | `Sphere` | 🔴 缺失 |
| struct | `SubCameraView` | 🔴 缺失 |
| struct | `ViewVisibility` | 🔴 缺失 |
| struct | `Viewport` | 🔴 缺失 |
| struct | `VisibilityClass` | 🔴 缺失 |
| struct | `VisibilityPlugin` | 🔴 缺失 |
| struct | `VisibilityRange` | 🔴 缺失 |
| struct | `VisibilityRangePlugin` | 🔴 缺失 |
| struct | `VisibleEntities` | 🔴 缺失 |
| struct | `VisibleEntityRanges` | 🔴 缺失 |
| struct | `VisibleMeshEntities` | 🔴 缺失 |
| enum | `Camera3dDepthLoadOp` | 🔴 缺失 |
| enum | `CameraOutputMode` | 🔴 缺失 |
| enum | `ClearColorConfig` | 🔴 缺失 |
| enum | `CubemapLayout` | 🔴 缺失 |
| enum | `MsaaWriteback` | 🔴 缺失 |
| enum | `NormalizedRenderTarget` | 🔴 缺失 |
| enum | `Projection` | 🔴 缺失 |
| enum | `RenderTarget` | 🔴 缺失 |
| enum | `ScalingMode` | 🔴 缺失 |
| enum | `ScreenSpaceTransmissionQuality` | 🔴 缺失 |
| enum | `ViewportConversionError` | 🔴 缺失 |
| enum | `Visibility` | 🔴 缺失 |
| enum | `VisibilitySystems` | 🔴 缺失 |
| trait | `CameraProjection` | 🔴 缺失 |
| trait | `DynCameraProjection` | 🔴 缺失 |
| trait | `MeshAabb` | 🔴 缺失 |
| trait | `SetViewVisibility` | 🔴 缺失 |

---

## 图例

- ✅ = 已实现
- 🔴 = 缺失 (需要新增)

*注意: 此对比仅检查类型名称是否存在，不验证方法签名是否完全一致。*
*为避免文件过大，每个类别最多显示100个struct和50个enum。*
