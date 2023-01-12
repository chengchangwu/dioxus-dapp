use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn NetworkSwitcher(cx: Scope) -> Element {
    // TODO
    // const { networkConfiguration, setNetworkConfiguration } = useNetworkConfiguration();
    // console.log(networkConfiguration);
    cx.render(rsx! {
        label {
            class: "cursor-pointer label",
            a {
                "Network"
            }
            select {
                class: "select max-w-xs",
                // TODO:
                // value: "{networkConfiguration}",
                // onChange={(e) => setNetworkConfiguration(e.target.value)}
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
