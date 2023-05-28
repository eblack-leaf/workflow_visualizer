use dioxus::prelude::*;
use dioxus_free_icons::Icon;

fn main() {
    #[cfg(target_family = "wasm")]
    dioxus_web::launch(Root);
}
fn Root(cx: Scope) -> Element {
    let rsx = rsx!(

    );
    cx.render(rsx)
}
