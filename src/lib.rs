pub mod components;
pub mod views;
pub mod wallets;

#[derive(PartialEq)]
pub struct Package<'a> {
    pub version: &'a str,
}

#[derive(PartialEq)]
pub struct Wallet {
    pub balance: usize,
}
