use dioxus::prelude::*;

#[derive(Props)]
pub struct WalletModalProps<'a> {
    class: &'a str,
}

#[allow(non_snake_case)]
pub fn WalletModal<'a>(cx: Scope<'a, WalletModalProps<'a>>) -> Element<'a> {
    let fade_in = true;
    let fade_in = if fade_in {
        "wallet-adapter-modal-fade-in"
    } else {
        ""
    };
    let class = cx.props.class;
    cx.render(rsx! {
        // portal &&
        // createPortal(
            div {
                aria_labelledby: "wallet-adapter-modal-title",
                aria_modal: "true",
                // className={`wallet-adapter-modal ${fadeIn && 'wallet-adapter-modal-fade-in'} ${className}`}
                class: "wallet-adapter-modal {fade_in} {class}",
                // ref={ref}
                role: "dialog",
                div {
                    class: "wallet-adapter-modal-container",
                    div {
                        class: "wallet-adapter-modal-wrapper",
                        button {
                            // onClick={handleClose},
                            class: "wallet-adapter-modal-button-close",
                            svg {
                                width: "14",
                                height: "14",
                                path {
                                    d: "M14 12.461 8.3 6.772l5.234-5.233L12.006 0 6.772 5.234 1.54 0 0 1.539l5.234 5.233L0 12.006l1.539 1.528L6.772 8.3l5.69 5.7L14 12.461z",
                                }
                            }
                        }
        //                 {installedWallets.length ? (
        //                     <>
        //                         <h1 className="wallet-adapter-modal-title">Connect a wallet on Solana to continue</h1>
        //                         <ul className="wallet-adapter-modal-list">
        //                             {installedWallets.map((wallet) => (
        //                                 <WalletListItem
        //                                     key={wallet.adapter.name}
        //                                     handleClick={(event) => handleWalletClick(event, wallet.adapter.name)}
        //                                     wallet={wallet}
        //                                 />
        //                             ))}
        //                             {otherWallets.length ? (
        //                                 <Collapse expanded={expanded} id="wallet-adapter-modal-collapse">
        //                                     {otherWallets.map((wallet) => (
        //                                         <WalletListItem
        //                                             key={wallet.adapter.name}
        //                                             handleClick={(event) =>
        //                                                 handleWalletClick(event, wallet.adapter.name)
        //                                             }
        //                                             tabIndex={expanded ? 0 : -1}
        //                                             wallet={wallet}
        //                                         />
        //                                     ))}
        //                                 </Collapse>
        //                             ) : null}
        //                         </ul>
        //                         {otherWallets.length ? (
        //                             <button
        //                                 className="wallet-adapter-modal-list-more"
        //                                 onClick={handleCollapseClick}
        //                                 tabIndex={0}
        //                             >
        //                                 <span>{expanded ? 'Less ' : 'More '}options</span>
        //                                 <svg
        //                                     width="13"
        //                                     height="7"
        //                                     viewBox="0 0 13 7"
        //                                     xmlns="http://www.w3.org/2000/svg"
        //                                     className={`${
        //                                         expanded ? 'wallet-adapter-modal-list-more-icon-rotate' : ''
        //                                     }`}
        //                                 >
        //                                     <path d="M0.71418 1.626L5.83323 6.26188C5.91574 6.33657 6.0181 6.39652 6.13327 6.43762C6.24844 6.47872 6.37371 6.5 6.50048 6.5C6.62725 6.5 6.75252 6.47872 6.8677 6.43762C6.98287 6.39652 7.08523 6.33657 7.16774 6.26188L12.2868 1.626C12.7753 1.1835 12.3703 0.5 11.6195 0.5H1.37997C0.629216 0.5 0.224175 1.1835 0.71418 1.626Z" />
        //                                 </svg>
        //                             </button>
        //                         ) : null}
        //                     </>
        //                 ) : (
        //                     <>
        //                         <h1 className="wallet-adapter-modal-title">
        //                             You'll need a wallet on Solana to continue
        //                         </h1>
        //                         <div className="wallet-adapter-modal-middle">
        //                             <WalletSVG />
        //                             <button
        //                                 type="button"
        //                                 className="wallet-adapter-modal-middle-button"
        //                                 onClick={(event) => handleWalletClick(event, getStartedWallet.adapter.name)}
        //                             >
        //                                 Get started
        //                             </button>
        //                         </div>
        //                         {otherWallets.length ? (
        //                             <>
        //                                 <button
        //                                     className="wallet-adapter-modal-list-more"
        //                                     onClick={handleCollapseClick}
        //                                     tabIndex={0}
        //                                 >
        //                                     <span>{expanded ? 'Hide ' : 'Already have a wallet? View '}options</span>
        //                                     <svg
        //                                         width="13"
        //                                         height="7"
        //                                         viewBox="0 0 13 7"
        //                                         xmlns="http://www.w3.org/2000/svg"
        //                                         className={`${
        //                                             expanded ? 'wallet-adapter-modal-list-more-icon-rotate' : ''
        //                                         }`}
        //                                     >
        //                                         <path d="M0.71418 1.626L5.83323 6.26188C5.91574 6.33657 6.0181 6.39652 6.13327 6.43762C6.24844 6.47872 6.37371 6.5 6.50048 6.5C6.62725 6.5 6.75252 6.47872 6.8677 6.43762C6.98287 6.39652 7.08523 6.33657 7.16774 6.26188L12.2868 1.626C12.7753 1.1835 12.3703 0.5 11.6195 0.5H1.37997C0.629216 0.5 0.224175 1.1835 0.71418 1.626Z" />
        //                                     </svg>
        //                                 </button>
        //                                 <Collapse expanded={expanded} id="wallet-adapter-modal-collapse">
        //                                     <ul className="wallet-adapter-modal-list">
        //                                         {otherWallets.map((wallet) => (
        //                                             <WalletListItem
        //                                                 key={wallet.adapter.name}
        //                                                 handleClick={(event) =>
        //                                                     handleWalletClick(event, wallet.adapter.name)
        //                                                 }
        //                                                 tabIndex={expanded ? 0 : -1}
        //                                                 wallet={wallet}
        //                                             />
        //                                         ))}
        //                                     </ul>
        //                                 </Collapse>
        //                             </>
        //                         ) : null}
        //                     </>
        //                 )}
                    }
                }
                div {
                    class: "wallet-adapter-modal-overlay",
                    // onMouseDown={handleClose}
                }
            }
        //     portal
        // )
    })
}
