use dioxus::prelude::*;

#[derive(Props)]
pub struct WalletModalButtonProps<'a> {
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn WalletModalButton<'a>(cx: Scope<'a, WalletModalButtonProps<'a>>) -> Element<'a> {
    log::debug!("modal button children : {:?}", &cx.props.children);
    cx.render(rsx! {
        // TODO
        &cx.props.children
    })
}
