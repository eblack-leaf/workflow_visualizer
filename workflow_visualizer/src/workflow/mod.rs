use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub use bridge::Sender;
#[cfg(target_os = "android")]
pub use native::AndroidInterface;
pub use runner::Runner;
pub use web::start_web_worker;

use crate::Visualizer;

mod bridge;
mod native;
mod run;
mod runner;
mod web;
/// Main trait to establish communication between the app and UI thread.
#[async_trait]
pub trait Workflow
where
    Self: Default,
{
    /// Input to the app
    type Action: Debug + Clone + Send + Sync + Sized + 'static + Serialize + for<'a> Deserialize<'a>;
    /// Output from the app
    type Response: Debug
        + Clone
        + Send
        + Sync
        + Sized
        + 'static
        + Serialize
        + for<'a> Deserialize<'a>;
    /// configure triggers to the visualizer from responses
    fn handle_response(visualizer: &mut Visualizer, response: Self::Response);
    /// handle actions input to the app
    async fn handle_action(engen: Arc<Mutex<Self>>, action: Self::Action) -> Self::Response;
    fn exit_action() -> Self::Action;
    fn is_exit_response(res: &Self::Response) -> bool;
}
