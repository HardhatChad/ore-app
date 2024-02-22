use std::str::FromStr;

use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{
    components::ActivityTable,
    gateway::AsyncResult,
    hooks::{use_ore_balance, use_user_transfers},
};

// TODO Not found

#[component]
pub fn User(cx: Scope, id: String) -> Element {
    let user_id = Pubkey::from_str(id);

    if user_id.is_err() {
        return render! {
            p {
                "Invalid user id"
            }
        };
    }

    let user_id = user_id.unwrap();
    let (balance, _) = use_ore_balance(cx, user_id);

    let container_class = "flex flex-row justify-between py-2 px-1";
    let title_class = "text-gray-300";
    let value_class = "font-medium";

    render! {
        div {
            class: "flex flex-col gap-16",
            div {
                class: "flex flex-col gap-2",
                h2 {
                    class: "text-lg md:text-2xl font-bold",
                    "User"
                }
                div {
                    class: "{container_class}",
                    p {
                        class: "{title_class}",
                        "ID"
                    }
                    p {
                        class: "{value_class}",
                        "{id}"
                    }
                }
                div {
                    class: "{container_class}",
                    p {
                        class: "{title_class}",
                        "Balance"
                    }
                    match balance {
                        AsyncResult::Ok(balance) => {
                            render! {
                                p {
                                    class: "{value_class}",
                                    "{balance.real_number_string_trimmed()}"
                                }
                            }
                        }
                        _ => {
                            render! {
                                p {
                                    class: "{value_class} w-16 h-8 bg-gray-100 animate-pulse",
                                }
                            }
                        }
                    }
                }
            }
            UserActivity {
                user_id: user_id
            }
        }
    }
}

#[component]
pub fn UserActivity(cx: Scope, user_id: Pubkey) -> Element {
    let offset = use_state(cx, || 0u64);
    let (transfers, has_more) = use_user_transfers(cx, *user_id, offset);
    match transfers {
        AsyncResult::Ok(transfers) => {
            render! {
                div {
                    class: "flex flex-col gap-4 grow w-full h-2/3 pb-20 min-h-16 rounded justify-start text-black",
                    div {
                        class: "flex flex-row justify-between",
                        h2 {
                            class: "text-lg md:text-2xl font-bold",
                            "Activity"
                        }
                    }
                    ActivityTable{
                        offset: offset,
                        transfers: transfers,
                        has_more: has_more
                    }
                }
            }
        }
        _ => {
            render! {
                div {
                    class: "flex flex-row h-64 w-full animate-pulse bg-gray-100 rounded",
                }
            }
        }
    }
}
