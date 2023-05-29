use dioxus::html::h1;
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
                h1 { class:"text-4xl sm:text-7xl pl-4 font-bold", "WORKFLOW"}
                p { class:"pl-10 sm:pl-20 mt-4 text-xs italic text-neutral-300", "structure for describing an application."}
            }
            div {
                class:"text-neutral-300 bg-orange-700 py-4 ml-16 md:ml-32 text-right md:text-center",
                h1 { class:"text-2xl sm:text-6xl pr-4 font-semibold", "VISUALIZER"}
                p { class:"mt-4 text-xs italic text-neutral-300 pr-2", "web/native rendering tools."}
            }
            div {
                class:"text-neutral-800 bg-yellow-600 py-4 max-w-[75%] ml-10",
                h1 { class:"text-lg sm:text-xl pl-4", "OVERVIEW"}
                p { class:"px-16 mt-4 text-sm italic text-neutral-700", "something."}
            }
            div {
                class:"text-neutral-300 bg-green-700 py-4 ml-24 sm:ml-32 md:ml-48 text-center mr-4 md:mr-16 h-16",
            }
            div {
                class:"text-neutral-300 bg-blue-700 py-4 ml-20 md:ml-36 max-w-[55%] h-12",
            }
            div {
                class:"text-neutral-300 bg-indigo-700 py-4 ml-32 sm:ml-48 md:ml-64 text-center mr-16 md:mr-32 h-8",
            }
            div {
                class:"text-neutral-300 bg-violet-700 py-4 ml-24 md:ml-40 max-w-[45%] h-4",
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
