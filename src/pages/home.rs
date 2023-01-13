use crate::views::HomeView;
use crate::{Package, Wallet};
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
    let pkg = Package { version: "0.1.0" };
    let wallet = Wallet { balance: 0 };
    cx.render(rsx! {
        HomeView {
            pkg: pkg,
            wallet: wallet,
        }
    })
}
