#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod talisman_astar_dapp {
    #[ink(storage)]
    pub struct TalismanAstarDapp {}

    impl TalismanAstarDapp {
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
