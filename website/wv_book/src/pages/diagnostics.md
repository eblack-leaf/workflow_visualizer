# Diagnostics

This crate has a feature `diagnostics` that can be enabled to compile the
`DiagnosticsHandle<T>` parameters to subscribing systems in the `Visualizer`.
This adds a local storage resource that records information as the system runs.
This is output using `trace!` macro to show the system count with `@sys.counter:<number-times-exec>` and
whatever `ext` data the system has decided to record. A trait
`Record` is available to instrument recorders and can be passed to a
`Local<DiagnosticsHandle<impl Record>>` to be used in a system. `Diagnostics` can be used
as an interpreter for `trace!` messages to extract the messages into structured data
to be read visually with the help of `DiagnosticsVisualizer`

### Diagnostics Visualizer

Work-In-Progress to better navigate `trace!` diagnostics messages.