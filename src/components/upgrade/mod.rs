mod confirm;
mod done;
mod edit;

use confirm::*;
use done::*;
use edit::*;

use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::{signature::Signature, transaction::Transaction};

#[derive(Clone)]
pub enum UpgradeStep {
    Edit,
    Confirm(Transaction),
    Done(Signature),
}

#[component]
pub fn Upgrade() -> Element {
    let upgrade_step = use_signal(|| UpgradeStep::Edit);
    let amount_input = use_signal(|| "".to_string());
    let parsed_amount: u64 = match amount_input.read().parse::<f64>() {
        Ok(n) => (n * 10f64.powf(ore::TOKEN_DECIMALS_V1.into())) as u64,
        Err(_) => 0,
    };

    match upgrade_step.cloned() {
        UpgradeStep::Edit => {
            rsx! {
                UpgradeEdit {
                    upgrade_step: upgrade_step,
                    amount_input: amount_input,
                    parsed_amount: parsed_amount
                }
            }
        }
        UpgradeStep::Confirm(tx) => {
            rsx! {
                UpgradeConfirm {
                    upgrade_step: upgrade_step,
                    tx: tx,
                    amount: parsed_amount,
                }
            }
        }
        UpgradeStep::Done(signature) => {
            rsx! {
                UpgradeDone {
                    signature: signature,
                    amount: parsed_amount
                }
            }
        }
    }
}
