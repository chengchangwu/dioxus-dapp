pub mod components;
pub mod views;
pub mod wallets;

pub use components::{airdrop, footer};
pub use views::home;

#[derive(PartialEq)]
pub struct Package<'a> {
    pub version: &'a str,
}

#[derive(PartialEq)]
pub struct Wallet {
    pub balance: usize,
}
