use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn WalletMultiButton(cx: Scope) -> Element {
    // TODO
    let copied = use_state(&cx, || false);
    let active = use_state(&cx, || false);
    let aria_expanded = if **active { "true" } else { "false" };
    let button_style = format!("pointerEvents: {}", if **active { "none" } else { "auto" },);
    cx.render(rsx! {
        div {
            class: "wallet-adapter-dropdown",
            button {
                aria_expanded: "{aria_expanded}",
                class: "wallet-adapter-button-trigger",
                style: "{button_style}",
                onclick: move |_| { *active.make_mut() = true },
                // start_icon: {<WalletIcon wallet={wallet} />}
                // {...props}
                // {content}
            }
            ul {
                aria_label: "dropdown-list",
                // class: {`wallet-adapter-dropdown-list ${active && 'wallet-adapter-dropdown-list-active'}`},
                // ref: {ref},
                role: "menu",
                li {
                    onclick: |_| { /* TODO: copyAddress */ },
                    class: "wallet-adapter-dropdown-list-item",
                    role: "menuitem",
                    copied.then(|| "Copied").or(Some("Copy address"))
                }
                li {
                    onclick: |_| { /* TODO: openModal */ },
                    class: "wallet-adapter-dropdown-list-item",
                    role: "menuitem",
                    "Change wallet"
                }
                li {
                    onclick: |_| { /* TODO: disconnect */ },
                    class: "wallet-adapter-dropdown-list-item",
                    role: "menuitem",
                    "Disconnect"
                }
            }
        }
    })
}
