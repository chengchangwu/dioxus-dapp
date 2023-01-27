use std::future::Future;

use super::super::web3::PublicKey;

pub struct Wallet {}
pub struct WalletContextState {
    // auto_connect: bool,
    // wallets: Vec<Wallet>,
    wallet: Option<Wallet>,
    public_key: Option<PublicKey>,
    connecting: bool,
    connected: bool,
    disconnecting: bool,

    // select(walletName: WalletName | null): void;
    connect: Box<dyn Future<Output = ()>>,
    disconnect: Box<dyn Future<Output = ()>>,
    // sendTransaction: WalletAdapterProps['sendTransaction'];
    // signTransaction: SignerWalletAdapterProps['signTransaction'] | undefined;
    // signAllTransactions: SignerWalletAdapterProps['signAllTransactions'] | undefined;
    // signMessage: MessageSignerWalletAdapterProps['signMessage'] | undefined;
}

pub fn use_wallet() -> (bool, Option<Wallet>, bool) {
    (false, None, false)
}
