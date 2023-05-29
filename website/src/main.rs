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
                        Link { class: "text-2xl font-bold", to: "/", "Workflow-Visualizer"}
                        Link { class: "pl-8 text-sm text-neutral-400", to: "/architecture", "ARCHITECTURE" }
                        Link { class: "pl-8 text-sm text-neutral-400", to: "doc/workflow_visualizer/index.html", external: true, "DOC"}
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
            class:"grid sm:grid-cols-2 gap-4 place-content-center",
            div {
                class:"px-4 py-8 text-6xl italic",
                p { class: "text-orange-500", "Workflow-Visualizer"}
                div {
                    class: "pl-8 pt-4 text-sm italic",
                    "Web/Native UI toolkit ..."
                }
            }
            div {
                class:"pt-16",
                div {
                    dioxus_free_icons::Icon {
                        class: "h-6 w-6 text-neutral-500 inline",
                        icon: dioxus_free_icons::icons::hi_solid_icons::HiAdjustments,
                    }
                    p {class:"text-lg inline pl-4", "WEB | "}
                    p {class:"text-lg inline text-orange-500", "NATIVE"}
                }
                p { class:"pt-4", "Visualizer tools compile to Wasm (WebAssembly) to run in the browser,\
                or take full advantage of the hardware running bare metal with Vulkan/DirectX12/Metal via WGPU."}
                br {}
                div {
                    dioxus_free_icons::Icon {
                        class: "h-6 w-6 text-neutral-500 inline",
                        icon: dioxus_free_icons::icons::hi_solid_icons::HiDatabase,
                    }
                    p {class:"text-lg inline pl-4", "LOCAL | "}
                    p {class:"text-lg inline text-orange-500", "REMOTE"}
                }
                p { class:"pt-4", "Visualizer tools compile to Wasm (WebAssembly) to run in the browser,\
                or take full advantage of the hardware running bare metal with Vulkan/DirectX12/Metal via WGPU."}
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
