# Visualizer

The `Visualizer` handles displaying visual elements needed by the application. This uses a [`Job`](job.md) to create
an extensible container that holds tasks for running fns on the data. This is used to run a render pass that invokes
any render fns attached to the visualizer and presents to the surface of window initialized by the [`Runner`](runner.md).
