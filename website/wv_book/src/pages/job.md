# Job 

A `Job` is a struct for executing `Task`s which run functions on data in a `Container`. 

### Container
A `Container` can hold `Res` or resources which have one slot per type, or it can hold
`Entity`s composed of many types associating them with a unique id to reference the parts.
See [`bevy_ecs::World`]() for more information.
### Task
A `Task` is a scheduled set of fns to run on the `Container`. These fns can be grouped into `SystemSet`s to
synchronize when parts of the schedule should be done before/after other sets.
```rust
#[derive(SystemSet)]
enum Sets {
    First,
}
let task = Task::new();
task.add_systems((some_fn.in_set(Sets::First)));
```
Which would add `some_fn` to the set `First`. 
This is a thin wrapper around [`bevy_ecs::Schedule`](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Schedule.html).
##### Execution State 
A `Job` includes an `ExecutionState` to allow for `Visualizer::suspend()` and `Visualizer::resume()` to be pause execution on
mobile platforms when minimizing the app. 
```rust
job.suspend();// sets ExecutionState to Suspended
job.resume();// sets ExecutionState to Resumed
```

#### Storing Entities

A `Job` can store entities associated with an id to reference specific entities in systems.
```rust
let entity = job.container.spawn(/* bundle */).id();
job.store_entity("special", entity); 
// ...
let data = job.get_entity::<Components>("special").unwrap();
```

##### Exit | Idle

On creation a `Job` inserts `Exit` and `Idle` resources to aid in control flow.
###### Idle 
`Idle` can be set in systems by bringing in the resource
```rust
fn system(idle: ResMut<Idle>) {
    idle.can_idle = false;
}
```
When set to `false` the `Runner` sets the control flow to poll again.
When set to `true` the `Runner` sets the control flow to wait for user input 
or hardware events. A system is inserted to attempt to set `Idle.can_idle` to `true`
at the beginning of every frame. You can set this to `false` any time in your loop to 
signal that execution should keep running even without input, such as in animations.
###### Exit
`Exit` can be set in a similar way.
```rust
fn system(exit: ResMut<Exit>) {
    exit.request_exit();
}
```
This will signal the event loop that it should exit and send `Workflow::exit_action()` to 
your application to gracefully handle shutdown.
