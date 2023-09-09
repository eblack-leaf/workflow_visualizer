use std::collections::{HashMap, HashSet};

use crate::text::component::{Glyph, GlyphId};

use crate::texture_atlas::{AtlasLocation, TextureCoordinates};
use crate::{Area, NumericalContext};

pub(crate) struct AtlasGlyphReference {
    pub(crate) count: u32,
}

impl AtlasGlyphReference {
    pub(crate) fn new() -> Self {
        Self { count: 0 }
    }
    pub(crate) fn increment(&mut self) {
        self.count += 1;
    }
    #[allow(unused)]
    pub(crate) fn decrement(&mut self) {
        let sub_value = (self.count == 0) as u32;
        self.count -= sub_value;
    }
}

pub(crate) type Bitmap = Vec<u8>;
pub(crate) struct AtlasGlyphReferences {
    pub(crate) references: HashMap<GlyphId, AtlasGlyphReference>,
}

impl AtlasGlyphReferences {
    pub(crate) fn new() -> Self {
        Self {
            references: HashMap::new(),
        }
    }
}
pub(crate) struct AtlasWriteQueue {
    pub(crate) queue: HashMap<AtlasLocation, (TextureCoordinates, Area<NumericalContext>, Bitmap)>,
}

impl AtlasWriteQueue {
    pub(crate) fn new() -> Self {
        Self {
            queue: HashMap::new(),
        }
    }
}
pub(crate) struct AtlasAddQueue {
    pub(crate) queue: HashSet<Glyph>,
}

impl AtlasAddQueue {
    pub(crate) fn new() -> Self {
        Self {
            queue: HashSet::new(),
        }
    }
}
pub(crate) struct AtlasGlyphs {
    pub(crate) glyphs: HashMap<
        GlyphId,
        (
            TextureCoordinates,
            Area<NumericalContext>,
            AtlasLocation,
            Bitmap,
        ),
    >,
}

impl AtlasGlyphs {
    pub(crate) fn new() -> Self {
        Self {
            glyphs: HashMap::new(),
        }
    }
}
