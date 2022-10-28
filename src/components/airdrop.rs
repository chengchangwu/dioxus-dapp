use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn RequestAirdrop(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            button {
                class: "px-8 m-2 btn animate-pulse bg-gradient-to-r from-[#9945FF] to-[#14F195] hover:from-pink-500 hover:to-yellow-500 ...",
                // TODO
                onclick: move |_| {},
                span {
                    "Airdrop 1"
                }
            }
        }
    })
}
