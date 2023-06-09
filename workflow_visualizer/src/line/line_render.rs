use bevy_ecs::component::Component;
use wgpu::util::DeviceExt;

use crate::{DeviceContext, GfxSurface, Position};

#[derive(Component)]
pub struct LineRenderPoints {
    pub(crate) points: Vec<Position<DeviceContext>>,
}

#[derive(Component)]
pub struct LineRender {
    pub(crate) head: usize,
    pub(crate) tail: usize,
    pub(crate) capacity: usize,
}

impl LineRender {
    pub fn head_value(
        &self,
        cpu: &Vec<Position<DeviceContext>>,
    ) -> Option<Position<DeviceContext>> {
        cpu.get(self.head).copied()
    }
    pub fn tail_value(
        &self,
        cpu: &Vec<Position<DeviceContext>>,
    ) -> Option<Position<DeviceContext>> {
        cpu.get(self.tail).copied()
    }
    pub fn head(&self) -> usize {
        self.head
    }
    pub fn tail(&self) -> usize {
        self.tail
    }
    pub fn set_head_value(
        &mut self,
        cpu: &mut Vec<Position<DeviceContext>>,
        pos: Position<DeviceContext>,
    ) {
        if let Some(position) = cpu.get_mut(self.head) {
            *position = pos;
        }
        // TODO need to trigger GPU write somehow maybe moving this out to a system with component hook
    }
    pub fn set_tail_value(
        &mut self,
        cpu: &mut Vec<Position<DeviceContext>>,
        pos: Position<DeviceContext>,
    ) {
        if let Some(position) = cpu.get_mut(self.tail) {
            *position = pos;
        }
        // TODO need to trigger GPU write somehow maybe moving this out to a system with component hook
    }
    pub fn adjust_head(&mut self, amount: usize, towards_tail: bool) {
        if towards_tail {
            self.head = match self.head.checked_add(amount) {
                None => self.tail,
                Some(am) => am,
            }
        } else {
            self.head = match self.head.checked_sub(amount) {
                None => 0,
                Some(am) => am,
            }
        }
        if self.head > self.tail {
            self.head = self.tail;
        }
    }
    pub fn adjust_tail(&mut self, amount: usize, towards_head: bool) {
        if towards_head {
            self.tail = match self.tail.checked_sub(amount) {
                None => self.head,
                Some(am) => am,
            }
        } else {
            self.tail = match self.tail.checked_add(amount) {
                None => self.capacity,
                Some(am) => am,
            }
        }
        if self.tail < self.head {
            self.tail = self.head;
        }
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            head: 0,
            tail: capacity,
            capacity,
        }
    }
}
