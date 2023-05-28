use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_router::{Link, Route, Router};

fn main() {
    #[cfg(target_family = "wasm")]
    dioxus_web::launch(root);
}
fn root(cx: Scope) -> Element {
    let rsx = rsx!(
        Router {
            Route { to: "/", landing(cx) }
            Route { to: "/architecture", architecture(cx)}
        }
    );
    cx.render(rsx)
}
fn landing(cx: Scope) -> Element {
    let rsx = rsx!(
        div {
            class: "font-mono text-neutral-300 bg-neutral-800",
            div {}
            Link { to: "doc/workflow_visualizer/index.html", external: true, "API Reference"}
        }

    );
    cx.render(rsx)
}
fn architecture(cx: Scope) -> Element {
    let rsx = rsx!(
        p { "hello world... architecture"}
    );
    cx.render(rsx)
}
