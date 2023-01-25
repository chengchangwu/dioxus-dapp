use dioxus::prelude::*;

use crate::wallets::{hooks::use_wallet, window};

#[derive(Props)]
pub struct WalletMultiButtonProps<'a> {
    #[props(default)]
    pub class: &'a str,
    #[props(default)]
    pub disabled: bool,
    #[props(optional)]
    pub end_icon: Option<Element<'a>>,
    #[props(optional)]
    pub start_icon: Option<Element<'a>>,
    // style?: CSSProperties;
    // tabIndex?: number;
    pub children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn WalletMultiButton<'a>(cx: Scope<'a, WalletMultiButtonProps<'a>>) -> Element<'a> {
    // TODO
    let (public_key, wallet, disconnect) = use_wallet();
    let copied = use_state(&cx, || false);
    let active = use_state(&cx, || false);
    let wallet_adapter_dropdown_list_active = if *active.current() {
        "wallet-adapter-dropdown-list-active"
    } else {
        ""
    };
    let button_style = format!("pointerEvents: {}", if **active { "none" } else { "auto" },);
    let detected = use_state(&cx, || {
        window()
            .expect("No window object")
            .solana()
            .and_then(|s| s.is_brave_wallet().then_some(true))
            .is_some()
    });
    log::debug!("detected wallet? {detected}, active? {active}");
    if wallet.is_none() {
        return cx.render(rsx! {
            // WalletModalButton { ..cx.props.clone() }
            button {
                class: "wallet-adapter-button {cx.props.class}",
                disabled: cx.props.disabled,
                // style={props.style}
                // tabIndex={props.tabIndex || 0}
                r#type: "button",
                // start_icon,
                &cx.props.children,
                // end_icon
            }
        });
    };
    // TODO
    // if (!base58) return <WalletConnectButton {...props}>{children}</WalletConnectButton>;
    cx.render(rsx! {
        div {
            class: "wallet-adapter-dropdown",
            button {
                // aria_expanded: active,
                class: "wallet-adapter-button-trigger",
                // style: "{button_style}",
                onclick: move |_| { active.with_mut(|v| *v = true) },
                i {
                    class: "wallet-adapter-button-start-icon",
                    // <WalletIcon wallet={wallet} />
                }
                // {content}
            }
            ul {
                // aria_label: "dropdown-list",
                class: "wallet-adapter-dropdown-list {wallet_adapter_dropdown_list_active}",
                // ref: {ref},
        //         role: "menu",
                li {
                    onclick: |_| { /* TODO: copyAddress */ },
                    class: "wallet-adapter-dropdown-list-item",
                    // role: "menuitem",
                    copied.then(|| "Copied").or(Some("Copy address"))
                }
                li {
                    onclick: |_| { /* TODO: openModal */ },
                    class: "wallet-adapter-dropdown-list-item",
                    // role: "menuitem",
                    "Change wallet"
                }
                li {
                    onclick: |_| { /* TODO: disconnect */ },
                    class: "wallet-adapter-dropdown-list-item",
                    // role: "menuitem",
                    "Disconnect"
                }
            }
        }
    })
}
