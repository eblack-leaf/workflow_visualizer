use std::fmt::Debug;

use bevy_ecs::prelude::{EventReader, Events, IntoSystemConfig, NonSend, Resource};
use gloo_worker::{HandlerId, Worker, WorkerBridge};
use winit::event_loop::EventLoopProxy;

use crate::{SyncPoint, Visualizer, Workflow};
use crate::workflow::runner::EngenHandle;

pub(crate) struct Receiver<T: Send + 'static> {
    #[cfg(not(target_family = "wasm"))]
    pub(crate) receiver: tokio::sync::mpsc::UnboundedReceiver<T>,
    #[cfg(target_family = "wasm")]
    pub(crate) receiver: T,
}

impl<T: Send + 'static> Receiver<T> {
    #[cfg(not(target_family = "wasm"))]
    pub(crate) async fn receive(&mut self) -> Option<T> {
        self.receiver.recv().await
    }
    #[cfg(target_family = "wasm")]
    pub(crate) fn receive(&mut self) {}
}

pub(crate) struct Responder<T: Send + 'static + Debug>(pub(crate) EventLoopProxy<T>);

impl<T: Send + 'static + Debug> Responder<T> {
    pub(crate) fn respond(&self, response: T) {
        self.0.send_event(response).expect("responder");
    }
}
/// Sender is for sending actions to the app from within the visualizer
#[cfg(not(target_family = "wasm"))]
#[derive(Resource)]
pub struct Sender<T: Workflow + Default + 'static> {
    sender: NativeSender<T>,
}
#[cfg(not(target_family = "wasm"))]
impl<T: Workflow> Sender<T> {
    pub(crate) fn new(sender: NativeSender<T>) -> Self {
        Self { sender }
    }
    pub fn send(&self, action: <T as Workflow>::Action) {
        self.sender.send(action);
    }
}
#[cfg(target_family = "wasm")]
#[derive(Resource)]
pub struct Sender<T: Workflow + Default + 'static> {
    sender: WebSender<T>,
}
#[cfg(target_family = "wasm")]
impl<T: Workflow + Default> Sender<T> {
    pub(crate) fn new(sender: WebSender<T>) -> Self {
        Self { sender }
    }
    pub fn send(&self, action: <T as Workflow>::Action) {
        self.sender.send(action);
    }
}

#[cfg(target_family = "wasm")]
pub(crate) struct WebSender<T: Workflow + Default + 'static>(
    pub &'static mut WorkerBridge<EngenHandle<T>>,
);

#[cfg(target_family = "wasm")]
impl<T: Workflow + 'static + Default> WebSender<T> {
    pub(crate) fn send(&self, input: <EngenHandle<T> as Worker>::Input) {
        self.0.send(input);
    }
}

#[cfg(not(target_family = "wasm"))]
pub(crate) struct NativeSender<T: Workflow>(
    pub(crate) tokio::sync::mpsc::UnboundedSender<T::Action>,
);

#[cfg(not(target_family = "wasm"))]
impl<T: Workflow> NativeSender<T> {
    pub(crate) fn new(sender: tokio::sync::mpsc::UnboundedSender<T::Action>) -> Self {
        Self(sender)
    }
    pub(crate) fn send(&self, action: <T as Workflow>::Action) {
        self.0.send(action).expect("native sender.md");
    }
}
///
pub(crate) struct OutputWrapper<T: Workflow + Default + 'static> {
    pub(crate) handler_id: HandlerId,
    pub(crate) response: <EngenHandle<T> as Worker>::Output,
}

impl<T: Workflow + Default + 'static> OutputWrapper<T>
where
    Self: Sized,
{
    pub(crate) fn new(handler_id: HandlerId, response: <EngenHandle<T> as Worker>::Output) -> Self {
        Self {
            handler_id,
            response,
        }
    }
}
