# SyncPoints

To parallelize as much as possible a `Task` will run its systems 
alongside other systems that can be run while preserving order of
mutable access to components and resources. This can be configured using
`SyncPoint`s. 

### Core SyncPoints
```rust
pub enum SyncPoint {
    Event,
    Initialization,
    Config,
    Preparation,
    Spawn,
    Reconfigure,
    ResolveVisibility,
    Resolve,
    PushDiff,
    Finish,
}
```
There are many stages to the `Task`s that are run on the visualizer's `Job`.
Here is the overview of sync points. 
- `JobSyncPoint::Idle` set holds `attempt_to_idle` to set idling behavior.
- `SyncPoint::Event` set holds all the event updaters to process new events.
- `SyncPoint::Initialization` set is for preparation of resources and entities.
- `UserSpaceSyncPoint::Initialization` set in for user initialization.
- `SyncPoint::Config` set is for configuring the initialized parts.
- `SyncPoint::Preparation` set is for preparing parts according to the config.
- `UserSpaceSyncPoint::Process` set is for user fns.
- `SyncPoint::Spawn` set is for spawning new bundles via `Request`s.
- `SyncPoint::Reconfigure` set is for config new parts spawned
- `SyncPoint::ResolveVisibility` set is for determining visibility.
- `SyncPoint::Resolve` set is for resolving diffs in parts after changes.
- `UserSpaceSyncPoint::Resolve` set is for resolving user fns.
- `SyncPoint::PushDiff` set is for extracting the differences in element state.
- `SyncPoint::Finish` set is for cleanup after extraction.


### UserSpace SyncPoints

These sync points are put in place to be correct within the context of the 
core sync points and provide clean sets where the user can add systems. When implementing
`Attach::attach` these are the main sets to be concerned with for correct execution.
```rust
pub enum UserSpaceSyncPoint {
    Initialization,
    Process,
    Resolve,
}
```
###### UserSpaceSyncPoint::Initialization
This gives a spot for the user to attach systems that initialize data and is ensured to run
after core visualizer elements have been initialized.
###### UserSpaceSyncPoint::Process
This should be used to compute and update any values that need doing once per frame.
This runs after input/events have been processed and focus/visibility set. This is 
the only spot to request new entities from and have them spawn immediately. This lib
uses a [`Request`](request.md) to wrap any new bundles and when paired with a 
`request::spawn::<Bundle>(...)` system inserted in the `SyncPoint::Spawn` set, it will
correctly spawn to have all pipelines process appropriately. Due to the nature of 
ECS pattern it can be hard to recognize when a component will run in a system unintentionally
before it has all its parts which would opt the entity out of the unintentional system.
It is clearer to define a boundary when all need to be ready and use `Request` to wrap 
your bundles.
```rust
let bundle = ...;
let req = Request::new(bundle);
visualizer.add_entities(vec![req]);
visualizer.job.task(/* TaskLabel */)
    .add_systems((request::spawn::<Bundle>.in_set(SyncPoint::Spawn),));
```
###### UserSpaceSyncPoint::Resolve

This should be used to read the state after all elements have been spawned and
processed, but before sending data to the render `Extraction`s which check a
`Cache` and decide what to give to the render task. This is useful for reacting to changes
made by the system started in `UserSpaceSyncPoint::Process`.

### Labeled Tasks
The `Visualizer` labels `Task`s that handle preparation for the system to run. Some of these tasks have
a limited set of these `SyncPoint`s as they need to handle less ambiguity of logic. You can
refer to these by using the associated constants on the `Visualizer` impl.
```rust
visualizer.job.task(Visualizer::TASK_MAIN).add_systems((...));
```
##### Visualizer::TASK_STARTUP
This task is run once after initialization of the `Attachment`s queued in the visualizer.

- `SyncPoint::Initialization`
- `UserSpaceSyncPoint::Initialization`
- `SyncPoint::Preparation`
- `SyncPoint::Resolve`
- `UserSpaceSyncPoint::Resolve`
- `SyncPoint::Finish`

##### Visualizer::TASK_MAIN
This task is run every iteration of the event loop, while `!job.suspended()`.

- `JobSyncPoint::Idle`
- `SyncPoint::Event`
- `SyncPoint::Initialization`
- `UserSpaceSyncPoint::Initialization`
- `SyncPoint::Preparation`
- `UserSpaceSyncPoint::Process`
- `SyncPoint::Spawn`
- `SyncPoint::Reconfigure`
- `SyncPoint::ResolveVisibility`
- `SyncPoint::Resolve`
- `UserSpaceSyncPoint::Resolve`
- `SyncPoint::PushDiff`
- `SyncPoint::Finish`

##### Visualizer::TASK_RENDER_STARTUP
This task is run once after the initialization of the `Attachment`s queued in the visualizer.

- `SyncPoint::Initialization,`
- `UserSpaceSyncPoint::Initialization,`
- `SyncPoint::Preparation,`
- `SyncPoint::Resolve,`
- `UserSpaceSyncPoint::Resolve,`
- `SyncPoint::Finish,`

##### Visualizer::TASK_RENDER_MAIN
This task is run every time a redraw is requested.

- `JobSyncPoint::Idle,`
- `SyncPoint::Initialization,`
- `UserSpaceSyncPoint::Initialization,`
- `SyncPoint::Preparation,`
- `SyncPoint::Resolve,`
- `UserSpaceSyncPoint::Resolve,`
- `SyncPoint::Finish,`


The `Task`s labeled "*_MAIN" have an extra set `JobSyncPoint::Idle` to communicate with 
the control_flow of the event loop.