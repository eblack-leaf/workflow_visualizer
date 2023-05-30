use dioxus::html::h1;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;

fn main() {
    #[cfg(target_family = "wasm")]
    dioxus_web::launch(root);
}
fn root(cx: Scope) -> Element {
    let rsx = rsx!(
        div {
            class: " text-neutral-300 bg-neutral-800",
            div {
            class: "px-4 py-8",
                    a { class: "text-lg sm:text-2xl font-bold", href:"/", "W-V"}
                    a { class: "pl-4 sm:pl-8 text-sm text-neutral-400 underline underline-offset-2 decoration-red-800",
                    href: "book/index.html", "ARCHITECTURE" }
                    a { class: "pl-4 sm:pl-8 text-sm text-neutral-400 underline underline-offset-2 decoration-orange-700",
                    href: "doc/workflow_visualizer/index.html", "API"}
                    a { class:"h-12 w-12 text-neutral-400 inline ml-4 sm:ml-8",
                    href:"https://github.com/eblack-leaf/workflow_visualizer",
                        Icon {
                            class:"inline",
                            icon: dioxus_free_icons::icons::fi_icons::FiGithub,
                        }
                    }
            }
            div {
                class:"py-4",
                div {
                    class:"bg-green-800 text-neutral-300 py-4 max-w-[85%] text-center",
                    h1 { class:"text-4xl sm:text-7xl pl-4 font-bold", "WORKFLOW"}
                }
                div {
                    class:"text-neutral-300 bg-sky-800 py-2 ml-16 md:ml-32 text-center",
                    h1 { class:"text-2xl sm:text-6xl pr-4 font-semibold inline", "VISUALIZER"}
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
                    class:"text-sm text-neutral-300 bg-orange-800 py-4 ml-8 sm:ml-32 md:ml-48 text-center mr-4 md:mr-16",
                    "Impl "
                    a {
                        class:"font-bold text-md text-orange-300",
                        href:"https://eblack-leaf.github.io/workflow_visualizer/doc/workflow_visualizer/trait.Workflow.html",
                        "Workflow"
                    }
                    " to setup application."
                }
                div {
                    class:"text-sm text-neutral-300 bg-amber-700 py-4 md:ml-8 max-w-[85%] text-center",
                    "Utilize "
                    a {
                        class:"font-bold text-md text-amber-400",
                        href:"doc/workflow_visualizer/struct.Visualizer.html",
                        "Visualizer"
                    }
                    " to define aesthetics & reactions."
                }
                div {
                    class:"px-4 text-sm text-neutral-300 bg-indigo-800 py-4 ml-7 sm:ml-16 md:ml-64 text-center mr-4 md:mr-16",
                    "Impl "
                    a {
                        class:"font-bold text-md text-indigo-400",
                        href:"doc/workflow_visualizer/trait.Render.html",
                        "Render"
                    }
                    " + "
                    a {
                        class:"font-bold text-md text-indigo-400",
                        href:"doc/workflow_visualizer/trait.Attach.html",
                        "Attach"
                    }
                    " to extend rendering."
                }
                div {
                    class:"px-4 text-sm text-neutral-300 bg-lime-800 py-4 ml-2 md:ml-15 max-w-[75%] text-center",
                    a {
                        class:"font-bold text-md text-lime-500",
                        href:"https://github.com/bevyengine/bevy/tree/main/crates/bevy_ecs",
                        "Bevy-ECS"
                    }
                    " for composition over inheritance."
                }
                div {
                    class:"mt-16 mx-4 font-mono md:h-96",
                    p {class:"text-md text-neutral-500 text-center", "Overview"}
                    div {
                        class:"",
                        div {
                            class:"text-md text-center",
                            p{
                                class:"py-4",
                                "Workflow-Visualizer is a "
                                a {
                                    class:"text-neutral-500",
                                    href:"https://www.rust-lang.org/",
                                    "rust"
                                }
                                " library for creating responsive web/native UI applications."
                            }
                            p {
                                class:"py-4", " Powered by "
                                a {class:"text-neutral-500", href:"https://wgpu.rs/", "wgpu.rs"}
                                " for performant cross-platform gpu acceleration."
                            }
                            p{
                                class:"", "Desktop: " span {class:"text-xs text-neutral-400 italic", "Linux | Windows | Mac"}
                            }
                            p {class:"", "Mobile: " span{class:"text-xs text-neutral-400 italic", "Android | (iOS coming soon...)"}}
                            p {class:"", "Web: "
                                a {class:"text-neutral-500 italic", href:"https://webassembly.org/", "wasm"}
                            }
                            p{class:"py-4", "See " a {class:"text-neutral-500", href:"book/index.html", "ARCHITECTURE" } " for more."}
                        }
                    }
                }
            }
        }
    );
    cx.render(rsx)
}
