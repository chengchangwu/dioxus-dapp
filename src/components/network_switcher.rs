use crate::{contexts::NetworkConfiguration, hooks::use_local_storage};
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn NetworkSwitcher(cx: Scope) -> Element {
    let network_configuration = use_local_storage::<NetworkConfiguration>(cx).unwrap();
    log::debug!("network configuration {}", network_configuration.read().0);
    cx.render(rsx! {
        label {
            class: "cursor-pointer label",
            a {
                "Network"
            }
            select {
                class: "select max-w-xs",
                value: "{network_configuration.read().0}",
                onchange: move |evt| { network_configuration.write().0 = evt.value.clone(); },
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
