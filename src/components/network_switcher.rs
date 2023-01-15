use crate::wallets::hooks::use_local_storage;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn NetworkSwitcher(cx: Scope) -> Element {
    let network_configuration = use_local_storage::<String>(cx, "network", "devnet".to_string());
    log::debug!("network configuration {}", network_configuration.current());
    cx.render(rsx! {
        label {
            class: "cursor-pointer label",
            a {
                "Network"
            }
            select {
                class: "select max-w-xs",
                value: "{network_configuration.current()}",
                onchange: move |evt| { network_configuration.with_mut(|x| *x = evt.value.clone()) },
                option {
                    value: "mainnet-beta",
                    "main"
                }
                option {
                    value: "devnet",
                    "dev"
                }
                option {
                    value: "testnet",
                    "test"
                }
            }
        }
    })
}
