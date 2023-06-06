# Touch

This module handles input from mouse clicks/touchscreen touches. Both of these events get
distilled to a `TouchEvent` to handle both input methods uniformly. The `Visualizer` registers
these events and sends them to the `Job` to be processed. Any `Touch` that is not grabbed by a
`TouchListener` resets `Focus` and clears the `FocusedEntity`. If the `Touch` is within the bounds
of the `Area` of an entity that opt in to have a `TouchListener`, it is grabbed and prevents any underlying
entities from receiving that `Touch`. 

### Touch Listeners

Entities can listen to `OnPress` and `OnRelease` touch types that trigger when a `Touch` begins in the area,
or when both the beginning and end of the `Touch` are within the bounds. This is to prevent errant presses from
triggering action when not intended. 3 aspects can be read from this input for each entity.
`TouchedState` holds whether it is currently pressed. This lasts for one loop then is reset in `SyncPoint::Finish`.
`ToggleState` tracks toggling of the entity. One press enables and the consecutive press disables.
`TouchLocation` can be read to see where the press was received within the `Area`.

```rust
Touchable::new(TouchListener::on_press())
// or
Touchable::new(TouchListener::on_release())
```

### Tracking

On touchscreens multiple `Interactor`s can be active at the same time. This can be unintentional such as 
resting your hand on the phone screen when reaching for a button. This cannot happen on mouse devices as the
mouse is the only `Interactor`. To prevent glitch presses due to extra touch interaction, the first interactor
is set as the `Primary` interactor and all other presses are tracked but secondary and not considered for
triggering events. Once the `Primary` is released, other presses can be activated as the primary again.

### Adapters

Each source has an adapter which can be used to read the current state of presses. `TouchAdapter` and `MouseAdapter` fill
these roles. 