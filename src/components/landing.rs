use std::collections::HashMap;

use dioxus::prelude::*;
use num_format::{Locale, ToFormattedString};
use serde::Deserialize;
use solana_client_wasm::solana_sdk::blake3::Hash as Blake3Hash;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;
use web_time::{Duration, Instant};

use crate::{
    components::{
        DiscordIcon, Footer, FuzzlandIcon, GithubIcon, OreIcon, OreLogoIcon, OttersecIcon, XIcon,
    },
    hooks::{use_is_onboarded, use_ore_supply, UiTokenAmountBalance},
    miner::WEB_WORKERS,
    route::Route,
    utils::asset_path,
};

#[derive(Copy, Clone, PartialEq, Eq)]
enum TextColor {
    Black,
    White,
}

pub fn Landing() -> Element {
    // let mut current_page = use_signal(|| 0);
    let nav = navigator();
    let is_onboarded = use_is_onboarded();
    let mut i = use_signal(|| 0usize);
    let themes = [
        // (asset_path("rock.png"), TextColor::Black),
        // (asset_path("rock-10.png"), TextColor::Black),
        // (asset_path("rock-11.png"), TextColor::Black),
        (asset_path("rock-2.jpg"), TextColor::White),
        (asset_path("rock-3.jpg"), TextColor::White),
        (asset_path("rock-4.jpg"), TextColor::White),
        (asset_path("rock-6.jpg"), TextColor::White),
        (asset_path("rock-5.jpg"), TextColor::White),
        (asset_path("rock-9.jpg"), TextColor::White),
    ];
    let len = themes.len();
    let text_color = themes[*i.read() % len].1;

    // Change the background image every 8 sec
    use_future(move || async move {
        loop {
            async_std::task::sleep(Duration::from_secs(10)).await;
            i.set(i.cloned().saturating_add(1));
        }
    });

    // If the user is already onboarded, redirect to home.
    if is_onboarded.read().0 {
        nav.replace(Route::Home {});
    }

    rsx! {
        for (index, theme) in themes.iter().enumerate() {
            BgImg {
                visible: *i.read() % len == index,
                bg_img: theme.0.clone(),
                index
            }
        }
        div {
            class: "absolute top-0 flex flex-col w-full h-full overflow-y-scroll z-50 snap-y snap-mandatory",
            Hero {
                text_color,
                title: "It's time to mine.",
                subtitle: &"ORE is a fair-launch, proof-of-work, digital currency everyone can mine."
            }
            Block {
                title: &"Proof of work.",
                title2: &"On Solana.",
                detail: &"ORE can be mined on any laptop, phone, or home computer. You don't need advanced hardware or a software degree to get started.",
                section: Section::A,
                text_color
            }
            Block {
                title: &"Fixed supply.",
                title2: &"Predictable future.",
                detail: &"ORE has a total maximum supply of 21m tokens. At a steady rate of one token per minute, all ORE in existence will be mined by the year 2064.",
                section: Section::B,
                text_color
            }
            Block {
                title: &"Borderless asset.",
                title2: &"Permissionless cash.",
                detail: &"ORE is internet-native money that moves at the speed of the light. It can be sent to anyone, anywhere in the world, in under a second, with negligable fees.",
                section: Section::D,
                text_color
            }
            Block {
                title: &"Fair launch.",
                title2: &"Immutable code.",
                detail: &"ORE has no insider token allocation nor pre-mined supply. The smart contract is open source and has been reviewed by multiple world-class auditing firms.",
                section: Section::C,
                text_color
                // TODO Sec3
                // TODO Neodyme
            }
            Footer {
                transparent_bg: true,
                show_site_map: true
            }
        }
    }
}

#[component]
fn BgImg(visible: bool, bg_img: String, index: usize) -> Element {
    let visibility = if visible { "opacity-100" } else { "opacity-0" };
    rsx! {
        div {
            key: "{index}",
            class: "fixed top-0 w-full h-full bg-cover bg-center transition-opacity duration-1000 z-0 {visibility}",
            style: "background-image: url({bg_img})"
        }
    }
}

#[component]
fn Navbar(text_color: TextColor) -> Element {
    let copy_color = match text_color {
        TextColor::Black => "text-black",
        TextColor::White => "text-white",
    };
    rsx! {
        div {
            class: "flex flex-row justify-between px-4 sm:px-8 py-6 md:py-8 w-full transition-colors {copy_color}",
            Link {
                to: Route::Landing {},
                class: "flex flex-row h-10 my-auto",
                OreLogoIcon {
                    class: "h-6 md:h-8 my-auto"
                }
            }
            SocialLinks {
                text_color
            }
        }
    }
}

#[component]
fn SocialLinks(text_color: TextColor) -> Element {
    let button_color = match text_color {
        TextColor::Black => "text-black hover:bg-black hover:text-white",
        TextColor::White => "text-white hover:bg-white hover:text-black",
    };
    rsx! {
        div {
            class: "flex flex-row sm:text-sm md:text-base lg:text-lg my-auto gap-4 md:gap-8",
            Link {
                to: "https://discord.gg/ore-supply",
                class: "flex h-10 w-10 transition-colors rounded-full transition-colors {button_color}",
                new_tab: true,
                DiscordIcon {
                    class: "w-6 h-6 m-auto"
                }
            }
            Link {
                to: "https://github.com/regolith-labs/ore",
                class: "flex h-10 w-10 transition-colors rounded-full transition-colors {button_color}",
                new_tab: true,
                GithubIcon {
                    class: "w-6 h-6 m-auto"
                }
            }
            Link {
                to: "https://x.com/oresupply",
                class: "flex h-10 w-10 transition-colors rounded-full transition-colors {button_color}",
                new_tab: true,
                XIcon {
                    class: "w-5 h-5 m-auto"
                }
            }
        }
    }
}

#[component]
fn Hero(title: String, subtitle: String, text_color: TextColor) -> Element {
    let copy_color = match text_color {
        TextColor::Black => "text-black selection:bg-black selection:text-white",
        TextColor::White => "text-white selection:bg-white selection:text-black",
    };
    let cta_color = match text_color {
        TextColor::Black => "bg-black text-white selection:bg-black selection:text-white",
        TextColor::White => "bg-white text-black selection:bg-white selection:text-black",
    };
    rsx! {
        div {
            class: "flex flex-col min-h-dvh h-full w-full snap-start snap-always",
            Navbar {
                text_color
            }
            div {
                class: "flex flex-col gap-y-8 sm:gap-y-10 md:gap-y-12 w-full md:mx-auto my-auto pb-24 px-4 md:px-8",
                div {
                    class: "flex flex-col gap-y-4 sm:gap-y-6 md:gap-y-8 {copy_color} transition-colors",
                    p {
                        class: "text-left sm:text-center text-6xl md:text-7xl lg:text-8xl font-bold font-hero",
                        "{title}"
                    }
                    p {
                        class: "text-left sm:text-center text-xl sm:text-2xl md:text-3xl lg:text-4xl mx-auto font-hero font-medium",
                        "{subtitle}"
                    }
                }
                Link {
                    class: "mr-auto sm:mx-auto text-center sm:text-lg md:text-xl lg:text-2xl font-semibold transition-colors transition-transform hover:scale-105 hover:shadow {cta_color} px-6 py-3 rounded-full",
                    to: Route::Home {},
                    "Get started →"
                }
            }
        }
    }
}

#[component]
fn Block(
    title: String,
    title2: String,
    detail: String,
    section: Section,
    text_color: TextColor,
) -> Element {
    let copy_color = match text_color {
        TextColor::Black => "text-black selection:bg-black selection:text-white",
        TextColor::White => "text-white selection:bg-white selection:text-black",
    };
    rsx! {
        div {
            class: "flex min-h-dvh h-full w-full py-8 md:py-16 px-3 sm:px-8 snap-start",
            div {
                class: "flex flex-col h-full w-full justify-between",
                div {
                    class: "flex flex-col gap-4 sm:gap-6 md:gap-8 transition-colors {copy_color}",
                    p {
                        class: "text-4xl md:text-5xl lg:text-6xl font-bold font-hero",
                        "{title}"
                        br {}
                        span {
                            class: "opacity-70",
                            "{title2}"
                        }
                    }
                    p {
                        class: "text-xl md:text-2xl lg:text-3xl leading-relaxed max-w-[48rem] font-medium font-hero",
                        "{detail}"
                    }
                    BlockCta {
                        section: section.clone(),
                        text_color
                    }
                }
                div {
                    class: "flex h-full w-full",
                    match section {
                        Section::A => rsx! { SectionA { text_color } },
                        Section::B => rsx! { SectionB { text_color } },
                        Section::C => rsx! { SectionC { text_color } },
                        Section::D => rsx! { SectionD { text_color } },
                    }
                }
            }
        }
    }
}

#[component]
fn BlockCta(section: Section, text_color: TextColor) -> Element {
    let style = "flex shrink font-semibold text-center mr-auto mt-4 px-5 py-3 transition-colors transition-transform rounded-full hover:scale-105 hover:shadow";
    let cta_color = match text_color {
        TextColor::Black => "bg-black text-white",
        TextColor::White => "bg-white text-black",
    };
    match section {
        Section::A => rsx! {
            Link {
                class: "{style} {cta_color}",
                to: Route::Download {},
                "Download the app →"
            }
        },
        Section::B => rsx! {
            Link {
                class: "{style} {cta_color}",
                to: Route::OreTokenomics {},
                "Learn more →"
            }
        },
        Section::C => rsx! {
            Link {
                class: "{style} {cta_color}",
                to: "https://github.com/regolith-labs/ore",
                new_tab: true,
                "Checkout the code →"
            }
        },
        Section::D => rsx! {
            Link {
                class: "{style} {cta_color}",
                to: "https://jup.ag/swap/USDC-ORE",
                new_tab: true,
                "Buy now →"
            }
        },
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Section {
    A,
    B,
    C,
    D,
}

// TODO Hash animation
// TODO Current hashpower measurement?
#[component]
fn SectionA(text_color: TextColor) -> Element {
    let copy_color = match text_color {
        TextColor::Black => "text-black",
        TextColor::White => "text-white",
    };

    let mut sample_hash = use_signal(|| Blake3Hash::new_unique());

    let hashrate = use_resource(move || async move {
        let size = 10u64;
        let t = Instant::now();
        for i in 0..size {
            let _ = drillx::hash(&[0; 32], &i.to_le_bytes());
        }
        60_000u128
            .saturating_div(t.elapsed().as_millis())
            .saturating_mul(size.into())
            .saturating_mul(*WEB_WORKERS as u128)
    });

    // Animate the hash to visualize mining.
    use_future(move || async move {
        loop {
            async_std::task::sleep(std::time::Duration::from_millis(125)).await;
            sample_hash.set(Blake3Hash::new_unique());
        }
    });

    rsx! {
        div {
            class: "flex flex-col w-full my-auto gap-8 md:gap-12 max-w-[48rem]",
            if let Some(hashrate) = hashrate.cloned() {
                div {
                    class: "flex flex-col gap-2 {copy_color} transition-colors",
                    p {
                        class: "opacity-80 font-medium",
                        "Your hashpower (est.)"
                    }
                    div {
                        class: "flex flex-row gap-2",
                        p {
                            class: "text-2xl md:text-3xl lg:text-4xl font-bold font-hero",
                            "{hashrate} H/min"
                        }
                    }
                }
            }
            div {
                class: "flex flex-col gap-2 {copy_color} transition-colors",
                p {
                    class: "opacity-80 font-medium",
                    "Sample"
                }
                div {
                    class: "flex flex-row gap-2",
                    p {
                        class: "text-2xl md:text-3xl lg:text-4xl font-bold font-mono",
                        "{sample_hash.cloned().to_string()[1..17]}"
                    }
                }
            }
        }
    }
}

#[component]
fn SectionB(text_color: TextColor) -> Element {
    let supply = use_ore_supply();
    let circulating_supply = supply
        .cloned()
        .and_then(|s| s.ok())
        .map(|s| amount_to_ui_amount(s.balance(), s.decimals))
        .unwrap_or_else(|| 0f64) as u64;
    rsx! {
        div {
            class: "flex flex-col gap-8 md:gap-12 my-auto",
            OreValue {
                title: "Current supply (devnet)".to_string(),
                amount: circulating_supply,
                text_color
            }
            OreValue {
                title: "Total supply".to_string(),
                amount: 21_000_000,
                text_color
            }
        }
    }
}

#[component]
fn OreValue(title: String, amount: u64, text_color: TextColor) -> Element {
    let copy_color = match text_color {
        TextColor::Black => "text-black",
        TextColor::White => "text-white",
    };
    rsx! {
        div {
            class: "flex flex-col gap-2 {copy_color} transition-colors",
            p {
                class: "opacity-80 font-medium",
                "{title}"
            }
            div {
                class: "flex flex-row gap-2",
                OreIcon {
                    class: "w-6 h-6 md:w-7 md:h-7 lg:w-8 lg:h-8 my-auto"
                }
                p {
                    class: "text-2xl md:text-3xl lg:text-4xl font-bold font-hero",
                    "{amount.to_formatted_string(&Locale::en)}"
                }
            }
        }
    }
}

#[component]
fn SectionC(text_color: TextColor) -> Element {
    let text_color = match text_color {
        TextColor::Black => "text-black",
        TextColor::White => "text-white",
    };
    rsx! {
        div {
            class: "flex flex-col gap-2 my-auto",
            p {
                class: "opacity-80 font-medium {text_color}",
                "Audited by"
            }
            div {
                class: "flex flex-row gap-8 md:gap-12",
                Link {
                    to: "https://osec.io/",
                    class: "flex p-2 md:p-4 transition-colors rounded-full transition-colors {text_color}",
                    new_tab: true,
                    OttersecIcon {
                        class: "w-10 h-10 md:w-12 md:h-12 m-auto"
                    }
                }
                Link {
                    to: "https://fuzz.land/",
                    class: "flex p-2 md:p-4 transition-colors rounded-full transition-colors {text_color}",
                    new_tab: true,
                    FuzzlandIcon {
                        class: "w-10 h-10 md:w-12 md:h-12 m-auto"
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct JupPriceApiResponse {
    data: HashMap<String, JupPriceData>,
    #[serde(rename = "timeTaken")]
    _time_taken: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JupPriceData {
    #[serde(rename = "id")]
    _id: String,
    #[serde(rename = "mintSymbol")]
    _mint_symbol: String,
    #[serde(rename = "vsToken")]
    _vs_token: String,
    #[serde(rename = "vsTokenSymbol")]
    _vs_token_symbol: String,
    price: f64,
}

#[component]
fn SectionD(text_color: TextColor) -> Element {
    let text_color = match text_color {
        TextColor::Black => "text-black",
        TextColor::White => "text-white",
    };

    let quotes = use_resource(move || async move {
        reqwest::get("https://price.jup.ag/v6/price?ids=USDC,EURC,WBTC&vsToken=ORE")
            .await?
            .json::<JupPriceApiResponse>()
            .await
    });

    rsx! {
        div {
            class: "flex flex-row flex-wrap gap-8 md:gap-12 my-auto align-top transition-colors {text_color}",
            if let Some(Ok(quotes)) = &*quotes.read() {
                Quote {
                    title: "ORE/USDC",
                    price: quotes.data["USDC"].price,
                    symbol: "$",
                    decimals: 2
                }
                Quote {
                    title: "ORE/EURC",
                    price: quotes.data["EURC"].price,
                    symbol: "€",
                    decimals: 2
                }
                Quote {
                    title: "ORE/WBTC",
                    price: quotes.data["WBTC"].price,
                    symbol: "₿",
                    decimals: 8
                }
            }
        }
    }
}

#[component]
fn Quote(title: String, price: f64, symbol: String, decimals: usize) -> Element {
    let price = format!("{0:.1$}", 1f64 / price, decimals);
    rsx! {
        div {
            class: "flex flex-col gap-2",
            p {
                class: "opacity-80 font-medium",
                "{title}"
            }
            div {
                class: "flex flex-row gap-0.5 text-2xl md:text-3xl lg:text-4xl font-bold font-hero",
                p {
                    "{symbol}"
                }
                p {
                    "{price}"
                }
            }
        }
    }
}
