use super::Link;
use dioxus::prelude::*;

#[derive(Props)]
pub struct ContentContainerProps<'a> {
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn ContentContainer<'a>(cx: Scope<'a, ContentContainerProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "flex-1 drawer h-52",
            input {
                // id: "my-drawer",
                class: "grow drawer-toggle",
                r#type: "checkbox",
            }
            div {
                class: "items-center  drawer-content",
                &cx.props.children
            }
            div {
                class: "drawer-side",
                label {
                    // html_for: "my-drawer",
                    class: "drawer-overlay",
                }
                ul {
                    class: "p-4 overflow-y-auto menu w-80 bg-base-100",
                    li {
                        h1 {
                            "Menu"
                        }
                    }
                    li {
                        Link {
                            href: "/",
                        }
                        a {
                            "Home"
                        }
                    }
                    li {
                        Link {
                            href: "/basics",
                        }
                        a {
                            "Basics"
                        }
                    }
                }
            }
        }
    })
}
