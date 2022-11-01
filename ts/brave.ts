interface BraveWalletEvents {
    connect(...args: unknown[]): unknown;
    disconnect(...args: unknown[]): unknown;
}

interface BraveWallet {
    isBraveWallet?: boolean;
    publicKey?: { toBytes(): Uint8Array };
    isConnected: boolean;
    signMessage(message: Uint8Array): Promise<{ signature: Uint8Array }>;
    connect(): Promise<void>;
    disconnect(): Promise<void>;
}

interface BraveWindow extends Window {
    solana?: BraveWallet;
}

declare const window: BraveWindow;

export function detect_brave_wallet() {
    if (window.solana?.isBraveWallet) {
        return true;
    }
    return false;
}