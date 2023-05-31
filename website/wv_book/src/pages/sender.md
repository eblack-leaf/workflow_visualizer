# Sender

### Definition

```rust
pub struct Sender<T: Workflow + Default + 'static> { /* private fields */ }
```

The sender is used by the application to send actions.

```rust
pub fn send(&self, action: <T as Workflow>::Action) {}
```

is its only method and can be accessed from a `NonSend` resource in the visualizer's [`Job`](job.md).

### Usage

```rust
struct Engen {
    // your app data
}
enum Action {
    Notify(String),
}
impl Workflow for Engen {
    // ...
    type Action = Action;
    // ...
}
fn system(sender: NonSend<Sender<App>>, /* other params */) {
    // do stuff that needs to send action ...
    sender.send(Action::Notify(...));
}
```

This action will be sent to the application thread and `Workflow::handle_action` will be run to give you 
space to respond with what is needed.