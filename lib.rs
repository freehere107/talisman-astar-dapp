#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod talisman_dapp {
    #[ink(storage)]
    pub struct TalismanDapp {}

    impl TalismanDapp {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn ping(&self) -> bool {
            true
        }
    }
}
