use dioxus::prelude::{use_shared_state, use_shared_state_provider, ScopeState, UseSharedState};

use super::super::web3::PublicKey;

pub struct Wallet {}

pub struct WalletContextState {
    auto_connect: bool,
    wallets: Vec<Wallet>,
    pub wallet: Option<Wallet>,
    public_key: Option<PublicKey>,
    connecting: bool,
    connected: bool,
    disconnecting: bool,
    // select(walletName: WalletName | null): void;
    // connect: Box<dyn Future<Output = ()>>,
    // disconnect: Box<dyn Future<Output = ()>>,
    // sendTransaction: WalletAdapterProps['sendTransaction'];
    // signTransaction: SignerWalletAdapterProps['signTransaction'] | undefined;
    // signAllTransactions: SignerWalletAdapterProps['signAllTransactions'] | undefined;
    // signMessage: MessageSignerWalletAdapterProps['signMessage'] | undefined;
}

pub fn use_wallet(cx: &ScopeState) -> Option<UseSharedState<WalletContextState>> {
    use_shared_state::<WalletContextState>(cx)
}

/// Provide wallet state for components down the hierarchy to consume without having to drill props.
pub fn use_wallet_provider(cx: &ScopeState) {
    use_shared_state_provider(cx, || WalletContextState {
        auto_connect: false,
        wallets: vec![],
        wallet: None,
        public_key: None,
        connecting: false,
        connected: false,
        disconnecting: false,
    });
}
