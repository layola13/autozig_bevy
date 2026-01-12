# Bevy ECS vs AutoZig ECS API Comparison

**Total Bevy Public Items:** 2539
**Total Missing Items:** 1761
**Completion:** 30.64%

### `archetype.rs`

- [ ] Function `component_count`
- [ ] Function `component_index`
- [ ] Function `edges`
- [ ] Function `empty`
- [ ] Function `entities_with_location`
- [ ] Function `entity_table_row`
- [ ] Function `get_archetype_after_bundle_insert`
- [ ] Function `get_archetype_after_bundle_remove`
- [ ] Function `get_archetype_after_bundle_take`
- [ ] Function `get_storage_type`
- [ ] Function `has_add_hook`
- [ ] Function `has_add_observer`
- [ ] Function `has_despawn_hook`
- [ ] Function `has_despawn_observer`
- [ ] Function `has_insert_hook`
- [ ] Function `has_insert_observer`
- [ ] Function `has_remove_hook`
- [ ] Function `has_remove_observer`
- [ ] Function `has_replace_hook`
- [ ] Function `has_replace_observer`
- [ ] Function `index_u32`
- [ ] Function `iter_components`
- [ ] Function `sparse_set_components`
- [ ] Function `table_components`
- [ ] Struct `ArchetypeRow`
- [ ] Type `ComponentIndex`

### `batching.rs`

- [ ] Function `batches_per_thread`
- [ ] Function `calc_batch_size`
- [ ] Function `fixed`
- [ ] Function `max_batch_size`
- [ ] Function `min_batch_size`
- [ ] Struct `BatchingStrategy`

### `bundle/info.rs`

- [ ] Enum `InsertMode`
- [ ] Function `contributed_components`
- [ ] Function `explicit_components`
- [ ] Function `iter_contributed_components`
- [ ] Function `iter_explicit_components`
- [ ] Function `iter_required_components`
- [ ] Function `required_components`

### `bundle/mod.rs`

- [ ] Trait `DynamicBundle`

### `bundle/remove.rs`

- [ ] Function `empty_pre_remove`

### `bundle/spawner.rs`

- [ ] Function `reserve_storage`
- [ ] Function `spawn_at`

### `bundle/tests.rs`

- [ ] Struct `SparseA`
- [ ] Struct `SparseV`

### `change_detection/maybe_location.rs`

- [ ] Function `as_deref_mut`
- [ ] Function `as_deref`
- [ ] Function `as_mut`
- [ ] Function `as_ref`
- [ ] Function `assign`
- [ ] Function `caller`
- [ ] Function `copied`
- [ ] Function `copied`
- [ ] Function `into_option`
- [ ] Function `new_with_flattened`
- [ ] Function `new_with`
- [ ] Function `transpose`
- [ ] Function `unwrap_or_default`
- [ ] Function `zip`
- [ ] Struct `MaybeLocation`

### `change_detection/params.rs`

- [ ] Function `as_mut`
- [ ] Function `as_ref`
- [ ] Function `has_changed_since`
- [ ] Function `set_ticks`
- [ ] Function `set_ticks`
- [ ] Function `with_type`

### `change_detection/tick.rs`

- [ ] Function `present_tick`
- [ ] Function `set`
- [ ] Struct `ComponentTickCells`

### `change_detection/traits.rs`

- [ ] Function `as_deref_mut`
- [ ] Function `filter_map_unchanged`
- [ ] Function `try_map_unchanged`

### `component/clone.rs`

- [ ] Function `component_clone_ignore`
- [ ] Function `component_clone_via_clone`
- [ ] Function `component_clone_via_reflect`
- [ ] Function `global_default_fn`
- [ ] Function `reflect`
- [ ] Function `resolve`
- [ ] Struct `DefaultCloneBehaviorSpecialization`
- [ ] Type `ComponentCloneFn`

### `component/info.rs`

- [ ] Function `any_queued_mut`
- [ ] Function `any_queued`
- [ ] Function `any_registered`
- [ ] Function `clone_behavior`
- [ ] Function `get_descriptor`
- [ ] Function `get_info_unchecked`
- [ ] Function `get_name`
- [ ] Function `get_resource_id`
- [ ] Function `get_valid_id`
- [ ] Function `get_valid_resource_id`
- [ ] Function `is_id_valid`
- [ ] Function `is_send_and_sync`
- [ ] Function `iter_registered`
- [ ] Function `mutable`
- [ ] Function `mutable`
- [ ] Function `new_resource`
- [ ] Function `new_with_layout`
- [ ] Function `num_queued_mut`
- [ ] Function `num_queued`
- [ ] Function `num_registered`
- [ ] Function `relationship_accessor`
- [ ] Function `required_components`
- [ ] Function `resource_id`
- [ ] Function `valid_component_id`
- [ ] Function `valid_resource_id`

### `component/register.rs`

- [ ] Function `any_queued_mut`
- [ ] Function `apply_queued_registrations`
- [ ] Function `as_queued`
- [ ] Function `next_mut`
- [ ] Function `num_queued_mut`
- [ ] Function `peek_mut`
- [ ] Function `peek`
- [ ] Function `queue_register_component_with_descriptor`
- [ ] Function `queue_register_component`
- [ ] Function `queue_register_non_send`
- [ ] Function `queue_register_resource_with_descriptor`
- [ ] Function `queue_register_resource`
- [ ] Function `register_component_with_descriptor`
- [ ] Function `register_component`
- [ ] Function `register_non_send`
- [ ] Function `register_resource_with_descriptor`
- [ ] Function `register_resource`
- [ ] Struct `ComponentIds`
- [ ] Struct `QueuedComponents`

### `component/required.rs`

- [ ] Enum `RequiredComponentsError`
- [ ] Function `iter_ids`
- [ ] Function `register_required_by_id`
- [ ] Function `register_required_dynamic_with`
- [ ] Function `register_required`
- [ ] Struct `RequiredComponentConstructor`
- [ ] Struct `RequiredComponent`
- [ ] Struct `RequiredComponentsRegistrator`

### `entity/clone_entities.rs`

- [ ] Enum `EntityClonerFilter`
- [ ] Enum `FilterableId`
- [ ] Function `allow_by_ids_if_new`
- [ ] Function `allow_by_ids`
- [ ] Function `allow_if_new`
- [ ] Function `allow`
- [ ] Function `build_opt_in`
- [ ] Function `build_opt_out`
- [ ] Function `clone_entity_mapped`
- [ ] Function `component_info`
- [ ] Function `deny_by_ids`
- [ ] Function `deny`
- [ ] Function `entity_mapper`
- [ ] Function `insert_mode`
- [ ] Function `linked_cloning`
- [ ] Function `linked_cloning`
- [ ] Function `linked_cloning`
- [ ] Function `move_components`
- [ ] Function `moving`
- [ ] Function `override_clone_behavior_with_id`
- [ ] Function `override_clone_behavior`
- [ ] Function `ptr`
- [ ] Function `queue_deferred`
- [ ] Function `queue_entity_clone`
- [ ] Function `read_reflect`
- [ ] Function `remove_clone_behavior_override_with_id`
- [ ] Function `remove_clone_behavior_override`
- [ ] Function `source`
- [ ] Function `spawn_clone`
- [ ] Function `target_component_written`
- [ ] Function `type_registry`
- [ ] Function `with_default_clone_fn`
- [ ] Function `without_required_by_components`
- [ ] Function `without_required_components`
- [ ] Function `write_target_component_ptr`
- [ ] Function `write_target_component_reflect`
- [ ] Function `write_target_component`
- [ ] Struct `ComponentCloneCtx`
- [ ] Struct `OptIn`
- [ ] Struct `OptOut`
- [ ] Struct `ScalarType`
- [ ] Struct `SourceComponent`
- [ ] Struct `VectorType`
- [ ] Trait `Marker`

### `entity/entity_set.rs`

- [ ] Function `as_inner`
- [ ] Function `as_mut_inner`
- [ ] Function `from_entity_set_iterator`
- [ ] Function `from_iterator_unchecked`
- [ ] Function `system`
- [ ] Struct `Thing`
- [ ] Struct `UniqueEntityIter`

### `entity/hash_map.rs`

- [ ] Function `into_keys`
- [ ] Function `keys`
- [ ] Struct `EntityHashMap`
- [ ] Struct `IntoKeys`
- [ ] Struct `Keys`

### `entity/hash_set.rs`

- [ ] Function `extract_if`
- [ ] Struct `Drain`
- [ ] Struct `EntityHashSet`
- [ ] Struct `ExtractIf`
- [ ] Struct `IntoIter`
- [ ] Struct `Iter`

### `entity/index_map.rs`

- [ ] Function `as_boxed_inner`
- [ ] Function `as_inner`
- [ ] Function `as_mut_slice`
- [ ] Function `as_mut_slice`
- [ ] Function `as_slice`
- [ ] Function `as_slice`
- [ ] Function `as_slice`
- [ ] Function `as_slice`
- [ ] Function `as_slice`
- [ ] Function `first_mut`
- [ ] Function `from_boxed_slice_unchecked`
- [ ] Function `get_index_mut`
- [ ] Function `get_range_mut`
- [ ] Function `get_range_mut`
- [ ] Function `get_range`
- [ ] Function `get_range`
- [ ] Function `into_boxed_inner`
- [ ] Function `into_boxed_slice`
- [ ] Function `into_keys`
- [ ] Function `into_keys`
- [ ] Function `into_slice`
- [ ] Function `into_values`
- [ ] Function `iter_mut`
- [ ] Function `iter_mut`
- [ ] Function `keys`
- [ ] Function `keys`
- [ ] Function `last_mut`
- [ ] Function `new_mut`
- [ ] Function `split_at_mut`
- [ ] Function `split_at`
- [ ] Function `split_first_mut`
- [ ] Function `split_first`
- [ ] Function `split_last_mut`
- [ ] Function `split_last`
- [ ] Function `values_mut`
- [ ] Struct `Drain`
- [ ] Struct `EntityIndexMap`
- [ ] Struct `IntoIter`
- [ ] Struct `IntoKeys`
- [ ] Struct `IterMut`
- [ ] Struct `Iter`
- [ ] Struct `Keys`
- [ ] Struct `Slice`

### `entity/index_set.rs`

- [ ] Function `as_boxed_inner`
- [ ] Function `as_inner`
- [ ] Function `as_slice`
- [ ] Function `as_slice`
- [ ] Function `as_slice`
- [ ] Function `as_slice`
- [ ] Function `from_boxed_slice_unchecked`
- [ ] Function `get_range`
- [ ] Function `get_range`
- [ ] Function `into_boxed_inner`
- [ ] Function `into_boxed_slice`
- [ ] Function `split_at`
- [ ] Function `split_first`
- [ ] Function `split_last`
- [ ] Struct `Drain`
- [ ] Struct `EntityIndexSet`
- [ ] Struct `IntoIter`
- [ ] Struct `Iter`
- [ ] Struct `Slice`

### `entity/map_entities.rs`

- [ ] Function `get_map_mut`
- [ ] Function `get_map`
- [ ] Function `world_scope`
- [ ] Struct `SceneEntityMapper`

### `entity/mod.rs`

- [ ] Enum `EntityNotSpawnedError`
- [ ] Enum `SpawnError`
- [ ] Function `after_versions_and_could_alias`
- [ ] Function `after_versions`
- [ ] Function `alloc_many`
- [ ] Function `any_spawned`
- [ ] Function `check_can_spawn_at`
- [ ] Function `cmp_approx`
- [ ] Function `contains_spawned`
- [ ] Function `count_spawned`
- [ ] Function `entity_get_spawn_or_despawn_tick`
- [ ] Function `entity_get_spawned_or_despawned_by`
- [ ] Function `from_index_and_generation`
- [ ] Function `from_index`
- [ ] Function `from_raw_u32`
- [ ] Function `from_raw_u32`
- [ ] Function `get_spawned`
- [ ] Function `index_u32`
- [ ] Function `is_index_spawned`
- [ ] Function `resolve_from_index`
- [ ] Function `try_from_bits`
- [ ] Struct `AllocEntitiesIterator`
- [ ] Struct `Entities`
- [ ] Struct `EntityGeneration`
- [ ] Struct `EntityIndex`
- [ ] Struct `EntityValidButNotSpawnedError`
- [ ] Struct `InvalidEntityError`

### `entity/unique_array.rs`

- [ ] Function `as_inner`
- [ ] Function `as_mut_slice`
- [ ] Function `as_mut_slice`
- [ ] Function `as_slice`
- [ ] Function `as_slice`
- [ ] Function `each_ref`
- [ ] Function `from_arc_array_unchecked`
- [ ] Function `from_boxed_array_unchecked`
- [ ] Function `from_rc_array_unchecked`
- [ ] Function `into_arc_inner`
- [ ] Function `into_boxed_inner`
- [ ] Function `into_rc_inner`
- [ ] Struct `UniqueEntityEquivalentArray`
- [ ] Type `IntoIter`
- [ ] Type `UniqueEntityArray`

### `entity/unique_slice.rs`

- [ ] Function `as_inner`
- [ ] Function `as_inner`
- [ ] Function `as_inner`
- [ ] Function `as_mut_inner`
- [ ] Function `as_mut_inner`
- [ ] Function `as_mut_ptr_range`
- [ ] Function `as_mut_ptr`
- [ ] Function `as_slice`
- [ ] Function `as_slice`
- [ ] Function `cast_slice_of_mut_unique_entity_slice_mut`
- [ ] Function `cast_slice_of_unique_entity_slice_mut`
- [ ] Function `cast_slice_of_unique_entity_slice`
- [ ] Function `chunk_by_mut`
- [ ] Function `chunk_by`
- [ ] Function `chunks_exact_mut`
- [ ] Function `chunks_exact`
- [ ] Function `chunks_mut`
- [ ] Function `chunks`
- [ ] Function `first_chunk`
- [ ] Function `from_arc_slice_unchecked`
- [ ] Function `from_boxed_slice_unchecked`
- [ ] Function `from_mut_slice_iterator_unchecked`
- [ ] Function `from_mut`
- [ ] Function `from_rc_slice_unchecked`
- [ ] Function `from_ref`
- [ ] Function `from_slice_iterator_unchecked`
- [ ] Function `get_unchecked_mut`
- [ ] Function `get_unchecked`
- [ ] Function `into_arc_inner`
- [ ] Function `into_boxed_inner`
- [ ] Function `into_rc_inner`
- [ ] Function `into_remainder`
- [ ] Function `into_remainder`
- [ ] Function `into_slice`
- [ ] Function `into_vec`
- [ ] Function `last_chunk`
- [ ] Function `rchunks_exact_mut`
- [ ] Function `rchunks_exact`
- [ ] Function `rchunks_mut`
- [ ] Function `rchunks`
- [ ] Function `remainder`
- [ ] Function `remainder`
- [ ] Function `reverse`
- [ ] Function `rotate_left`
- [ ] Function `rotate_right`
- [ ] Function `rsplit_mut`
- [ ] Function `rsplit`
- [ ] Function `rsplitn_mut`
- [ ] Function `rsplitn`
- [ ] Function `sort_by_cached_key`
- [ ] Function `sort_by_key`
- [ ] Function `sort_by`
- [ ] Function `sort_unstable_by_key`
- [ ] Function `sort_unstable_by`
- [ ] Function `sort_unstable`
- [ ] Function `sort`
- [ ] Function `split_at_checked`
- [ ] Function `split_at_mut_checked`
- [ ] Function `split_at_mut`
- [ ] Function `split_at`
- [ ] Function `split_first_chunk`
- [ ] Function `split_first`
- [ ] Function `split_inclusive_mut`
- [ ] Function `split_inclusive`
- [ ] Function `split_last_chunk`
- [ ] Function `split_last`
- [ ] Function `split_mut`
- [ ] Function `split`
- [ ] Function `splitn_mut`
- [ ] Function `splitn`
- [ ] Function `swap`
- [ ] Function `to_vec`
- [ ] Function `windows`
- [ ] Struct `UniqueEntityEquivalentSliceIterMut`
- [ ] Struct `UniqueEntityEquivalentSliceIter`
- [ ] Struct `UniqueEntityEquivalentSlice`
- [ ] Type `ChunkByMut`
- [ ] Type `ChunkBy`
- [ ] Type `ChunksExactMut`
- [ ] Type `ChunksExact`
- [ ] Type `ChunksMut`
- [ ] Type `Chunks`
- [ ] Type `IterMut`
- [ ] Type `Iter`
- [ ] Type `RChunksExactMut`
- [ ] Type `RChunksExact`
- [ ] Type `RChunksMut`
- [ ] Type `RChunks`
- [ ] Type `RSplitMut`
- [ ] Type `RSplitNMut`
- [ ] Type `RSplitN`
- [ ] Type `RSplit`
- [ ] Type `SplitInclusiveMut`
- [ ] Type `SplitInclusive`
- [ ] Type `SplitMut`
- [ ] Type `SplitNMut`
- [ ] Type `SplitN`
- [ ] Type `Split`
- [ ] Type `UniqueEntitySlice`
- [ ] Type `Windows`

### `entity/unique_vec.rs`

- [ ] Function `append`
- [ ] Function `as_mut_ptr`
- [ ] Function `as_mut_slice`
- [ ] Function `as_mut_slice`
- [ ] Function `as_mut_vec`
- [ ] Function `as_ptr`
- [ ] Function `as_slice`
- [ ] Function `as_slice`
- [ ] Function `as_slice`
- [ ] Function `as_vec`
- [ ] Function `dedup_by_key`
- [ ] Function `dedup_by`
- [ ] Function `from_raw_parts`
- [ ] Function `from_vec_unchecked`
- [ ] Function `into_boxed_slice`
- [ ] Function `leak`
- [ ] Function `pop`
- [ ] Function `reserve_exact`
- [ ] Function `resize_with`
- [ ] Function `retain_mut`
- [ ] Function `retain`
- [ ] Function `set_len`
- [ ] Function `shrink_to_fit`
- [ ] Function `shrink_to`
- [ ] Function `spare_capacity_mut`
- [ ] Function `splice`
- [ ] Function `split_off`
- [ ] Function `truncate`
- [ ] Function `try_reserve_exact`
- [ ] Function `try_reserve`
- [ ] Struct `UniqueEntityEquivalentVec`
- [ ] Type `Drain`
- [ ] Type `IntoIter`
- [ ] Type `Splice`
- [ ] Type `UniqueEntityVec`

### `entity_disabling.rs`

- [ ] Function `disabling_ids`
- [ ] Function `empty`
- [ ] Function `register_disabling_component`
- [ ] Struct `Disabled`

### `error/bevy_error.rs`

- [ ] Function `bevy_error_panic_hook`
- [ ] Function `downcast_ref`

### `error/command_handling.rs`

- [ ] Trait `CommandWithEntity`
- [ ] Trait `HandleError`

### `error/handler.rs`

- [ ] Enum `ErrorContext`
- [ ] Function `debug`
- [ ] Function `error`
- [ ] Function `ignore`
- [ ] Function `info`
- [ ] Function `kind`
- [ ] Function `panic`
- [ ] Function `trace`
- [ ] Function `warn`
- [ ] Type `ErrorHandler`

### `error/mod.rs`

- [ ] Type `Result`

### `event/mod.rs`

- [ ] Function `event_key`
- [ ] Function `register_event_key`
- [ ] Struct `EventKey`
- [ ] Trait `EntityEvent`

### `event/trigger.rs`

- [ ] Function `trigger_entity_internal`
- [ ] Struct `EntityComponentsTrigger`
- [ ] Struct `EntityTrigger`
- [ ] Struct `GlobalTrigger`
- [ ] Struct `PropagateEntityTrigger`

### `hierarchy.rs`

- [ ] Function `add_child`
- [ ] Function `add_child`
- [ ] Function `add_children`
- [ ] Function `add_children`
- [ ] Function `clear_children`
- [ ] Function `clear_children`
- [ ] Function `detach_all_children`
- [ ] Function `detach_all_children`
- [ ] Function `detach_child`
- [ ] Function `detach_child`
- [ ] Function `detach_children`
- [ ] Function `detach_children`
- [ ] Function `insert_child`
- [ ] Function `insert_child`
- [ ] Function `insert_children`
- [ ] Function `insert_children`
- [ ] Function `remove_child`
- [ ] Function `remove_child`
- [ ] Function `remove_children`
- [ ] Function `remove_children`
- [ ] Function `replace_children_with_difference`
- [ ] Function `replace_children_with_difference`
- [ ] Function `replace_children`
- [ ] Function `replace_children`
- [ ] Function `sort_by_cached_key`
- [ ] Function `sort_by_key`
- [ ] Function `sort_by`
- [ ] Function `sort_unstable_by_key`
- [ ] Function `sort_unstable_by`
- [ ] Function `swap`
- [ ] Function `validate_parent_has_component`
- [ ] Function `with_child`
- [ ] Function `with_child`
- [ ] Function `with_children`
- [ ] Function `with_children`
- [ ] Type `ChildSpawnerCommands`
- [ ] Type `ChildSpawner`

### `intern.rs`

- [ ] Enum `A`
- [ ] Function `intern`
- [ ] Struct `A`
- [ ] Struct `Interned`
- [ ] Struct `Interner`

### `lib.rs`

- [ ] Struct `HotPatchChanges`
- [ ] Struct `HotPatched`

### `lifecycle.rs`

- [ ] Function `messages`
- [ ] Function `on_despawn`
- [ ] Function `on_replace`
- [ ] Function `read_with_id`
- [ ] Function `reader_mut_with_messages`
- [ ] Function `reader_mut`
- [ ] Function `reader`
- [ ] Function `try_on_add`
- [ ] Function `try_on_despawn`
- [ ] Function `try_on_insert`
- [ ] Function `try_on_remove`
- [ ] Function `try_on_replace`
- [ ] Function `write`
- [ ] Struct `Add`
- [ ] Struct `Despawn`
- [ ] Struct `HookContext`
- [ ] Struct `Insert`
- [ ] Struct `Remove`
- [ ] Struct `RemovedComponentMessages`
- [ ] Struct `Replace`
- [ ] Type `ComponentHook`
- [ ] Type `RemovedIterWithId`
- [ ] Type `RemovedIter`

### `message/iterators.rs`

- [ ] Function `batching_strategy`
- [ ] Function `for_each_with_id`
- [ ] Function `without_id`
- [ ] Struct `MessageIteratorWithId`
- [ ] Struct `MessageIterator`
- [ ] Struct `MessageParIter`

### `message/message_cursor.rs`

- [ ] Function `missed_messages`
- [ ] Function `par_read_mut`
- [ ] Function `par_read`
- [ ] Function `read_mut_with_id`
- [ ] Function `read_mut`
- [ ] Function `read_with_id`
- [ ] Struct `MessageCursor`

### `message/message_mutator.rs`

- [ ] Function `par_read`
- [ ] Function `read_with_id`
- [ ] Struct `MessageMutator`

### `message/message_reader.rs`

- [ ] Function `par_read`
- [ ] Function `read_with_id`
- [ ] Struct `MessageReader`

### `message/message_registry.rs`

- [ ] Function `deregister_messages`
- [ ] Function `register_message`
- [ ] Function `run_updates`
- [ ] Struct `MessageRegistry`

### `message/message_writer.rs`

- [ ] Function `write_batch`
- [ ] Function `write_default`
- [ ] Function `write`
- [ ] Struct `MessageWriter`

### `message/messages.rs`

- [ ] Function `get_cursor_current`
- [ ] Function `get_cursor`
- [ ] Function `get_message`
- [ ] Function `iter_current_update_messages`
- [ ] Function `oldest_message_count`
- [ ] Function `update_drain`
- [ ] Function `write_batch`
- [ ] Function `write_default`
- [ ] Function `write`
- [ ] Struct `Messages`
- [ ] Struct `WriteBatchIds`

### `message/mod.rs`

- [ ] Struct `MessageId`

### `message/mut_iterators.rs`

- [ ] Function `batching_strategy`
- [ ] Function `for_each_with_id`
- [ ] Function `without_id`
- [ ] Struct `MessageMutIteratorWithId`
- [ ] Struct `MessageMutIterator`
- [ ] Struct `MessageMutParIter`

### `message/update.rs`

- [ ] Function `message_update_condition`
- [ ] Function `message_update_system`
- [ ] Function `signal_message_update_system`
- [ ] Struct `MessageUpdateSystems`

### `name.rs`

- [ ] Function `mutate`
- [ ] Function `set`
- [ ] Struct `NameOrEntity`
- [ ] Struct `Name`

### `never.rs`

- [ ] Type `Never`

### `observer/centralized_storage.rs`

- [ ] Function `component_observers`
- [ ] Function `entity_component_observers`
- [ ] Function `entity_observers`
- [ ] Function `global_observers`
- [ ] Function `global_observers`
- [ ] Function `try_get_observers`
- [ ] Struct `Observers`
- [ ] Type `ObserverMap`

### `observer/distributed_storage.rs`

- [ ] Function `event_keys`
- [ ] Function `system_name`
- [ ] Function `watch_entities`
- [ ] Function `watch_entity`
- [ ] Function `with_dynamic_runner`
- [ ] Function `with_entities`
- [ ] Function `with_entities`
- [ ] Function `with_entity`
- [ ] Function `with_error_handler`
- [ ] Function `with_event_key`
- [ ] Function `with_event_keys`
- [ ] Struct `ObservedBy`

### `observer/entity_cloning.rs`

- [ ] Function `add_observers`

### `observer/mod.rs`

- [ ] Function `trigger_ref_with`
- [ ] Function `trigger_ref`
- [ ] Function `trigger_with`

### `observer/runner.rs`

- [ ] Type `ObserverRunner`

### `observer/system_param.rs`

- [ ] Function `caller`
- [ ] Function `event_key`
- [ ] Function `event_mut`
- [ ] Function `event_ptr`
- [ ] Function `get_propagate`
- [ ] Function `observer`
- [ ] Function `original_event_target`
- [ ] Function `propagate`
- [ ] Function `trigger_mut`
- [ ] Struct `On`
- [ ] Struct `TriggerContext`

### `query/access.rs`

- [ ] Enum `AccessConflicts`
- [ ] Function `add_archetypal`
- [ ] Function `add_component_read`
- [ ] Function `add_component_read`
- [ ] Function `add_component_write`
- [ ] Function `add_component_write`
- [ ] Function `add_resource_read`
- [ ] Function `add_resource_read`
- [ ] Function `add_resource_write`
- [ ] Function `add_resource_write`
- [ ] Function `add_unfiltered_read_all_resources`
- [ ] Function `add_unfiltered_resource_read`
- [ ] Function `add_unfiltered_resource_write`
- [ ] Function `add_unfiltered_write_all_resources`
- [ ] Function `and_with`
- [ ] Function `and_without`
- [ ] Function `append_or`
- [ ] Function `archetypal`
- [ ] Function `clear_writes`
- [ ] Function `combined_access`
- [ ] Function `extend_access`
- [ ] Function `extend`
- [ ] Function `extend`
- [ ] Function `extend`
- [ ] Function `get_conflicts_single`
- [ ] Function `get_conflicts`
- [ ] Function `get_conflicts`
- [ ] Function `get_conflicts`
- [ ] Function `has_any_component_read`
- [ ] Function `has_any_component_write`
- [ ] Function `has_any_read`
- [ ] Function `has_any_resource_read`
- [ ] Function `has_any_resource_write`
- [ ] Function `has_any_write`
- [ ] Function `has_archetypal`
- [ ] Function `has_component_read`
- [ ] Function `has_component_write`
- [ ] Function `has_read_all_components`
- [ ] Function `has_read_all_resources`
- [ ] Function `has_read_all`
- [ ] Function `has_resource_read`
- [ ] Function `has_resource_write`
- [ ] Function `has_write_all_components`
- [ ] Function `has_write_all_resources`
- [ ] Function `has_write_all`
- [ ] Function `is_components_compatible`
- [ ] Function `is_resources_compatible`
- [ ] Function `is_subset_components`
- [ ] Function `is_subset_resources`
- [ ] Function `is_subset`
- [ ] Function `is_subset`
- [ ] Function `matches_everything`
- [ ] Function `matches_nothing`
- [ ] Function `read_all_components`
- [ ] Function `read_all_components`
- [ ] Function `read_all_resources`
- [ ] Function `read_all`
- [ ] Function `read_all`
- [ ] Function `read_all`
- [ ] Function `remove_component_read`
- [ ] Function `remove_component_write`
- [ ] Function `remove_conflicting_access`
- [ ] Function `resource_reads_and_writes`
- [ ] Function `resource_reads`
- [ ] Function `resource_writes`
- [ ] Function `try_iter_component_access`
- [ ] Function `with_filters`
- [ ] Function `without_filters`
- [ ] Function `write_all_components`
- [ ] Function `write_all_components`
- [ ] Function `write_all_resources`
- [ ] Function `write_all`
- [ ] Function `write_all`
- [ ] Function `write_all`
- [ ] Struct `FilteredAccessSet`
- [ ] Struct `UnboundedAccessError`

### `query/access_iter.rs`

- [ ] Enum `ResourceAccessLevel`
- [ ] Function `has_conflicts`

### `query/builder.rs`

- [ ] Function `and`
- [ ] Function `data`
- [ ] Function `extend_access`
- [ ] Function `filter`
- [ ] Function `mut_id`
- [ ] Function `optional`
- [ ] Function `or`
- [ ] Function `ref_id`
- [ ] Function `transmute_filtered`
- [ ] Function `transmute`
- [ ] Function `with_id`
- [ ] Function `without_id`
- [ ] Function `without`

### `query/fetch.rs`

- [ ] Function `extract`
- [ ] Function `is_spawned`
- [ ] Function `set_table`
- [ ] Function `spawn_tick`
- [ ] Function `spawned_by`
- [ ] Struct `A`
- [ ] Struct `B`
- [ ] Struct `C`
- [ ] Struct `C`
- [ ] Struct `Client`
- [ ] Struct `D`
- [ ] Struct `DerivedNonReleaseMutable`
- [ ] Struct `DerivedNonReleaseRead`
- [ ] Struct `DerivedReleaseMutable`
- [ ] Struct `DerivedReleaseRead`
- [ ] Struct `EntityFetch`
- [ ] Struct `IgnoredQuery`
- [ ] Struct `NamedQuery`
- [ ] Struct `ReadFetch`
- [ ] Struct `RefFetch`
- [ ] Struct `SpawnDetailsFetch`
- [ ] Struct `SpawnDetails`
- [ ] Struct `TupleQuery`
- [ ] Struct `UnitQuery`
- [ ] Struct `WriteFetch`
- [ ] Trait `ClientState`
- [ ] Trait `ReleaseStateQueryData`
- [ ] Type `QueryItem`
- [ ] Type `ROQueryItem`

### `query/filter.rs`

- [ ] Struct `Allow`
- [ ] Struct `OrFetch`
- [ ] Struct `SpawnedFetch`
- [ ] Struct `Spawned`

### `query/iter.rs`

- [ ] Function `fetch_next_back`
- [ ] Function `fetch_next_back`
- [ ] Function `fetch_next`
- [ ] Function `fetch_next`
- [ ] Function `fetch_next`
- [ ] Function `remaining_mut`
- [ ] Function `remaining`
- [ ] Function `sort_by_cached_key`
- [ ] Function `sort_by_cached_key`
- [ ] Function `sort_by_key`
- [ ] Function `sort_by_key`
- [ ] Function `sort_by`
- [ ] Function `sort_by`
- [ ] Function `sort_unstable_by_key`
- [ ] Function `sort_unstable_by_key`
- [ ] Function `sort_unstable_by`
- [ ] Function `sort_unstable_by`
- [ ] Function `sort_unstable`
- [ ] Function `sort_unstable`
- [ ] Function `sort`
- [ ] Function `sort`
- [ ] Struct `QueryManyUniqueIter`
- [ ] Struct `QuerySortedIter`
- [ ] Struct `QuerySortedManyIter`

### `query/par_iter.rs`

- [ ] Function `batching_strategy`
- [ ] Function `batching_strategy`
- [ ] Function `batching_strategy`
- [ ] Function `for_each_init`
- [ ] Function `for_each_init`
- [ ] Function `for_each_init`
- [ ] Struct `QueryParManyIter`
- [ ] Struct `QueryParManyUniqueIter`

### `query/state.rs`

- [ ] Function `as_readonly`
- [ ] Function `component_access`
- [ ] Function `from_builder`
- [ ] Function `get_manual`
- [ ] Function `get_many_mut`
- [ ] Function `get_many_unique_mut`
- [ ] Function `get_many_unique`
- [ ] Function `get_many`
- [ ] Function `get_unchecked`
- [ ] Function `iter_combinations_mut`
- [ ] Function `iter_combinations_unchecked`
- [ ] Function `iter_combinations`
- [ ] Function `iter_manual`
- [ ] Function `iter_many_manual`
- [ ] Function `iter_many_mut`
- [ ] Function `iter_many_unique_manual`
- [ ] Function `iter_many_unique_mut`
- [ ] Function `iter_many_unique`
- [ ] Function `iter_many`
- [ ] Function `iter_mut`
- [ ] Function `iter_unchecked`
- [ ] Function `join_filtered`
- [ ] Function `join`
- [ ] Function `matched_archetypes`
- [ ] Function `matched_tables`
- [ ] Function `matches_component_set`
- [ ] Function `new_archetype`
- [ ] Function `par_iter_mut`
- [ ] Function `par_iter`
- [ ] Function `query_manual`
- [ ] Function `query_mut`
- [ ] Function `query_unchecked_manual_with_ticks`
- [ ] Function `query_unchecked_manual`
- [ ] Function `query_unchecked_with_ticks`
- [ ] Function `query_unchecked`
- [ ] Function `query`
- [ ] Function `single_mut`
- [ ] Function `single_unchecked_manual`
- [ ] Function `single_unchecked`
- [ ] Function `single`
- [ ] Function `transmute_filtered`
- [ ] Function `transmute`
- [ ] Function `try_new`
- [ ] Function `update_archetypes_unsafe_world_cell`
- [ ] Function `update_archetypes`
- [ ] Function `validate_world`

### `reflect/bundle.rs`

- [ ] Function `apply_or_insert_mapped`
- [ ] Function `fn_pointers`
- [ ] Function `take`
- [ ] Struct `ReflectBundleFns`
- [ ] Struct `ReflectBundle`

### `reflect/component.rs`

- [ ] Function `apply_or_insert_mapped`
- [ ] Function `copy`
- [ ] Function `fn_pointers`
- [ ] Function `map_entities`
- [ ] Function `reflect_mut`
- [ ] Function `reflect_unchecked_mut`
- [ ] Function `reflect`
- [ ] Function `register_component`
- [ ] Struct `ReflectComponentFns`
- [ ] Struct `ReflectComponent`

### `reflect/entity_commands.rs`

- [ ] Function `insert_reflect_with_registry`
- [ ] Function `insert_reflect`
- [ ] Function `remove_reflect_with_registry`
- [ ] Function `remove_reflect`
- [ ] Trait `ReflectCommandExt`

### `reflect/event.rs`

- [ ] Struct `ReflectEventFns`
- [ ] Struct `ReflectEvent`

### `reflect/from_world.rs`

- [ ] Function `fn_pointers`
- [ ] Function `from_world`
- [ ] Struct `ReflectFromWorldFns`
- [ ] Struct `ReflectFromWorld`

### `reflect/map_entities.rs`

- [ ] Function `map_entities`
- [ ] Struct `ReflectMapEntities`

### `reflect/mod.rs`

- [ ] Function `from_reflect_with_fallback`
- [ ] Function `new_with_derived_types`
- [ ] Struct `AppFunctionRegistry`
- [ ] Struct `AppTypeRegistry`

### `reflect/resource.rs`

- [ ] Function `apply_or_insert`
- [ ] Function `copy`
- [ ] Function `fn_pointers`
- [ ] Function `reflect_mut`
- [ ] Function `reflect_unchecked_mut`
- [ ] Function `reflect`
- [ ] Function `register_resource`
- [ ] Struct `ReflectResourceFns`
- [ ] Struct `ReflectResource`

### `relationship/mod.rs`

- [ ] Function `clone_relationship_target`
- [ ] Function `relationship_target`
- [ ] Function `relationship`
- [ ] Struct `RelationshipCloneBehaviorSpecialization`
- [ ] Type `SourceIter`

### `relationship/related_methods.rs`

- [ ] Function `add_one_related`
- [ ] Function `add_one_related`
- [ ] Function `add_related`
- [ ] Function `add_related`
- [ ] Function `commands_mut`
- [ ] Function `despawn_children`
- [ ] Function `despawn_children`
- [ ] Function `despawn_related`
- [ ] Function `despawn_related`
- [ ] Function `detach_all_related`
- [ ] Function `detach_all_related`
- [ ] Function `insert_recursive`
- [ ] Function `insert_recursive`
- [ ] Function `insert_related`
- [ ] Function `insert_related`
- [ ] Function `remove_recursive`
- [ ] Function `remove_recursive`
- [ ] Function `remove_related`
- [ ] Function `remove_related`
- [ ] Function `replace_related_with_difference`
- [ ] Function `replace_related_with_difference`
- [ ] Function `replace_related`
- [ ] Function `replace_related`
- [ ] Function `target_entity`
- [ ] Function `target_entity`
- [ ] Function `with_related_entities`
- [ ] Function `with_related_entities`
- [ ] Function `with_related`
- [ ] Function `with_related`
- [ ] Struct `RelatedSpawnerCommands`
- [ ] Struct `RelatedSpawner`

### `relationship/relationship_query.rs`

- [ ] Function `iter_ancestors`
- [ ] Function `iter_descendants_depth_first`
- [ ] Function `iter_descendants`
- [ ] Function `iter_leaves`
- [ ] Function `iter_siblings`
- [ ] Function `related`
- [ ] Function `relationship_sources`
- [ ] Function `root_ancestor`

### `schedule/auto_insert_apply_deferred.rs`

- [ ] Struct `IgnoreDeferred`

### `schedule/condition.rs`

- [ ] Function `any_component_removed`
- [ ] Function `any_match_filter`
- [ ] Function `any_with_component`
- [ ] Function `condition_changed_to`
- [ ] Function `condition_changed`
- [ ] Function `not`
- [ ] Function `on_message`
- [ ] Function `resource_added`
- [ ] Function `resource_changed_or_removed`
- [ ] Function `resource_changed`
- [ ] Function `resource_equals`
- [ ] Function `resource_exists_and_changed`
- [ ] Function `resource_exists_and_equals`
- [ ] Function `resource_exists`
- [ ] Function `resource_removed`
- [ ] Struct `AndMarker`
- [ ] Struct `NandMarker`
- [ ] Struct `NorMarker`
- [ ] Struct `NotMarker`
- [ ] Struct `OrMarker`
- [ ] Struct `XnorMarker`
- [ ] Struct `XorMarker`
- [ ] Type `And`
- [ ] Type `BoxedCondition`
- [ ] Type `Nand`
- [ ] Type `Nor`
- [ ] Type `NotSystem`
- [ ] Type `Or`
- [ ] Type `Xnor`
- [ ] Type `Xor`

### `schedule/config.rs`

- [ ] Function `in_set_inner`
- [ ] Function `run_if_dyn`
- [ ] Struct `ScheduleConfigTupleMarker`
- [ ] Struct `ScheduleConfig`

### `schedule/error.rs`

- [ ] Function `to_string`
- [ ] Function `to_string`

### `schedule/executor/mod.rs`

- [ ] Struct `ApplyDeferred`
- [ ] Struct `SystemSchedule`

### `schedule/executor/multi_threaded.rs`

- [ ] Struct `ExecutorState`
- [ ] Struct `MainThreadExecutor`
- [ ] Struct `MultiThreadedExecutor`

### `schedule/executor/single_threaded.rs`

- [ ] Struct `SingleThreadedExecutor`

### `schedule/graph/dag.rs`

- [ ] Function `check_for_cross_dependencies`
- [ ] Function `check_for_overlapping_groups`
- [ ] Function `check_for_redundant_edges`
- [ ] Function `connected`
- [ ] Function `disconnected`
- [ ] Function `ensure_toposorted`
- [ ] Function `flatten_undirected`
- [ ] Function `flatten`
- [ ] Function `get_toposort`
- [ ] Function `graph_mut`
- [ ] Function `graph`
- [ ] Function `group_by_key`
- [ ] Function `is_dirty`
- [ ] Function `is_toposorted`
- [ ] Function `reachable`
- [ ] Function `remove_redundant_edges`
- [ ] Function `toposort_and_graph`
- [ ] Function `toposort`
- [ ] Function `transitive_closure`
- [ ] Function `transitive_edges`
- [ ] Function `transitive_reduction`
- [ ] Function `try_convert`

### `schedule/graph/graph_map.rs`

- [ ] Function `all_edges`
- [ ] Function `contains_edge`
- [ ] Function `contains_node`
- [ ] Function `edge_count`
- [ ] Function `edges_directed`
- [ ] Function `edges`
- [ ] Function `neighbors_directed`
- [ ] Function `node_count`
- [ ] Function `nodes`
- [ ] Function `opposite`
- [ ] Function `remove_edge`
- [ ] Function `remove_node`
- [ ] Function `reserve_edges`
- [ ] Function `reserve_nodes`
- [ ] Function `simple_cycles_in_component`
- [ ] Function `toposort`
- [ ] Function `try_convert`
- [ ] Struct `Graph`
- [ ] Type `DiGraph`
- [ ] Type `UnGraph`

### `schedule/graph/mod.rs`

- [ ] Function `add_config`
- [ ] Struct `GraphInfo`

### `schedule/mod.rs`

- [ ] Struct `TestSchedule`

### `schedule/node.rs`

- [ ] Function `check_if_not_empty`
- [ ] Function `check_type_set_ambiguity`
- [ ] Function `get_conditions_mut`
- [ ] Function `get_conditions_mut`
- [ ] Function `get_conditions`
- [ ] Function `get_conditions`
- [ ] Function `get_conflicting_systems`
- [ ] Function `get_key_or_insert`
- [ ] Function `get_key`
- [ ] Function `has_conditions`
- [ ] Function `has_conditions`
- [ ] Function `initialize`
- [ ] Function `initialize`
- [ ] Function `is_set`
- [ ] Function `is_system`
- [ ] Function `to_string`
- [ ] Struct `AmbiguousSystemConflictsWarning`
- [ ] Struct `SystemKey`
- [ ] Struct `SystemSetKey`
- [ ] Struct `SystemSets`
- [ ] Struct `SystemTypeSetAmbiguityError`
- [ ] Struct `SystemWithAccess`
- [ ] Struct `Systems`
- [ ] Struct `TestSet`

### `schedule/schedule.rs`

- [ ] Enum `Chain`
- [ ] Function `add_build_pass`
- [ ] Function `allow_ambiguous_component`
- [ ] Function `allow_ambiguous_resource`
- [ ] Function `apply_deferred`
- [ ] Function `build_schedule`
- [ ] Function `check_change_ticks`
- [ ] Function `configure_schedules`
- [ ] Function `conflicting_systems`
- [ ] Function `dependency`
- [ ] Function `entry`
- [ ] Function `get_build_settings`
- [ ] Function `get_executor_kind`
- [ ] Function `get_node_name`
- [ ] Function `graph_mut`
- [ ] Function `graph`
- [ ] Function `hierarchy`
- [ ] Function `ignore_ambiguity`
- [ ] Function `ignore_ambiguity`
- [ ] Function `initialize`
- [ ] Function `initialize`
- [ ] Function `iter_ignored_ambiguities`
- [ ] Function `iter_mut`
- [ ] Function `label`
- [ ] Function `print_ignored_ambiguities`
- [ ] Function `remove_build_pass`
- [ ] Function `remove_entry`
- [ ] Function `remove_systems_in_set`
- [ ] Function `remove_systems_in_set`
- [ ] Function `remove_systems_in_set`
- [ ] Function `set_apply_final_deferred`
- [ ] Function `set_chained_with_config`
- [ ] Function `set_chained`
- [ ] Function `set_executor_kind`
- [ ] Function `systems_in_set`
- [ ] Function `systems_len`
- [ ] Function `systems`
- [ ] Function `warnings`
- [ ] Struct `ScheduleNotInitialized`

### `schedule/set.rs`

- [ ] Type `InternedScheduleLabel`
- [ ] Type `InternedSystemSet`

### `schedule/stepping.rs`

- [ ] Function `add_schedule`
- [ ] Function `always_run_node`
- [ ] Function `always_run`
- [ ] Function `begin_frame`
- [ ] Function `clear_breakpoint_node`
- [ ] Function `clear_breakpoint`
- [ ] Function `clear_node`
- [ ] Function `clear_schedule`
- [ ] Function `clear_system`
- [ ] Function `continue_frame`
- [ ] Function `cursor`
- [ ] Function `never_run_node`
- [ ] Function `never_run`
- [ ] Function `remove_schedule`
- [ ] Function `schedules`
- [ ] Function `set_breakpoint_node`
- [ ] Function `set_breakpoint`
- [ ] Function `skipped_systems`
- [ ] Function `step_frame`
- [ ] Struct `NotReady`

### `spawn.rs`

- [ ] Struct `SpawnIter`
- [ ] Struct `SpawnOneRelated`
- [ ] Struct `SpawnRelatedBundle`
- [ ] Struct `SpawnWith`
- [ ] Struct `Spawn`
- [ ] Struct `WithOneRelated`
- [ ] Struct `WithRelated`

### `storage/blob_array.rs`

- [ ] Function `drop_last_element`
- [ ] Function `get_drop`
- [ ] Function `get_ptr_mut`
- [ ] Function `get_ptr`
- [ ] Function `get_sub_slice`
- [ ] Function `get_unchecked_mut`
- [ ] Function `get_unchecked`
- [ ] Function `initialize_unchecked`
- [ ] Function `is_zst`
- [ ] Function `replace_unchecked`
- [ ] Function `swap_remove_and_drop_unchecked_nonoverlapping`
- [ ] Function `swap_remove_and_drop_unchecked`
- [ ] Function `swap_remove_unchecked_nonoverlapping`
- [ ] Function `swap_remove_unchecked`

### `storage/mod.rs`

- [ ] Function `prepare_component`

### `storage/resource.rs`

- [ ] Function `get_data`
- [ ] Function `get_ticks`
- [ ] Function `is_present`
- [ ] Struct `ResourceData`
- [ ] Struct `Resources`

### `storage/sparse_set.rs`

- [ ] Function `get_added_tick`
- [ ] Function `get_changed_by`
- [ ] Function `get_changed_tick`
- [ ] Function `get_drop`
- [ ] Function `get_or_insert_with`
- [ ] Function `get_ticks`
- [ ] Function `get_with_ticks`
- [ ] Function `indices`
- [ ] Function `iter_mut`
- [ ] Function `values_mut`
- [ ] Function `values`
- [ ] Struct `SparseSets`

### `storage/table/column.rs`

- [ ] Function `get_added_tick_unchecked`
- [ ] Function `get_added_ticks_slice`
- [ ] Function `get_changed_by_slice`
- [ ] Function `get_changed_by_unchecked`
- [ ] Function `get_changed_tick_unchecked`
- [ ] Function `get_changed_ticks_slice`
- [ ] Function `get_data_slice`
- [ ] Function `get_data_unchecked`
- [ ] Function `get_drop`
- [ ] Function `get_ticks_unchecked`

### `storage/table/mod.rs`

- [ ] Function `as_u32`
- [ ] Function `as_usize`
- [ ] Function `component_count`
- [ ] Function `empty`
- [ ] Function `entity_capacity`
- [ ] Function `from_u32`
- [ ] Function `from_usize`
- [ ] Function `get_added_tick`
- [ ] Function `get_added_ticks_slice_for`
- [ ] Function `get_changed_by_slice_for`
- [ ] Function `get_changed_by`
- [ ] Function `get_changed_tick`
- [ ] Function `get_changed_ticks_slice_for`
- [ ] Function `get_component`
- [ ] Function `get_data_slice_for`
- [ ] Function `get_drop_for`
- [ ] Function `get_ticks_unchecked`
- [ ] Function `has_column`
- [ ] Function `index_u32`
- [ ] Function `iter_columns`

### `storage/thin_array_ptr.rs`

- [ ] Function `as_slice`
- [ ] Function `clear_elements`
- [ ] Function `get_unchecked_mut`
- [ ] Function `get_unchecked`
- [ ] Function `initialize_unchecked`
- [ ] Function `realloc`
- [ ] Function `swap_remove_unchecked_nonoverlapping`
- [ ] Function `swap_remove_unchecked`
- [ ] Struct `ThinArrayPtr`

### `system/adapter_system.rs`

- [ ] Struct `IntoAdapterSystem`
- [ ] Struct `IsAdapterSystemMarker`

### `system/builder.rs`

- [ ] Function `local`
- [ ] Function `new_box`
- [ ] Function `new_box`
- [ ] Function `new_box`
- [ ] Function `query_filtered`
- [ ] Function `query`
- [ ] Function `resource_mut`
- [ ] Function `resource`
- [ ] Struct `FilteredResourcesMutParamBuilder`
- [ ] Struct `FilteredResourcesParamBuilder`
- [ ] Struct `IfBuilder`
- [ ] Struct `LocalBuilder`
- [ ] Struct `OptionBuilder`
- [ ] Struct `ParamBuilder`
- [ ] Struct `ParamSetBuilder`
- [ ] Struct `QueryParamBuilder`
- [ ] Struct `ResultBuilder`

### `system/combinator.rs`

- [ ] Struct `IntoPipeSystem`
- [ ] Struct `IsPipeSystemMarker`

### `system/commands/command.rs`

- [ ] Function `init_resource`
- [ ] Function `insert_batch`
- [ ] Function `insert_resource`
- [ ] Function `remove_resource`
- [ ] Function `run_schedule`
- [ ] Function `run_system_cached_with`
- [ ] Function `run_system_cached`
- [ ] Function `run_system_with`
- [ ] Function `run_system`
- [ ] Function `spawn_batch`
- [ ] Function `trigger_with`
- [ ] Function `unregister_system_cached`
- [ ] Function `unregister_system`
- [ ] Function `write_message`
- [ ] Trait `Command`

### `system/commands/entity_command.rs`

- [ ] Enum `EntityCommandError`
- [ ] Function `clone_components`
- [ ] Function `clone_with_opt_in`
- [ ] Function `clone_with_opt_out`
- [ ] Function `insert_by_id`
- [ ] Function `insert_from_world`
- [ ] Function `insert_with`
- [ ] Function `log_components`
- [ ] Function `move_components`
- [ ] Function `observe`
- [ ] Function `remove_by_id`
- [ ] Function `remove_with_requires`
- [ ] Function `retain`
- [ ] Trait `EntityCommand`

### `system/commands/mod.rs`

- [ ] Function `and_modify`
- [ ] Function `append`
- [ ] Function `clone_and_spawn_with_opt_in`
- [ ] Function `clone_and_spawn_with_opt_out`
- [ ] Function `clone_and_spawn`
- [ ] Function `clone_components`
- [ ] Function `clone_with_opt_in`
- [ ] Function `clone_with_opt_out`
- [ ] Function `commands_mut`
- [ ] Function `entry`
- [ ] Function `get_spawned_entity`
- [ ] Function `init_resource`
- [ ] Function `insert_batch_if_new`
- [ ] Function `insert_batch`
- [ ] Function `insert_by_id`
- [ ] Function `insert_if_new_and`
- [ ] Function `insert_if_new`
- [ ] Function `insert_if`
- [ ] Function `insert_resource`
- [ ] Function `log_components`
- [ ] Function `move_components`
- [ ] Function `new_from_entities`
- [ ] Function `observe`
- [ ] Function `or_default`
- [ ] Function `or_from_world`
- [ ] Function `or_try_insert_with`
- [ ] Function `or_try_insert`
- [ ] Function `queue_handled`
- [ ] Function `queue_handled`
- [ ] Function `queue_silenced`
- [ ] Function `queue_silenced`
- [ ] Function `queue`
- [ ] Function `queue`
- [ ] Function `register_system`
- [ ] Function `remove_by_id`
- [ ] Function `remove_if`
- [ ] Function `remove_resource`
- [ ] Function `remove_with_requires`
- [ ] Function `retain`
- [ ] Function `run_schedule`
- [ ] Function `run_system_cached_with`
- [ ] Function `run_system_cached`
- [ ] Function `run_system_with`
- [ ] Function `run_system`
- [ ] Function `spawn_batch`
- [ ] Function `trigger_with`
- [ ] Function `try_despawn`
- [ ] Function `try_insert_batch_if_new`
- [ ] Function `try_insert_batch`
- [ ] Function `try_insert_by_id`
- [ ] Function `try_insert_if_new_and`
- [ ] Function `try_insert_if_new`
- [ ] Function `try_insert_if`
- [ ] Function `try_insert`
- [ ] Function `try_remove_if`
- [ ] Function `try_remove`
- [ ] Function `unregister_system_cached`
- [ ] Function `unregister_system`
- [ ] Function `write_message`
- [ ] Struct `EntityEntryCommands`
- [ ] Struct `FetchState`

### `system/commands/parallel_scope.rs`

- [ ] Function `command_scope`
- [ ] Struct `ParallelCommands`

### `system/exclusive_function_system.rs`

- [ ] Function `with_name`
- [ ] Struct `HasExclusiveSystemInput`
- [ ] Struct `IsExclusiveFunctionSystem`

### `system/exclusive_system_param.rs`

- [ ] Trait `ExclusiveSystemParam`
- [ ] Type `ExclusiveSystemParamItem`

### `system/function_system.rs`

- [ ] Function `build_any_system`
- [ ] Function `build_system_with_input`
- [ ] Function `build_system`
- [ ] Function `get_unchecked`
- [ ] Function `has_deferred`
- [ ] Function `is_send`
- [ ] Function `matches_world`
- [ ] Function `meta_mut`
- [ ] Function `param_state_mut`
- [ ] Function `param_state`
- [ ] Function `set_exclusive`
- [ ] Function `set_has_deferred`
- [ ] Function `set_name`
- [ ] Function `set_non_send`
- [ ] Function `validate_param`
- [ ] Function `with_name`
- [ ] Struct `HasSystemInput`
- [ ] Struct `IsFunctionSystem`

### `system/input.rs`

- [ ] Struct `InMut`
- [ ] Struct `InRef`
- [ ] Struct `StaticSystemInput`
- [ ] Type `SystemIn`

### `system/mod.rs`

- [ ] Function `assert_is_read_only_system`
- [ ] Function `assert_is_system`
- [ ] Function `assert_system_does_not_conflict`

### `system/query.rs`

- [ ] Function `as_query_lens`
- [ ] Function `as_readonly`
- [ ] Function `get_inner`
- [ ] Function `get_many_inner`
- [ ] Function `get_many_mut_inner`
- [ ] Function `get_many_mut`
- [ ] Function `get_many_unique_inner`
- [ ] Function `get_many_unique_mut`
- [ ] Function `get_many_unique`
- [ ] Function `get_many`
- [ ] Function `get_unchecked`
- [ ] Function `into_query_lens`
- [ ] Function `into_readonly`
- [ ] Function `iter_combinations_inner`
- [ ] Function `iter_combinations_mut`
- [ ] Function `iter_combinations_unsafe`
- [ ] Function `iter_combinations`
- [ ] Function `iter_inner`
- [ ] Function `iter_many_inner`
- [ ] Function `iter_many_mut`
- [ ] Function `iter_many_unique_inner`
- [ ] Function `iter_many_unique_mut`
- [ ] Function `iter_many_unique_unsafe`
- [ ] Function `iter_many_unique`
- [ ] Function `iter_many_unsafe`
- [ ] Function `iter_many`
- [ ] Function `iter_mut`
- [ ] Function `iter_unsafe`
- [ ] Function `join_filtered_inner`
- [ ] Function `join_filtered`
- [ ] Function `join_inner`
- [ ] Function `join`
- [ ] Function `par_iter_inner`
- [ ] Function `par_iter_many_unique_mut`
- [ ] Function `par_iter_many_unique`
- [ ] Function `par_iter_many`
- [ ] Function `par_iter_mut`
- [ ] Function `par_iter`
- [ ] Function `query_inner`
- [ ] Function `query`
- [ ] Function `reborrow_unsafe`
- [ ] Function `single_inner`
- [ ] Function `single_mut`
- [ ] Function `single`
- [ ] Function `transmute_lens_filtered_inner`
- [ ] Function `transmute_lens_filtered`
- [ ] Function `transmute_lens_inner`
- [ ] Function `transmute_lens`
- [ ] Struct `Populated`
- [ ] Struct `QueryLens`
- [ ] Struct `Single`

### `system/schedule_system.rs`

- [ ] Function `value_mut`
- [ ] Function `value_mut`
- [ ] Struct `WithInputFromWrapper`
- [ ] Struct `WithInputWrapper`
- [ ] Type `ScheduleSystem`

### `system/system.rs`

- [ ] Struct `SystemStateFlags`
- [ ] Type `BoxedReadOnlySystem`
- [ ] Type `BoxedSystem`

### `system/system_param.rs`

- [ ] Function `downcast_mut_inner`
- [ ] Function `downcast_mut`
- [ ] Function `downcast`
- [ ] Function `invalid`
- [ ] Function `is`
- [ ] Function `last_run`
- [ ] Function `skipped`
- [ ] Function `this_run`
- [ ] Struct `Collide`
- [ ] Struct `Deferred`
- [ ] Struct `ExclusiveMarker`
- [ ] Struct `FetchState`
- [ ] Struct `If`
- [ ] Struct `InvariantParam`
- [ ] Struct `LongParam`
- [ ] Struct `MissingEvent`
- [ ] Struct `MissingResource`
- [ ] Struct `NonSendMarker`
- [ ] Struct `R`
- [ ] Struct `SpecialLocal`
- [ ] Struct `SpecialQuery`
- [ ] Struct `SpecialRes`
- [ ] Struct `StaticSystemParam`
- [ ] Struct `SystemChangeTick`
- [ ] Struct `SystemParamValidationError`
- [ ] Struct `TupleParam`
- [ ] Struct `UnitParam`
- [ ] Struct `WhereParam`
- [ ] Type `Read`
- [ ] Type `SCommands`
- [ ] Type `SQuery`
- [ ] Type `SResMut`
- [ ] Type `SRes`
- [ ] Type `SystemParamItem`
- [ ] Type `Write`

### `system/system_registry.rs`

- [ ] Function `from_entity`
- [ ] Function `initialized`
- [ ] Function `register_boxed_system`
- [ ] Function `register_system_cached`
- [ ] Function `register_system`
- [ ] Function `run_system_cached_with`
- [ ] Function `run_system_cached`
- [ ] Function `run_system_with`
- [ ] Function `run_system`
- [ ] Function `system`
- [ ] Function `unregister_system_cached`
- [ ] Function `unregister_system`
- [ ] Struct `RemovedSystem`
- [ ] Struct `SystemIdMarker`

### `world/command_queue.rs`

- [ ] Function `append`

### `world/deferred_world.rs`

- [ ] Function `entities_and_commands`
- [ ] Function `entity_mut`
- [ ] Function `get_mut_by_id`
- [ ] Function `get_non_send_mut_by_id`
- [ ] Function `get_non_send_resource_mut`
- [ ] Function `get_resource_mut_by_id`
- [ ] Function `get_resource_mut`
- [ ] Function `into_deferred`
- [ ] Function `non_send_resource_mut`
- [ ] Function `query`
- [ ] Function `resource_mut`
- [ ] Function `trigger_raw`
- [ ] Function `write_message_batch`
- [ ] Function `write_message_default`
- [ ] Function `write_message`

### `world/entity_access/entity_mut.rs`

- [ ] Function `archetype`
- [ ] Function `as_readonly`
- [ ] Function `as_unsafe_entity_cell`
- [ ] Function `contains_id`
- [ ] Function `contains_type_id`
- [ ] Function `get_by_id`
- [ ] Function `get_change_ticks_by_id`
- [ ] Function `get_change_ticks`
- [ ] Function `get_components_mut_unchecked`
- [ ] Function `get_components_mut`
- [ ] Function `get_components`
- [ ] Function `get_mut_assume_mutable_by_id_unchecked`
- [ ] Function `get_mut_assume_mutable_by_id`
- [ ] Function `get_mut_assume_mutable`
- [ ] Function `get_mut_by_id_unchecked`
- [ ] Function `get_mut_by_id`
- [ ] Function `get_ref`
- [ ] Function `into_borrow_by_id`
- [ ] Function `into_borrow`
- [ ] Function `into_components_mut_unchecked`
- [ ] Function `into_components_mut`
- [ ] Function `into_mut_assume_mutable_by_id`
- [ ] Function `into_mut_assume_mutable`
- [ ] Function `into_mut_by_id`
- [ ] Function `into_mut`
- [ ] Function `into_readonly`
- [ ] Function `into_ref`
- [ ] Function `location`
- [ ] Function `spawn_tick`
- [ ] Function `spawned_by`

### `world/entity_access/entity_ref.rs`

- [ ] Function `archetype`
- [ ] Function `contains_id`
- [ ] Function `contains_type_id`
- [ ] Function `get_by_id`
- [ ] Function `get_change_ticks_by_id`
- [ ] Function `get_change_ticks`
- [ ] Function `get_components`
- [ ] Function `get_ref`
- [ ] Function `location`
- [ ] Function `spawn_tick`
- [ ] Function `spawned_by`

### `world/entity_access/entry.rs`

- [ ] Function `and_modify`
- [ ] Function `insert_entry`
- [ ] Function `into_mut`
- [ ] Function `or_default`
- [ ] Function `take`
- [ ] Struct `OccupiedComponentEntry`
- [ ] Struct `VacantComponentEntry`

### `world/entity_access/except.rs`

- [ ] Function `as_readonly`
- [ ] Function `as_unsafe_entity_cell`
- [ ] Function `contains_id`
- [ ] Function `contains_id`
- [ ] Function `contains_type_id`
- [ ] Function `contains_type_id`
- [ ] Function `get_by_id`
- [ ] Function `get_by_id`
- [ ] Function `get_change_ticks_by_id`
- [ ] Function `get_change_ticks`
- [ ] Function `get_mut_by_id`
- [ ] Function `get_ref`
- [ ] Function `get_ref`
- [ ] Function `spawn_tick`
- [ ] Function `spawn_tick`
- [ ] Function `spawned_by`
- [ ] Function `spawned_by`
- [ ] Struct `EntityMutExcept`
- [ ] Struct `EntityRefExcept`

### `world/entity_access/filtered.rs`

- [ ] Enum `TryFromFilteredError`
- [ ] Function `archetype`
- [ ] Function `archetype`
- [ ] Function `as_readonly`
- [ ] Function `as_unsafe_entity_cell`
- [ ] Function `contains_id`
- [ ] Function `contains_id`
- [ ] Function `contains_type_id`
- [ ] Function `contains_type_id`
- [ ] Function `get_by_id`
- [ ] Function `get_by_id`
- [ ] Function `get_change_ticks_by_id`
- [ ] Function `get_change_ticks_by_id`
- [ ] Function `get_change_ticks`
- [ ] Function `get_change_ticks`
- [ ] Function `get_mut_by_id_unchecked`
- [ ] Function `get_mut_by_id`
- [ ] Function `get_mut_unchecked`
- [ ] Function `get_ref`
- [ ] Function `get_ref`
- [ ] Function `into_mut_assume_mutable`
- [ ] Function `into_mut`
- [ ] Function `into_mut`
- [ ] Function `location`
- [ ] Function `location`
- [ ] Function `spawn_tick`
- [ ] Function `spawn_tick`
- [ ] Function `spawned_by`
- [ ] Function `spawned_by`
- [ ] Struct `UnsafeFilteredEntityMut`

### `world/entity_access/world_mut.rs`

- [ ] Function `archetype`
- [ ] Function `as_mutable`
- [ ] Function `as_readonly`
- [ ] Function `clone_and_spawn_with_opt_in`
- [ ] Function `clone_and_spawn_with_opt_out`
- [ ] Function `clone_and_spawn`
- [ ] Function `clone_components`
- [ ] Function `clone_with_opt_in`
- [ ] Function `clone_with_opt_out`
- [ ] Function `contains_id`
- [ ] Function `contains_type_id`
- [ ] Function `despawn_no_free`
- [ ] Function `entry`
- [ ] Function `get_by_id`
- [ ] Function `get_change_ticks_by_id`
- [ ] Function `get_change_ticks`
- [ ] Function `get_components_mut_unchecked`
- [ ] Function `get_components_mut`
- [ ] Function `get_components`
- [ ] Function `get_mut_assume_mutable_by_id`
- [ ] Function `get_mut_assume_mutable`
- [ ] Function `get_mut_by_id`
- [ ] Function `get_ref`
- [ ] Function `get_resource_mut`
- [ ] Function `get_resource`
- [ ] Function `insert_by_id`
- [ ] Function `insert_by_ids`
- [ ] Function `insert_if_new`
- [ ] Function `insert_with_relationship_hook_mode`
- [ ] Function `into_borrow_by_id`
- [ ] Function `into_borrow`
- [ ] Function `into_components_mut_unchecked`
- [ ] Function `into_components_mut`
- [ ] Function `into_mut_assume_mutable_by_id`
- [ ] Function `into_mut_assume_mutable`
- [ ] Function `into_mut_by_id`
- [ ] Function `into_mut`
- [ ] Function `into_mutable`
- [ ] Function `into_readonly`
- [ ] Function `into_ref`
- [ ] Function `into_world_mut`
- [ ] Function `is_despawned`
- [ ] Function `is_spawned`
- [ ] Function `location`
- [ ] Function `modify_component_by_id`
- [ ] Function `modify_component`
- [ ] Function `move_components`
- [ ] Function `observe`
- [ ] Function `reborrow_scope`
- [ ] Function `remove_by_id`
- [ ] Function `remove_by_ids`
- [ ] Function `remove_with_requires`
- [ ] Function `resource_mut`
- [ ] Function `resource_scope`
- [ ] Function `resource`
- [ ] Function `retain`
- [ ] Function `spawn_tick`
- [ ] Function `spawned_by`
- [ ] Function `take`
- [ ] Function `try_archetype`
- [ ] Function `try_location`
- [ ] Function `try_resource_scope`
- [ ] Function `update_location`
- [ ] Function `world_scope`

### `world/entity_fetch.rs`

- [ ] Struct `EntityFetcher`

### `world/error.rs`

- [ ] Enum `EntityComponentError`
- [ ] Enum `ResourceFetchError`
- [ ] Struct `EntityDespawnError`
- [ ] Struct `TryInsertBatchError`
- [ ] Struct `TryRunScheduleError`

### `world/filtered_resource.rs`

- [ ] Function `add_read_all`
- [ ] Function `add_read_all`
- [ ] Function `add_read_by_id`
- [ ] Function `add_read_by_id`
- [ ] Function `add_write_all`
- [ ] Function `add_write_by_id`
- [ ] Function `as_readonly`
- [ ] Function `get_by_id`
- [ ] Function `get_by_id`
- [ ] Function `get_mut_by_id`
- [ ] Function `has_read`
- [ ] Function `has_read`
- [ ] Function `has_write`
- [ ] Function `into_mut_by_id`
- [ ] Function `into_mut`
- [ ] Struct `FilteredResourcesBuilder`
- [ ] Struct `FilteredResourcesMutBuilder`
- [ ] Struct `FilteredResourcesMut`
- [ ] Struct `FilteredResources`

### `world/mod.rs`

- [ ] Function `add_schedule`
- [ ] Function `allow_ambiguous_component`
- [ ] Function `allow_ambiguous_resource`
- [ ] Function `archetypes`
- [ ] Function `as_unsafe_world_cell_readonly`
- [ ] Function `bundles`
- [ ] Function `change_tick`
- [ ] Function `check_change_ticks`
- [ ] Function `clear_all`
- [ ] Function `clear_entities`
- [ ] Function `clear_resources`
- [ ] Function `clear_trackers`
- [ ] Function `components_queue`
- [ ] Function `components_registrator`
- [ ] Function `contains_non_send_by_id`
- [ ] Function `contains_non_send`
- [ ] Function `contains_resource_by_id`
- [ ] Function `contains_resource`
- [ ] Function `default_error_handler`
- [ ] Function `despawn_no_free`
- [ ] Function `entities_allocator_mut`
- [ ] Function `entities_allocator`
- [ ] Function `entities_and_commands`
- [ ] Function `entities_mut`
- [ ] Function `entity_mut`
- [ ] Function `get_by_id`
- [ ] Function `get_mut_by_id`
- [ ] Function `get_non_send_by_id`
- [ ] Function `get_non_send_mut_by_id`
- [ ] Function `get_non_send_resource_mut`
- [ ] Function `get_non_send_resource`
- [ ] Function `get_required_components_by_id`
- [ ] Function `get_required_components`
- [ ] Function `get_resource_by_id`
- [ ] Function `get_resource_change_ticks_by_id`
- [ ] Function `get_resource_change_ticks`
- [ ] Function `get_resource_mut_by_id`
- [ ] Function `get_resource_mut`
- [ ] Function `get_resource_or_init`
- [ ] Function `get_resource_or_insert_with`
- [ ] Function `get_resource_ref`
- [ ] Function `get_resource`
- [ ] Function `increment_change_tick`
- [ ] Function `init_non_send_resource`
- [ ] Function `init_resource`
- [ ] Function `insert_batch_if_new`
- [ ] Function `insert_batch`
- [ ] Function `insert_non_send_by_id`
- [ ] Function `insert_non_send_resource`
- [ ] Function `insert_resource_by_id`
- [ ] Function `insert_resource`
- [ ] Function `inspect_entity`
- [ ] Function `is_resource_added_by_id`
- [ ] Function `is_resource_added`
- [ ] Function `is_resource_changed_by_id`
- [ ] Function `is_resource_changed`
- [ ] Function `iter_resources_mut`
- [ ] Function `iter_resources`
- [ ] Function `last_change_tick_scope`
- [ ] Function `make_component`
- [ ] Function `modify_component_by_id`
- [ ] Function `modify_component`
- [ ] Function `non_send_resource_mut`
- [ ] Function `non_send_resource`
- [ ] Function `observers`
- [ ] Function `query_filtered`
- [ ] Function `query`
- [ ] Function `read_change_tick`
- [ ] Function `register_bundle`
- [ ] Function `register_component_hooks_by_id`
- [ ] Function `register_component_hooks`
- [ ] Function `register_component_with_descriptor`
- [ ] Function `register_component`
- [ ] Function `register_disabling_component`
- [ ] Function `register_dynamic_bundle`
- [ ] Function `register_required_components_with`
- [ ] Function `register_required_components`
- [ ] Function `register_resource_with_descriptor`
- [ ] Function `register_resource`
- [ ] Function `remove_non_send_by_id`
- [ ] Function `remove_non_send_resource`
- [ ] Function `remove_resource_by_id`
- [ ] Function `remove_resource`
- [ ] Function `removed_components`
- [ ] Function `removed_with_id`
- [ ] Function `removed`
- [ ] Function `resource_id`
- [ ] Function `resource_mut`
- [ ] Function `resource_ref`
- [ ] Function `resource_scope`
- [ ] Function `resource`
- [ ] Function `run_schedule`
- [ ] Function `schedule_scope`
- [ ] Function `spawn_at`
- [ ] Function `spawn_batch`
- [ ] Function `spawn_empty_at`
- [ ] Function `storages`
- [ ] Function `try_despawn_no_free`
- [ ] Function `try_despawn`
- [ ] Function `try_insert_batch_if_new`
- [ ] Function `try_insert_batch`
- [ ] Function `try_query_filtered`
- [ ] Function `try_query`
- [ ] Function `try_register_required_components_with`
- [ ] Function `try_register_required_components`
- [ ] Function `try_resource_scope`
- [ ] Function `try_run_schedule`
- [ ] Function `try_schedule_scope`
- [ ] Function `write_message_batch`
- [ ] Function `write_message_default`
- [ ] Function `write_message`

### `world/reflect.rs`

- [ ] Function `get_reflect_mut`
- [ ] Function `get_reflect`

### `world/spawn_batch.rs`

- [ ] Struct `SpawnBatchIter`

### `world/unsafe_world_cell.rs`

- [ ] Function `archetype`
- [ ] Function `archetypes`
- [ ] Function `bundles`
- [ ] Function `change_tick`
- [ ] Function `contains_id`
- [ ] Function `contains_type_id`
- [ ] Function `default_error_handler`
- [ ] Function `entities_allocator`
- [ ] Function `get_by_id`
- [ ] Function `get_change_ticks_by_id`
- [ ] Function `get_change_ticks`
- [ ] Function `get_entity_with_ticks`
- [ ] Function `get_mut_assume_mutable_by_id`
- [ ] Function `get_mut_assume_mutable`
- [ ] Function `get_mut_by_id`
- [ ] Function `get_non_send_resource_by_id`
- [ ] Function `get_non_send_resource_mut_by_id`
- [ ] Function `get_non_send_resource_mut`
- [ ] Function `get_non_send_resource`
- [ ] Function `get_ref`
- [ ] Function `get_resource_by_id`
- [ ] Function `get_resource_mut_by_id`
- [ ] Function `get_resource_mut`
- [ ] Function `get_resource_ref`
- [ ] Function `get_resource`
- [ ] Function `increment_change_tick`
- [ ] Function `last_trigger_id`
- [ ] Function `location`
- [ ] Function `removed_components`
- [ ] Function `spawn_tick`
- [ ] Function `spawned_by`
- [ ] Function `storages`
- [ ] Function `world_metadata`
- [ ] Struct `UnsafeEntityCell`


