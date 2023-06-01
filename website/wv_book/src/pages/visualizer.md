# Visualizer

The `Visualizer` handles displaying visual elements needed by the application. This uses a [`Job`](job.md) to create
an extensible container that holds tasks for running fns on the data. This is used to run a render pass that invokes
any render fns attached to the visualizer and presents to the surface of window initialized by the [`Runner`](runner.md).
The visualizer is composed of many parts which build on each other to provide a specific solution of a responsive UI
and is best explained by delving into each section specifically. The next section is [`Job`](job.md) to start us off. 

#### Interface Overview

see API doc for full reference