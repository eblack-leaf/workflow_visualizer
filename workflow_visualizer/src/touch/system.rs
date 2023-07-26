use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::entity::Entity;
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::Query;
use tracing::trace;

use crate::{Area, CurrentlyPressed, InterfaceContext, Layer, Position, PrimaryTouch, ScaleFactor, Section, ToggleState, TouchListener, TouchLocation, TouchTrigger, ViewportHandle};
use crate::focus::FocusedEntity;
use crate::touch::adapter::{TouchGrabState, TrackedTouch};
use crate::touch::component::{ListenableTouchType, TouchEvent, TouchType};

pub(crate) fn read_touch_events(
    mut event_reader: EventReader<TouchEvent>,
    mut primary_touch: ResMut<PrimaryTouch>,
    mut touch_listeners: Query<(
        Entity,
        &Position<InterfaceContext>,
        &Area<InterfaceContext>,
        &Layer,
        &TouchListener,
        &mut TouchTrigger,
        &mut CurrentlyPressed,
        &mut ToggleState,
        &mut TouchLocation,
    )>,
    scale_factor: Res<ScaleFactor>,
    viewport_handle: Res<ViewportHandle>,
    mut touch_grab_state: ResMut<TouchGrabState>,
    mut focused_entity: ResMut<FocusedEntity>,
) {
    let new_touches = event_reader.iter().cloned().collect::<Vec<TouchEvent>>();
    let mut cancelled_events = new_touches.clone();
    cancelled_events.retain(|c| c.ty == TouchType::Cancelled);
    let is_cancelled = !cancelled_events.is_empty();
    let mut trigger_on_press = false;
    let mut trigger_on_release = false;
    if !is_cancelled {
        for touch in new_touches.iter() {
            match touch.ty {
                TouchType::OnPress => {
                    primary_touch.touch.replace(TrackedTouch::new(touch.touch));
                    trigger_on_press = true;
                }
                TouchType::OnMove => {
                    if let Some(prime) = primary_touch.touch.as_mut() {
                        trace!("setting current touch");
                        prime.current = touch.touch;
                    }
                    if let Some(entity) = focused_entity.entity {
                        if let Ok((_, _, _, _, _, _, _, _, mut touch_location)) =
                            touch_listeners.get_mut(entity)
                        {
                            if let Some(mut tracked) = touch_location.0 {
                                trace!("setting current touch");
                                tracked.current = touch.touch;
                            }
                        }
                    }
                }
                TouchType::OnRelease => {
                    if let Some(prime) = primary_touch.touch.as_mut() {
                        prime.end.replace(touch.touch);
                    }
                    if let Some(entity) = focused_entity.entity {
                        if let Ok((_, _, _, _, _, _, mut pressed, _, _)) =
                            touch_listeners.get_mut(entity)
                        {
                            pressed.currently_pressed = false;
                        }
                    }
                    trigger_on_release = true;
                }
                _ => {}
            }
        }
    }
    if !new_touches.is_empty() && !is_cancelled {
        for (entity, pos, area, layer, listener, _, _, _, _) in touch_listeners.iter() {
            let section = Section::from((*pos, *area));
            if !listener.disabled {
                match listener.listened_type {
                    ListenableTouchType::OnPress => {
                        if trigger_on_press {
                            if let Some(prime) = primary_touch.touch {
                                let touch_origin = prime.origin.to_interface(scale_factor.factor())
                                    - viewport_handle.section.position;
                                if section.contains(touch_origin) {
                                    if let Some(grab_state) = touch_grab_state.grab_state.as_mut() {
                                        if *layer > grab_state.1 {
                                            grab_state.0 = entity;
                                            grab_state.1 = *layer;
                                        }
                                    } else {
                                        touch_grab_state.grab_state.replace((entity, *layer));
                                    }
                                }
                            }
                        }
                    }
                    ListenableTouchType::OnRelease => {
                        if trigger_on_release {
                            if let Some(prime) = primary_touch.touch {
                                if let Some(end) = prime.end {
                                    let touch_origin = prime.origin.to_interface(scale_factor.factor())
                                        - viewport_handle.section.position;
                                    let touch_end = end.to_interface(scale_factor.factor())
                                        - viewport_handle.section.position;
                                    if section.contains(touch_origin) && section.contains(touch_end) {
                                        if let Some(grab_state) = touch_grab_state.grab_state.as_mut() {
                                            if *layer > grab_state.1 {
                                                grab_state.0 = entity;
                                                grab_state.1 = *layer;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if let Some(grabbed) = touch_grab_state.grab_state.take() {
            if let Ok((
                          _,
                          _,
                          _,
                          _,
                          listener,
                          mut touch_trigger,
                          mut currently_pressed,
                          mut toggle_state,
                          mut touch_location,
                      )) = touch_listeners.get_mut(grabbed.0)
            {
                if trigger_on_press {
                    currently_pressed.currently_pressed = true;
                    if let ListenableTouchType::OnPress = listener.listened_type {
                        touch_trigger.touched = true;
                        toggle_state.toggle = !toggle_state.toggle;
                        touch_location.0.replace(primary_touch.touch.unwrap());
                        focused_entity.entity.replace(grabbed.0);
                    }
                }
                if trigger_on_release {
                    currently_pressed.currently_pressed = false;
                    if let ListenableTouchType::OnRelease = listener.listened_type {
                        touch_trigger.touched = true;
                        toggle_state.toggle = !toggle_state.toggle;
                        touch_location.0.replace(primary_touch.touch.unwrap());
                        focused_entity.entity.replace(grabbed.0);
                    }
                }
            }
        } else if trigger_on_press && focused_entity.entity.is_some() {
            let _ = focused_entity.entity.take();
        }
    }
}

pub(crate) fn reset_touched(mut touch_listeners: Query<&mut TouchTrigger>) {
    for mut touched in touch_listeners.iter_mut() {
        touched.touched = false;
    }
}
