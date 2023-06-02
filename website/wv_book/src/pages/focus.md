# Focus 

This struct is for determining the `Focus` in the UI. A resource `FocusedEntity` exists
that is set when input is set to an entity. It is a simple implementation and put here 
for reference.

```rust
pub struct Focus {
    pub(crate) focused: bool,
}

impl Focus {
    pub fn new() -> Self {
        Self { focused: false }
    }
    pub fn focus(&mut self) {
        self.focused = true;
    }
    pub fn blur(&mut self) {
        self.focused = false;
    }
    pub fn focused(&self) -> bool {
        self.focused
    }
}
```

A `FocusInputListener` can be attached to an entity with `Focus` to signal the 
`VirtualKeyboard` to open when receiving focus.

```rust
visualizer.add_entities(vec![(
    // ... other components
        Focus::new(),
        FocusInputListener {},
    )]);
```