use std::path::Path;

use bevy_ecs::component::Component;
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use wgpu::util::DeviceExt;

use crate::gfx::GfxSurface;
use crate::GpuPosition;

#[derive(Clone)]
pub struct IconMesh {
    pub mesh: Vec<IconVertex>,
}

pub enum BundledIconKeys {
    Box,
}

impl IconMesh {
    pub fn new(mesh: Vec<IconVertex>) -> Self {
        Self { mesh }
    }
    pub fn bundled(icon_key: BundledIconKeys) -> Self {
        Self {
            mesh: match icon_key {
                BundledIconKeys::Box => {
                    let data = read_mesh_bytes(include_bytes!("icons/box.icon_mesh")).unwrap();
                    data
                }
            },
        }
    }
    pub(crate) fn to_gpu(&self, gfx_surface: &GfxSurface) -> GpuIconMesh {
        GpuIconMesh {
            mesh: gfx_surface
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("text vertex buffer"),
                    contents: bytemuck::cast_slice(&self.mesh),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                }),
            length: self.mesh.len() as u32,
        }
    }
}

pub(crate) struct GpuIconMesh {
    pub(crate) mesh: wgpu::Buffer,
    pub(crate) length: u32,
}

#[repr(C)]
#[derive(Component, Copy, Clone, Pod, Zeroable, Default)]
pub struct ColorInvert {
    pub signal: u32,
}

impl ColorInvert {
    pub fn on() -> Self {
        Self { signal: 1 }
    }
    pub fn off() -> Self {
        Self { signal: 0 }
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, Serialize, Deserialize, Debug)]
pub struct ColorHooks {
    pub is_negative_space: f32,
    pub is_hookable: f32,
}

impl ColorHooks {
    pub const NEGATIVE_SPACE: f32 = 1f32;
    pub const HOOKABLE: f32 = 1f32;
    pub const CONSTANT: f32 = 0f32;
    pub const POSITIVE_SPACE: f32 = 0f32;
    pub const fn new(negative_space: f32, hookable: f32) -> Self {
        Self {
            is_negative_space: negative_space,
            is_hookable: hookable,
        }
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, Serialize, Deserialize, Debug)]
pub struct IconVertex {
    pub position: GpuPosition,
    pub color_hooks: ColorHooks,
}

impl IconVertex {
    pub const fn new(position: GpuPosition, color_hooks: ColorHooks) -> Self {
        Self {
            position,
            color_hooks,
        }
    }
}

#[derive(Component, Hash, Eq, PartialEq, Copy, Clone)]
pub struct IconKey(pub &'static str);

#[derive(Component, Clone)]
pub struct IconMeshAddRequest {
    pub icon_key: IconKey,
    pub icon_mesh: IconMesh,
    pub max: u32,
}

impl IconMeshAddRequest {
    pub fn new(icon_key: IconKey, icon_mesh: IconMesh, max: u32) -> Self {
        Self {
            icon_key,
            icon_mesh,
            max,
        }
    }
}

pub fn read_mesh<P: AsRef<Path>>(path: P) -> Option<Vec<IconVertex>> {
    let data = std::fs::read(path).unwrap();
    read_mesh_bytes(data.as_slice())
}

pub fn read_mesh_bytes(data: &[u8]) -> Option<Vec<IconVertex>> {
    if let Ok(vec) = rmp_serde::from_slice(data) {
        return Some(vec);
    }
    None
}

pub fn write_mesh<P: AsRef<Path>>(mesh: &Vec<IconVertex>, path: P) {
    if let Ok(data) = rmp_serde::to_vec(mesh) {
        println!("writing {:?}", data);
        let _ = std::fs::write(path, data);
    }
}
