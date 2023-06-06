# Text Renderer

The text renderer is quite opinionated to enable some memory optimizations.
This lib uses [`fontdue`](https://github.com/mooman219/fontdue) to rasterize glyphs to a `Bitmap` that contains a 
coverage value that signifies how much the letter shape covers that location. This
is stored in an `Atlas` of glyph bitmaps and sampled from in a shader. Each time a `Text`
element updates its `String` value, the system checks for a differential in the letters
at each location and obtains the `TextureCoords` for where in the `Atlas` that shape exists.
The shader matches the texture coords to a bbox with dimensions determined by the letter 
shape and offset. Each letter needs to be stored just once per `Text` element and can be 
referenced multiple times without needing to store again. When changing text frequently
glyphs can be freed from the `Atlas` if not referenced any longer. This requires removing
part of the `wgpu::Texture` contents and when needing to insert a new glyph organization can 
be tricky if the size of the new glyph is not aligned. Padding is inserted to give each
glyph a consistent slot to inhabit. With mono-spaced fonts each character has the same
dimensions which allows this padding to be minimal as each shape size is known/consistent.
If one shape were bigger we could not know when the `Atlas` would need to put the bigger shape at a 
location and have to default to every slot being as big as the largest character. Memory
can be saved by only using mono-spaced fonts and by limiting the sizes of fonts
to a set number. 3 sizes of `Text` can be used which have been calibrated to be reasonable
on most devices. This fixed number creates consistent UI elements as well by not having
jitter using fractional sizing differences. This is a perfect symbiosis of performance and
aligning style to meet functionality. 

### Usage

To spawn a `Text` element, use the `TextBundle` 

```rust
pub struct TextBundle {
    pub pos: Position<InterfaceContext>,
    pub area: Area<InterfaceContext>,
    pub layer: Layer,
    pub text: Text,
    pub scale_alignment: TextScaleAlignment,
    pub color: Color,
    pub wrap_style: TextWrapStyle,
    /* private fields */
}
```

This will create the required components to hook this entity into the 
text pipeline. All elements employ `Cache` and `Extraction` to keep changes to 
the gpu minimal. Wrap this bundle in a `Request` to ensure it gets spawned at the 
right time.

```rust
visualizer.add_entities(vec![Request::new(TextBundle::new(...))]);
```

### Alignments

```rust
pub enum TextScaleAlignment {
    Small,
    Medium,
    Large,
}
```

### Implementation Notes

When rendering on devices with a `ScaleFactor` not equal to `1`, the `Font` size is scaled before rasterization of `Glyph`s as stretching `Bitmap`s of the 
glyph afterward to account for scaling would cause undesirable visual artifacts. `TextLetterDimensions` holds `Area<DeviceContext>` to illustrate that
the letter sizes already have scaling accounted for.