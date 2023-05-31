# Workflow

The first part to set up is the trait `Workflow`. This is defined as

```rust
#[async_trait]
pub trait Workflow
where
    Self: Default,
{
    type Action: Debug
        + Clone
        + PartialEq
        + Send
        + Sync
        + Sized
        + 'static
        + Serialize
        + for<'a> Deserialize<'a>;
    type Response: Debug
        + Clone
        + PartialEq
        + Send
        + Sync
        + Sized
        + 'static
        + Serialize
        + for<'a> Deserialize<'a>;
    fn handle_response(visualizer: &mut Visualizer, response: Self::Response);
    fn exit_action() -> Self::Action;
    fn exit_response() -> Self::Response;
    async fn handle_action(engen: Arc<Mutex<Self>>, action: Self::Action) -> Self::Response;
}
```

Lets break this down.

#### Action | Response

```rust
type Action;
type Response;
```

These associated types define what is considered input to your app (`Action`) and the
`Response`s they generate. Responses are the signals for triggering reactions in the UI. 
Having these types allows structured input to the app that clarify what can be sent.

#### Handlers

```rust
async fn handle_action(engen: Arc<Mutex<Self>>, action: Self::Action) -> Self::Response;
fn handle_response(visualizer: &mut Visualizer, response: Self::Response);
```

These define how to handle actions and responses defined above. When the [`Runner`](runner.md) starts
your app, it starts two processes. One for the UI to react to input, and one to run async tasks that
would otherwise block the UI thread until completed. To handle an action the trait needs to define an async fn.
This is possible due to a crate `async_trait` which wraps the async fn in a `Pin<Box<Future<...>>` to make it
compatible with the current way traits are defined. Recently a MVP for integrating async fn in traits was announced
so this will be moot in the coming months. The `handle_response` is sync because it runs inside the UI thread which is 
single threaded due to the nature of how receiving events from the system is implemented.

#### Exit Logic

```rust
fn exit_action() -> Self::Action;
fn exit_response() -> Self::Response;
```

These point the visualizer to the actions/responses that signify exiting. This allows the visualizer to 
stop the UI thread and send a similar response to the application thread to save state and confirm the exit.


## Implementation

Natively this lib uses [`tokio.rs`](https://tokio.rs/) to spawn a sophisticated async runtime which efficiently 
runs asynchronous tasks with performant scheduling. The threads more resemble multi-plexing within a core rather than 
sending it off to a different core.

In the browser, native threads cannot be used for security reasons. Web workers are sandboxed processes that can be 
spawned using javascript to accomplish this need. This has a different trait from the useful crate `gloo_worker` that needs
to be implemented. The `Workflow` mod blanket implements this for any struct that impls `Workflow`. 