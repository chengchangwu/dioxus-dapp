use crate::views::BasicsView;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Basics(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            // TODO:
            // Head {
            //     title {
            //         "Solana Scaffold"
            //     }
            //     meta {
            //         name: "description",
            //         content: "Basic Functionality",
            //     }
            // }
            BasicsView {
            }
        }
    })
}
