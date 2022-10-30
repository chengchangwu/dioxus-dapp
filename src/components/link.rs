use dioxus::prelude::*;

#[derive(Props)]
pub struct LinkProps<'a> {
    href: &'a str,
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Link<'a>(cx: Scope<'a, LinkProps<'a>>) -> Element<'a> {
    // TODO
    cx.render(rsx! {
        &cx.props.children
    })
}
