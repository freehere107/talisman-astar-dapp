// dApp staking whitelisted contract
//
// Project overview:
// Talisman ([talisman.xyz](https://www.talisman.xyz/)) is a multi-chain wallet and web application that enables users to “Traverse the Paraverse” with ease.
// We believe in a future where everyone has freedom via true agency over their intentions and finances, and we believe that the Polkadot ecosystem will bring this to fruition.
//
// For more on Talisman’s mission and approach, please see Nipsey’s talk at Polkadot Decoded:
// [Talisman: How Polkadot enables a trailblazing wallet stack | Polkadot Decoded 2023](https://www.youtube.com/watch?v=Q6M_HXsQoko)
//
// Today, because of its early stage of development and complex multi-chain architecture, both Polkadot and Kusama are changing rapidly and the user experience (UX) can be confusing for end users.
// It’s our goal to solve that using design, storytelling, and engineering.
// We abstract away the complexity of underlying implementation and provide a friendly user experience when using Polkadot and a way to discover new applications and services available to end users, while keeping them safe and educated about the actions they are taking.
//
// Talisman and Astar first announced Astar support in the wallet back in March 2022, and has Talisman has continued to integrate new features along the way:
//
// - Talisman Wallet supports portfolio management and viewing/sending of assets on Astar Wasm and EVM
// - Talisman Portal supports portfolio management of Astar Wasm and EVM assets, and allows you to connect via any wallet (Subwallet, Nova, Enkrypt, PJS, etc.)
// - Talisman Portal supports viewing of Astar EVM NFTs (Wasm NFTs in the future)
// - Talisman Portal supported the original Astar crowdloan, as well as the recent claiming of unlocked DOT from the original crowdloan
// - Talisman is frequently recommended as a wallet to use by Astar projects
//
// We are part of the builders program, check out our proposal here:
// https://forum.astar.network/t/talisman-dapp-staking-proposal/5747
//
// You can find us here:
// Website: https://www.talisman.xyz/
// Discord: https://discord.gg/talisman
// X: https://x.com/wearetalisman
// Medium: https://medium.com/we-are-talisman
// GitHub: https://github.com/TalismanSociety

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
