use crate::contexts::NetworkConfiguration;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn NetworkSwitcher(cx: Scope) -> Element {
    let network_configuration = use_shared_state::<NetworkConfiguration>(cx).unwrap();
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
