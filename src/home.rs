use crate::airdrop::RequestAirdrop;
use crate::{Package, Wallet};
use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct HomeViewProps<'a> {
    pkg: Package<'a>,
    wallet: Option<Wallet>,
}

#[allow(non_snake_case)]
pub fn HomeView<'a>(cx: Scope<'a, HomeViewProps<'a>>) -> Element<'a> {
    let pkg = &cx.props.pkg;
    let wallet = &cx.props.wallet;
    cx.render(rsx! {
        div {
            class: "md:hero mx-auto p-4",
            div {
                class: "md:hero-content flex flex-col",
                h1 {
                    class: "text-center text-5xl md:pl-12 font-bold text-transparent bg-clip-text bg-gradient-to-tr from-[#9945FF] to-[#14F195]",
                    "Scaffold Lite"
                    span {
                        class: "text-sm font-normal align-top text-slate-700",
                        "v{pkg.version}"
                    }
                }
                h4 {
                    class: "md:w-full text-center text-slate-300 my-2",
                    p {
                        "Simply the fastest way to get started."
                    }
                    "Rust, dioxus, tailwind, wallet, web3.js, and more."
                }
                div {
                    class: "max-w-md mx-auto mockup-code bg-primary p-6 my-2",
                    pre {
                        // TODO:
                        // data_prefix: ">",
                        code {
                            class: "truncate",
                            "Start building on Solana"
                        }
                    }
                }
                div {
                    class: "text-center",
                    RequestAirdrop {}
                    // TODO:
                    // {wallet.publicKey && <p>Public Key: {wallet.publicKey.toBase58()}</p>}
                    wallet.as_ref().and_then(|w| {
                        Some(rsx!{
                            // TODO:
                            //     p {
                            //         // "SOL Balance: {(balance || 0).toLocaleString()}"
                            //     }
                            p {
                                "SOL Balance: {w.balance}"
                            }
                        })
                    })
                }
            }
        }
    })
}
