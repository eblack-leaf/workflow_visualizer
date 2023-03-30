use bevy_ecs::prelude::{Component, Resource};

use crate::{Attach, Engen};

#[derive(Resource)]
pub struct VirtualKeyboardAdapter {}

#[derive(Component, Copy, Clone)]
pub enum VirtualKeyboardType {
    Keyboard,
    TelephonePad,
    NumberPad,
}

impl VirtualKeyboardAdapter {
    pub(crate) fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            let document = web_sys::window().unwrap().document().unwrap();
            let node = document.create_element("div").unwrap();
            node.set_inner_html(
                "<input type='text' maxlength='0' width=0 height=0 \
            id='keyboard_trigger' style='position: absolute;left: -1px;top: -1px;opacity: 0;\
            padding: 0;min-width: 0; min-height: 0;width: 0; height: 0;border: 0'>\
            <input type='tel' maxlength='0' width=0 height=0 \
            id='telephone_pad_trigger' style='position: absolute;left: -1px;top: -1px;opacity: 0;\
            padding: 0;min-width: 0; min-height: 0;width: 0; height: 0;border: 0'>\
            <input type='number' maxlength='0' width=0 height=0 \
            id='numpad_trigger' style='position: absolute;left: -1px;top: -1px;opacity: 0;\
            padding: 0;min-width: 0; min-height: 0;width: 0; height: 0;border: 0'>",
            );
            let body = document.body().unwrap();
            body.append_child(&node).unwrap();
        }
        Self {}
    }
    #[allow(unused)]
    pub fn open(&self, ty: VirtualKeyboardType) {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::{JsCast, prelude::*};
            let document = web_sys::window().unwrap().document().unwrap();
            let trigger_element = match ty {
                VirtualKeyboardType::Keyboard => document
                    .get_element_by_id("keyboard_trigger")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlElement>()
                    .unwrap(),
                VirtualKeyboardType::TelephonePad => document
                    .get_element_by_id("telephone_pad_trigger")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlElement>()
                    .unwrap(),
                VirtualKeyboardType::NumberPad => document
                    .get_element_by_id("numpad_trigger")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlElement>()
                    .unwrap(),
            };
            trigger_element.blur().unwrap();
            trigger_element.focus().unwrap();
            web_sys::console::info_1(&JsValue::from_str("opening vkey"));
        }
    }
    pub fn close(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::{JsCast, JsValue};
            let document = web_sys::window().unwrap().document().unwrap();
            document
                .get_element_by_id("keyboard_trigger")
                .unwrap()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap()
                .blur()
                .unwrap();
            document
                .get_element_by_id("telephone_pad_trigger")
                .unwrap()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap()
                .blur()
                .unwrap();
            document
                .get_element_by_id("numpad_trigger")
                .unwrap()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap()
                .blur()
                .unwrap();
            web_sys::console::info_1(&JsValue::from_str("closing vkey"));
        }
    }
}
pub struct VirtualKeyboardAttachment;
impl Attach for VirtualKeyboardAttachment {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .container
            .insert_resource(VirtualKeyboardAdapter::new());
    }
}
