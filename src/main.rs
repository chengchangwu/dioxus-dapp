use dioxus::prelude::*;
use dioxus_dapp::{footer::Footer, home::HomeView, Package, Wallet};

static STYLES: &'static str = include_str!("../output.css");

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let pkg = Package { version: "0.1.0" };
    let wallet = Wallet { balance: 0 };
    cx.render(rsx! {
        style {
            vec![STYLES]
        }
        HomeView {
            pkg: pkg,
            wallet: wallet,
        }
        Footer {}
    })
}
