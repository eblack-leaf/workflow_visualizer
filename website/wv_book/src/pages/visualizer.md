# Visualizer

The `Visualizer` handles displaying visual elements needed by the application. This uses a [`Job`](job.md) to create
an extensible container that holds tasks for running fns on the data. This is used to run a render pass that invokes
any render fns attached to the visualizer and presents to the surface of window initialized by the [`Runner`](runner.md).
The visualizer is composed of many parts which build on each other to provide a specific solution of a responsive UI
and is best explained by delving into each section specifically. The next section is [`Job`](job.md) to start us off.
This is the central glue for all the various parts. Each module attached to the `Visualizer` has been
vetted to elide repetitive systems from running when no updating has been done that could
change each aspect thanks to the `Filter` system paired with `ChangeDetection` that tracks 
mutable access to components. 
### Interface Overview
##### Init
First is the `Visualizer::new(...)`.
This creates 5 main `Task`s that help the visualizer handle interactions, and respond accordingly.
Next is the option to `Visualizer::set_gfx_options(...)` which can be used to reset the `GfxOptions`
after instantiation. There is an `async Visualizer::init_gfx(...)` which is used to create the `Gfx` module and `Viewport`.
This is the core initialization fn.
Attachments need to be invoked to insert into the `Job`. This can be accomplished with
`Visualizer::initialize(...)`.
##### Attachments
With `Visualizer::add_attachment::<impl Attach>()` the system can be extended with logic handlers.
Using `Visualizer::register_renderer::<impl Render>()` the system can add render fns to the render pass.
##### Helpers
You can `Visualizer::trigger_resize(...)` to resize the viewport and surface configuration.
The `Theme` can be changed via `Visualizer::set_theme(...)`.
Entities can be added by invoking `Visualizer::add_entities(...)` or `Visualizer::add_named_entities(...)`.
Scale factor can be set via `Visualizer::set_scale_factor(...)`.
##### Input
Touch events can be registered with `Visualizer::register_touch(...)`.
Mouse clicks can be registered with `Visualizer::register_mouse_click(...)`.
Mouse location can be set with `Visualizer::set_mouse_location(...)`.
Touches/Clicks can be cancelled with `Visualizer::cancel_touches()`.
##### Control Flow
A `Visualizer` can be controlled with `::suspend()` | `::resume()` | `::setup()` 
| `::exec()` | `::render()` | `::teardown()`


See [API](../../doc/workflow_visualizer/index.html) doc for full reference.