use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn BasicsView(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "md:hero mx-auto p-4",
            div {
                class: "md:hero-content flex flex-col",
                h1 {
                    class: "text-center text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-tr from-[#9945FF] to-[#14F195]",
                    "Basics"
                }
                // CONTENT GOES HERE
                div {
                    class: "text-center",
                    // TODO
                    // SignMessage {
                    // }
                    // SendTransaction {
                    // }
                }
            }
        }
    })
}
