use crate::canvas::Canvas;
use crate::text::rasterization::descriptor::GlyphDescriptor;
use crate::text::rasterization::{get_reference, GlyphHash, Rasterization};
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Clone)]
pub(crate) struct Remove {
    pub(crate) hash: GlyphHash,
}
fn consecutive_slices(data: Vec<usize>) -> Vec<Vec<usize>> {
    (&(0..data.len()).group_by(|&i| data[i] as usize - i))
        .into_iter()
        .map(|(_, group)| group.map(|i| data[i]).collect())
        .collect()
}
pub(crate) fn remove(rasterization: &mut Rasterization, canvas: &Canvas) {
    let mut checked_indices = HashSet::<usize>::new();
    for remove in rasterization.removes.iter() {
        if get_reference(&rasterization.references, remove.hash) == 0
            && !rasterization.retain_glyphs.contains(&remove.hash)
        {
            checked_indices.insert(*rasterization.descriptor_order.get(&remove.hash).unwrap());
            rasterization.descriptor_order.remove(&remove.hash);
        }
    }
    if checked_indices.is_empty() {
        return;
    }
    let mut checked_indices = checked_indices.iter().map(|u| *u).collect::<Vec<usize>>();
    checked_indices.sort();
    let ranges = consecutive_slices(checked_indices);
    for range in ranges.iter().rev() {
        let mut range_size: usize = 0;
        for placement_index in range.iter() {
            let placement = rasterization
                .descriptors
                .get(*placement_index)
                .unwrap()
                .descriptor;
            range_size += placement.size() as usize;
            rasterization
                .buffer
                .cpu
                .drain(placement.start() as usize..placement.end() as usize);
        }
        rasterization.buffer.gpu_len -= range_size;
        let last = range.last().unwrap();
        for index in *last..(rasterization.descriptors.len() - 1) {
            let glyph_placement: &mut GlyphDescriptor =
                rasterization.descriptors.get_mut(index).unwrap();
            glyph_placement.descriptor.parts[0] -= range_size as u32;
            *rasterization
                .descriptor_order
                .get_mut(&glyph_placement.hash)
                .unwrap() -= range.len();
            rasterization.swapped_glyphs.insert(glyph_placement.hash);
        }
        for index in range.iter() {
            rasterization.descriptors.remove(*index);
        }
    }
    canvas.queue.write_buffer(
        &rasterization.buffer.gpu,
        0,
        bytemuck::cast_slice(&rasterization.buffer.cpu),
    );
    rasterization.removes.clear();
}
