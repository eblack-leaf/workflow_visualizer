# Runner

The `Visualizer` can be used without a `Runner` to have full control of the event_loop. One would have to 
call the triggers associated with `Visualizer` to correctly use the struct and for usability, a `Runner` is 
provided as a useful solution to invoke the visualizer's tools.

A `Runner` has little data so instantiation is simple
```rust
let runner = Runner::new();
```

Natively it temporarily holds `desktop_dimensions` and `AndroidApp` to forward to the event
loop.

```rust
Runner::new()
        .with_desktop_dimensions((400, 600))
        .native_run::<Engen>(visualizer);
```

can be used to set fixed dimensions on desktop.

```rust
Runner::new()
        .with_android_app(android_app)
        .native_run::<Engen>(visualizer);
```

for Android, the system needs to be passed an `AndroidApp` to interact with the 
Android OS and invoke commands.

```rust
Runner::new().web_run::<Engen>(visualizer, "./worker.js".to_string());
```

To accomplish a background thread in the browser, the `Runner` must be passed the 
path to a Web Worker. See [introduction](../introduction.md) for web setup.

