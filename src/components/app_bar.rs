use crate::wallets;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn AppBar(cx: Scope) -> Element {
    // TODO: use_local_storage instread of use_state?
    let auto_connect = use_state(&cx, || false);
    cx.render(rsx! {
        div {
            class: "navbar flex flex-row md:mb-2 shadow-lg bg-neutral text-neutral-content",
            // TODO:
            // TODO:
            // Wallet & Settings
            div {
                class: "navbar-end",
                wallets::ui::MultiButton {
                    // TODO
                    // class: "btn btn-ghost mr-4",
                }
                div {
                    class: "dropdown dropdown-end",
                    div {
                        tabindex: "0",
                        class: "btn btn-square btn-ghost text-right",
                    }
                    svg {
                        class: "w-6 h-6",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z",
                        }
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z",
                        }
                    }
                }
                ul {
                    tabindex: "0",
                    class: "p-2 shadow menu dropdown-content bg-base-100 rounded-box sm:w-52",
                    li {
                        div {
                            class: "form-control",
                            label {
                                class: "cursor-pointer label",
                                a { "AutoConnect" }
                                input {
                                    "type": "checkbox",
                                    checked: "{auto_connect}",
                                    onchange: move |_| { *auto_connect.make_mut() = false },
                                    class: "toggle",
                                }
                            }
                            // TODO
                            // NetworkSwitcher {}
                        }
                    }
                }
            }
        }
    })
}
