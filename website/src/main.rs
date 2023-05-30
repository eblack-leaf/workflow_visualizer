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
                        Link { class: "text-lg sm:text-2xl font-bold", to: "/", "W-V"}
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
                class:"text-sm text-neutral-300 bg-red-800 py-4 ml-8 sm:ml-32 md:ml-48 text-center mr-4 md:mr-16",
                "Impl "
                Link {
                    class:"font-bold text-md text-red-300",
                    to:"https://eblack-leaf.github.io/workflow_visualizer/doc/workflow_visualizer/trait.Workflow.html",
                    external:true,
                    "Workflow"
                }
                " to setup application."
            }
            div {
                class:"bg-green-800 text-neutral-300 py-4 max-w-[85%] text-center",
                h1 { class:"text-4xl sm:text-7xl pl-4 font-bold inline", "WORKFLOW"}
            }
            div {
                class:"text-neutral-300 bg-sky-700 py-2 ml-16 sm:ml-16 md:ml-32 text-center",
                h1 { class:"text-2xl sm:text-6xl pr-4 font-semibold inline", "VISUALIZER"}
            }
            div {
                class:"text-sm text-neutral-300 bg-amber-700 py-4 md:ml-8 max-w-[85%] text-center",
                "Utilize "
                Link {
                    class:"font-bold text-md text-amber-400",
                    to:"https://eblack-leaf.github.io/workflow_visualizer/doc/workflow_visualizer/struct.Visualizer.html",
                    external:true,
                    "Visualizer"
                }
                " to define aesthetics & reactions."
            }
            div {
                class:"text-neutral-800 bg-fuchsia-800 max-w-[75%] ml-4 text-center",
                p { class:"px-4 text-sm text-neutral-300",
                    p { class:"font-semibold",
                        Icon { class:"h-10 w-10 inline pr-4 text-fuchsia-400", icon:dioxus_free_icons::icons::fi_icons::FiCodeSlash}
                        "Web/Native" span {class:"text-fuchsia-400", " UI "} "Toolkit."
                    }
                }
            }
            div {
                class:"px-4 text-sm text-neutral-300 bg-indigo-800 py-4 ml-7 sm:ml-16 md:ml-64 text-center mr-4 md:mr-16",
                "Impl "
                Link {
                    class:"font-bold text-md text-indigo-400",
                    to:"https://eblack-leaf.github.io/workflow_visualizer/doc/workflow_visualizer/trait.Render.html",
                    external:true,
                    "Render"
                }
                " + "
                Link {
                    class:"font-bold text-md text-indigo-400",
                    to:"https://eblack-leaf.github.io/workflow_visualizer/doc/workflow_visualizer/trait.Attach.html",
                    external:true,
                    "Attach"
                }
                " to extend rendering."
            }
            div {
                class:"px-4 text-sm text-neutral-300 bg-lime-800 py-4 ml-2 md:ml-15 max-w-[75%] text-center",
                Link {
                    class:"font-bold text-md text-lime-500",
                    to:"https://github.com/bevyengine/bevy/tree/main/crates/bevy_ecs",
                    external:true,
                    "Bevy-ECS"
                }
                " for composition over inheritance."
            }
            div {
                class:"mt-16 mx-4",
                p {class:"text-md text-neutral-500", "Overview"}
                div {
                    class:"py-4",
                    div {
                        class:"text-xs",
                        "Workflow-Visualizer is a rust library for creating responsive web/native UI applications.
                        See `ARCHITECTURE` for more.
                        "
                        div {
                            class:"",
                            // Icon { class:"h-6 w-6", icon:dioxus_free_icons::icons::fi_icons::FiCpu}
                            p {class:"", ""}
                        }
                    }
                    div {
                        class:"",
                    }
                }
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
