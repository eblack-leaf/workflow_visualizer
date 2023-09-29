use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

pub struct Media {}
impl Media {
    pub const ELEMENT_ID: &'static str = "media-overlay";
    pub const BUTTON_HANDLE: &'static str = "media-overlay-trigger";
    pub fn video(src: &str, ty: &str) {
        let document = web_sys::window().unwrap().document().unwrap();
        let node = document.create_element("div").unwrap();
        node.set_id(Self::ELEMENT_ID);
        let html = format!(
            "\
        <div style=\"display:flex; justify-content:center; width: 100%;height: 100%; padding:5px;
        background: black; position: absolute; top: 0;left: 0\">
            <video style=\"height:95%;width:auto\" controls>
                <source src={} type={}>
            </video>
        </div>
        <button id={} style=\"
                    position:absolute;
                    top:0;
                    left:0;
                    width:40px;
                    height:40px;
                    border:none;
                    color:white;
                    background:black;
                    text-align:center;
                    text-decoration:none;
                    font-size:32px;\">&times
        </button>",
            src,
            ty,
            Self::BUTTON_HANDLE
        );
        node.set_inner_html(html.as_str());
        let body = document.body().unwrap();
        body.append_child(&node).unwrap();
        let callback = Closure::wrap(Box::new(move |_e: web_sys::Event| {
            Self::remove();
        }) as Box<dyn FnMut(_)>);
        document
            .get_element_by_id(Self::BUTTON_HANDLE)
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap()
            .set_onclick(Some(callback.as_ref().unchecked_ref()));
        callback.forget();
    }
    pub fn remove() {
        let document = web_sys::window().unwrap().document().unwrap();
        document
            .get_element_by_id(Self::ELEMENT_ID)
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap()
            .remove();
    }
}
