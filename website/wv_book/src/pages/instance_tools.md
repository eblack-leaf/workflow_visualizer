# Instance Tools

This came to be when I saw a pattern arise from using instances to achieve performant
rendering. I found I could automate some of the structure needed to have a staging buffer 
to hold changes and only write changed regions. I also found a way to elide rewriting buffers
on removal of instances by employing a `NullBit`. This marks any data for that instance
as invalid and multiplies any attributes to 0 in a shader conditionally culling the 
shapes as gpu drivers discard any fragments with 0 area. Otherwise, we would have to 
remove the element from the drawing range by rewriting all other attributes after the hole.
Instead, I found it easier to just drop the one instance and mark it as a hole to be filled
by the next instance that is needed.

### Instance Attribute Manager

All data for an instance is broken into separate buffers to allow different rates of
writing to the buffer based on how often the data changes. Each has its own gpu resource to
let it update only the parts needing changes. One attribute could be the `Position` of an
element which might be changed frequently when moving, but the color would not change as it
moves. This architecture elides rewriting the colors as well when just the `Position` needs
rewriting. 

Here is an excerpt from the `TextRenderer` which employs usage of `InstanceAttributeManager`.
```rust
glyph_positions: InstanceAttributeManager::new::<AttributeType>(&gfx_surface, *max),
```

It is passed the `GfxSurface` and the `max` elements to hold initially before growing.
The `InstanceAttributeManager` uses the `size_of<T>` * max to get the actual byte size.

You can queue writes to the buffer using
```rust
InstanceAttributeManage::queue_write(...);
``` 

You can grow the buffer when needed using
```rust
InstanceAttributeManager::grow(...);
```
 Finally, you can push the writes to the gpu using
 ```rust
 InstanceAttributeManager::write(...);
 ```


### Key

Sometimes you will need to associate a `Key` with an instance to keep track of its lifetime.
This can be accomplished using a `KeyFactory` and `KeyFactory::generate()` which will
get the next free key unique to the factory.
```rust
let key = key_factory.generate();
```

### Index

The attributes are stored by `Index` in a buffer. This is useful for finding the 
byte offset required by gpu writing commands and for referencing an `Instance`s data to 
nullify it or update it. This is stored in an `Indexer` which takes the `Key` type to 
associate with the `Index`. This is useful because the `Index` may change but the `Key`
reference will point to the right location.

##### Indexer Overview

Indexer can be used as such. 
- `::has_instances()` to check if anything is present.
- `::max()` for the current max.
- `::count()` for the current number of instances.
- `::next(key)` to get the next index and tie it to the key given.
- `::remove(key)` to remove the index associated with the key.
- `::get_index(key)` to get the index from a key.
- `::should_grow()` to signal more instances than max have been requested. If this should
grow then the `::max()` will return the max needed to fit the requested amount.