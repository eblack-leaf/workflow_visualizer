# Introduction
Workflow Visualizer is a rust lib to help get headless apps/cmdline programs visualized using
modern gpu acceleration. 

> Impl Workflow to establish the actions/responses for your program

```rust
struct Engen {
    // any data you need for running application
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

This trait defines the messages that can be signaled between the visualizer and
your program establishing a bridge for communication. This is needed because a responsive 
app needs to run at 16.7ms per frame. To this end the visualizer spawns a background process 
to receive messages when actions need happening, then that thread can take as much time as 
needed doing async calls without interrupting the UI thread. On native platforms this takes form 
of a tokio task. On web targets this takes advantage of web workers which requires extra setup by
an application.

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

This brings me to the next topic the Runner. The Runner is responsible for establishing a connection 
to the platforms compositor, create a window to render within and run a loop to receive events from this connection.
The runner cannot exit this event loop, so it spawns the background thread before entering using the definitions provided in the 
Workflow trait.
To run natively
```rust
Runner::new()
        .with_desktop_dimensions((400, 600))
        .native_run::<Engen>(visualizer);
```
This is called with desktop_dimensions as it is the only platform that supports custom window sizing.
Mobile/Web targets must use fullscreen due to how they are structured. 

Android has a special requirement as well. To link with the android lifecycle the implementation refers to an Android
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
The application must extend GameActivity in Java to compile using cargo-ndk. See [Platform Specifics](./platform_specifics.md) for more information.
