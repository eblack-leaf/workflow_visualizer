# Attach
The `Attach` trait helps integrate the various parts needed to make 
a `Visualizer` work. Simply put it gives you a ref to the `Visualizer` and 
you can do whatever you want with it.
```rust
pub trait Attach {
    fn attach(visualizer: &mut Visualizer);
}
```
It formalizes adding things to the extensible `Container`, and it adds
an indirection between adding things directly to the `Container`. There are
core resources not attached until the `Window` can be obtained, yet renderers 
want to use these structs to make textures and bind groups. The `Visualizer` defers
these calls by wrapping the needed `Attach::attach(...)` in an `Attachment` to 
be invoked when needed.



### Usage

```rust
struct Attachee {}
impl Attach for Attachee {
    fn attach(visualizer: &mut Visualizer) {
        // ...
    }
}
visualizer.add_attachment::<Attachee>();
```
Which queues an

#### Attachment
You wont use this type directly but here it is for reference.
```rust
pub struct Attachment(pub Box<fn(&mut Visualizer)>);

impl Attachment {
    pub fn using<T: Attach>() -> Self {
        Self(Box::new(T::attach))
    }
}
```