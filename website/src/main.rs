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
                        Link { class: "text-lg sm:text-2xl font-bold", to: "/", "Wkflw-Vslzr"}
                        Link { class: "pl-4 sm:pl-8 text-sm text-neutral-400 underline underline-offset-2 decoration-red-800",
                        to: "/architecture", "ARCHITECTURE" }
                        Link { class: "pl-4 sm:pl-8 text-sm text-neutral-400 underline underline-offset-2 decoration-orange-700",
                        to: "doc/workflow_visualizer/index.html", external: true, "API"}
                        Link { class:"h-12 w-12 text-neutral-400 inline ml-4 sm:ml-8",
                        to:"https://github.com/eblack-leaf/workflow_visualizer", external: true,
                            Icon {
                                class:"inline",
                                icon: dioxus_free_icons::icons::fi_icons::FiGithub,
                            }
                        }
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
                p { class:"px-4 mt-4 text-md italic text-neutral-700",
                    "Rust lib for blazingly fast + stylish user interfaces. Render natively on "
                    span { class:"", "desktop/mobile" }
                    " and via " Link {class:"text-md font-bold text-neutral-900", to:"https://webassembly.org/", external:true, "wasm"}
                    " + browser on web. Powered by "
                    Link {
                        class:"font-bold text-md text-neutral-900",
                        to:"https://wgpu.rs/", external: true,
                        "wgpu.rs"
                    }
                }
            }
            div {
                class:"px-4 text-sm text-neutral-300 bg-green-800 py-4 ml-24 sm:ml-32 md:ml-48 text-center mr-4 md:mr-16",
                "Application thread is run async from UI thread to keep flat 16.7ms frame time. Impl "
                Link {
                    class:"font-bold text-md text-neutral-900",
                    to:"https://eblack-leaf.github.io/workflow_visualizer/doc/workflow_visualizer/trait.Workflow.html",
                    external:true,
                    "Workflow"
                }
                " to setup message passing infrastructure to communicate between the threads."
            }
            div {
                class:"px-4 text-sm text-neutral-300 bg-blue-800 py-4 ml-20 md:ml-36 max-w-[55%] text-center",
            }
            div {
                class:"px-4 text-sm text-neutral-300 bg-indigo-800 py-4 ml-32 sm:ml-48 md:ml-64 text-center mr-16 md:mr-32",
                "Single render pass for efficient mobile rendering. "
                "Extensible with custom render pipelines for specific solutions. Impl "
                Link {
                    class:"font-bold text-md text-neutral-900",
                    to:"https://eblack-leaf.github.io/workflow_visualizer/doc/workflow_visualizer/trait.Render.html",
                    external:true,
                    "Render"
                }
                " + "
                Link {
                    class:"font-bold text-md text-neutral-900",
                    to:"https://eblack-leaf.github.io/workflow_visualizer/doc/workflow_visualizer/trait.Attach.html",
                    external:true,
                    "Attach"
                }
                " to get started."
            }
            div {
                class:"px-4 text-sm text-neutral-300 bg-violet-800 py-4 ml-24 md:ml-40 max-w-[45%] text-center",
                "ECS pattern via "
                Link {
                    class:"font-bold text-md text-neutral-900",
                    to:"https://github.com/bevyengine/bevy/tree/main/crates/bevy_ecs",
                    external:true,
                    "Bevy-ECS"
                }
                " for composition over inheritance + blazingly fast iteration minimizing cache misses."
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
