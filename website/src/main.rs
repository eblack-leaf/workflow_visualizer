use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_router::{Link, Redirect, Route, Router};

fn main() {
    #[cfg(target_family = "wasm")]
    dioxus_web::launch(root);
}
fn root(cx: Scope) -> Element {
    let rsx = rsx!(
        Router {
            div {
                class: "font-mono text-neutral-300 bg-neutral-800",
                div {
                class: "px-4 py-8",
                        Link { class: "text-lg sm:text-2xl font-bold", to: "/", "Workflow-Visualizer"}
                        Link { class: "pl-4 sm:pl-8 text-sm text-neutral-400 underline underline-offset-2 decoration-red-800", to: "/architecture", "ARCHITECTURE" }
                        Link { class: "pl-4 sm:pl-8 text-sm text-neutral-400 underline underline-offset-2 decoration-orange-700", to: "doc/workflow_visualizer/index.html", external: true, "API"}
                }
                Route { to: "/", landing(cx) }
                Route { to: "/architecture", architecture(cx)}
                // Redirect { from: "", to: "/" }
            }
        }
    );
    cx.render(rsx)
}
fn landing(cx: Scope) -> Element {
    let rsx = rsx!(
        div {
            class:"py-4",
            div {
                class:"bg-red-800 text-neutral-300 py-4 max-w-[85%] md:text-center",
                h1 { class:"text-2xl sm:text-7xl pl-8 font-bold", "WORKFLOW"}
                p { class:"pl-20 mt-4 text-sm italic text-neutral-300", "structure for describing an application."}
            }
            div {
                class:"text-neutral-300 bg-orange-800 py-4 ml-32 text-right md:text-center",
                h1 { class:"text-lg sm:text-6xl pr-4 font-semibold", "VISUALIZER"}
                p { class:"px-16 mt-4 text-sm italic text-neutral-300", "web/native rendering tools."}
            }
            div {
                class:"text-neutral-800 bg-yellow-600 py-4 max-w-[75%] ml-10",
                h1 { class:"text-md sm:text-xl pl-4", "OVERVIEW"}
                p { class:"px-16 mt-4 text-sm italic text-neutral-700", "something."}
            }
        }
    );
    cx.render(rsx)
}
fn architecture(cx: Scope) -> Element {
    let rsx = rsx!(
        div {
            class:"",
            "Architecture"
        }
    );
    cx.render(rsx)
}
