#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::CodeBlock;

pub fn Download() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-4 h-full font-hero max-w-3xl w-full mx-auto pb-20 leading-7",
            div {
                class: "flex flex-col gap-4",
                p {
                    class: "text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold font-hero",
                    "Download"
                }
                p {
                    class: "text-lg sm:text-xl md:text-2xl lg:text-3xl font-bold mt-8 font-hero",
                    "CLI"
                }
                p {
                    "Use the ORE CLI to run a miner on any machine. "
                    "To get started, ensure you have Rust and cargo installed. "
                }
                CodeBlock {
                    text: "curl https://sh.rustup.rs -sSf | sh"
                }
                p {
                    "Next, install the Solana CLI and create a Solana keypair if you haven't done so already. "
                }
                CodeBlock {
                    text: "sh -c \"$(curl -sSfL https://release.solana.com/v1.18.4/install)\"\nsolana-keygen new"
                }
                p {
                    "Now, install the ORE CLI."
                }
                CodeBlock {
                    text: "cargo install ore-cli"
                }
                p {
                    "The ORE CLI uses your default Solana CLI config and identity. "
                    "Ensure you have enough SOL topped up on this account to pay for transaction fees. "
                    "To begin mining, use the mine command."
                }
                CodeBlock {
                    text: "ore \\\n    --rpc <RPC_URL> \\\n    --keypair <KEYPAIR_FILEPATH> \\\n    --priority-fee <MICROLAMPORTS> \\\n    mine \\\n    --threads <TRHEAD_COUNT>"
                }
                p {
                    "To mine in detached mode, use nohup."
                }
                CodeBlock {
                    text: "nohup ore mine > output.log 2>&1 &"
                }
            }
        }
    }
}
