pub mod airdrop;
pub mod home;

#[derive(PartialEq)]
pub struct Package<'a> {
    pub version: &'a str,
}

#[derive(PartialEq)]
pub struct Wallet {
    pub balance: usize,
}
