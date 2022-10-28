use dioxus::prelude::*;
use dioxus_dapp::{footer::Footer, home::HomeView, wallets, Package, Wallet};

static STYLES: &'static str = include_str!("../output.css");

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    let window = wallets::window().unwrap();
    let solana = window.solana();
    log::info!("Solano {solana:?}");
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let pkg = Package { version: "0.1.0" };
    let wallet = Wallet { balance: 0 };
    cx.render(rsx! {
        style {
            vec![STYLES]
        }
        div {
            class: "flex flex-col h-screen",
            HomeView {
                pkg: pkg,
                wallet: wallet,
            }
            Footer {}
        }
    })
}
