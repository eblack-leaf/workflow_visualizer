# Time

A `Timer` is present to mark things that should happen over time.
One can obtain a `TimeMarker` at a specific point in time.
```rust
fn system(timer: Res<Timer>) {
    let now = timer.mark();
}
```

The difference in `TimeMarker`s can be obtained by 
```rust
timer.time_since(...);
```

The frame time can be obtained by using
```rust
let frame_time = timer.frame_diff();
```

