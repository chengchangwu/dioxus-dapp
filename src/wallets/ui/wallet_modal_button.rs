use crate::wallets::ui::{Button, ButtonProps};
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn WalletModalButton<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    // TODO: reduce fields to be cloned?
    let props_clone = cx.props.clone();
    cx.render(rsx! {
        Button {
            ..props_clone
        }
    })
}
