# Workflow Visualizer

The crate workflow_visualizer is a rust lib for cross-platform UI applications.
The structure of this lib works by creating a headless app with actions and responses
that trigger reactions in the UI layer. 

## Overview

### Workflow

The process starts with defining your workflow. This is done by implementing [`Workflow`](./workflow.md).

```rust 
struct Engen {
    // your data here to run app
}
impl Workflow for Engen {
    type Action = ();
    type Response = ();
    fn handle_response(visualizer: &mut Visualizer, response: Self::Response) {
        // trigger actions in the visualizer to show the effects of responses
    }
    fn handle_action<'async_trait>(
        engen: Arc<Mutex<Self>>,
        action: Self::Action
    ) -> Pin<Box<dyn Future<Output = Self::Response> + Send + 'async_trait>>
        where
            Self: 'async_trait {
        // handle any engen tasks related to an action
        // ...async connect to a database for user data
        // ...fetch content from network request
        // return a Response to signal to the Visualizer
    }
    fn exit_action() -> Self::Action{
        // tell visualizer what action is the exit action 
    }
    fn exit_response() -> Self::Response{
        // tell visualizer what response is the exit response
    }
}
```
This establishes a communication protocol between a UI thread and a thread run in background for
your application actions. This is the structure of how an app runs using workflow_visualizer. 
This is a headless app as it only concerns itself with actions in to your app and the
responses that it generates. For this app to do anything visually we need another part.

### Visualizer 

Next we have to instantiate a [`Visualizer`](visualizer.md).

```rust
let theme = Theme::default();
let gfx_options = GfxOptions::native_defaults();
let mut visualizer = Visualizer::new(theme, gfx_options);
// config visualizer ...
```

This is a suite of rendering tools that are attached to a [`Job`](job.md). A `Job` is a `Container` for data
and a set of `Task`s to run functions on the container. One important purpose for a job is to collect various render pipelines
and provide a structure to create a render pass, and call any render functions that are attached to the job; 
see [`Render`](render.md). The visualizer also interprets input actions
such as providing listeners for touches/mouse input; see [`Touch`](touch.md). The visualizer has
a [`Viewport`](viewport.md) to convert screen coordinates to NDC coordinates used by Vulkan | DirectX12 | Metal.
This is used by renderers to correctly position elements using [`Coord`](coord.md) system which accounts for
scale factor of the device by using different `CoordContext`s. [`Visibility`](visibility.md) can be determined by 
reading from an elements associated component. Visible elements can receive [`Focus`](focus.md) to show
on-screen keyboard using `VirtualKeyboard` and receive input. Prebuilt core render pipelines are included
by default such as [`TextRenderer`](text_renderer.md), which is a memory-efficient glyph-caching text renderer which can be 
utilized by spawning a `Text` element. Other pipelines are available that are common to UI applications. 
If nothing quite solves your desired effect you can easily integrate your own renderer by implementing 
[`Render`](render.md) to setup the render function and [`Attach`](./attach.md) to configure how the renderer attaches to the 
visualizer's [`Job`](job.md).

### Runner

All of this structure needs an entry point. The [`Runner`](./runner.md) is responsible for establishing
a connection to the compositor on the platform and obtaining a window to draw within. The runner forwards events from 
the connection and links it to the appropriate calls into the visualizer. This event loop never returns control due to platform 
implementation details. A background thread is spawned before entering this loop and a bridge created using the definitions 
in [`Workflow`](./workflow.md) trait. To communicate to the application thread, a [`Sender`](./sender.md) can be used to 
send actions. This process should return a response and the UI thread will receive this response and react 
accordingly.

#### Native Run

When running natively on desktop, dimensions can be specified to get a fixed size to develop with.

```rust
Runner::new()
    .with_desktop_dimensions((400, 600))
    .native_run::<Engen>(visualizer);
```

This is all that is needed to run on desktop.

The runner also supports Android applications. To achieve this a few steps are needed.
To link with the android lifecycle the implementation refers to an Android
compatibility struct called AndroidApp. This serves as a pointer to android-activity that powers the event loop on Android and allows
accessing system functionality.
This must be given to a separate main function and then passed to the visualizer.
```rust
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(android_app: AndroidApp) {
    tracing_subscriber::fmt().init();
    let mut visualizer = visualizer::visualizer();
    visualizer.set_gfx_options(GfxOptions::limited_environment());
    Runner::new()
        .with_android_app(android_app)
        .native_run::<Engen>(visualizer);
}
```
The android entry point must be defined in Cargo.toml as a dynamic lib.
```toml
[lib]
name = "application"
crate-type = ["cdylib"]
path = "src/main.rs"
```
The application must extend GameActivity in Java and compile using cargo-ndk. See [Platform Specifics](./platform_specifics.md) for more information.

#### Web Run 

To run on the web, extra setup is needed. Web workers allow separate processes in the browser and are taken advantage of 
in this lib. 

A main function to start a web worker must be defined.
```rust
fn main() {
    start_web_worker::<Engen>();
}
```
A common build program for wasm is Trunk. Trunk supports web workers and can be configured by putting
```toml
[[bin]]
name = "worker"
path = "src/worker_main.rs"
```
in your Cargo.toml and
```html
<link data-trunk rel="rust" href="Cargo.toml" data-wasm-opt="z" data-bin="worker" data-type="worker" />
```
in the index.html that serves as entry point for your Trunk build. Due to how web worker api is formed you need to
tell the visualizer the path of the web worker as so
```rust
Runner::new().web_run::<Engen>(visualizer, "./worker.js".to_string());
```
The path in Trunk builds is the name of the binary defined in the index.html. See [Platform Specifics](./platform_specifics.md) for more details.

###### To recap we have three major parts.

Workflow
: trait for communication between application | UI thread.

Visualizer
: tools for rendering + connecting responses to interactions

Runner
: struct for establishing event loop connection + run
