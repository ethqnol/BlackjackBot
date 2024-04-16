#![allow(non_snake_case, unused)]
use struct_iterable::Iterable;
use dioxus::prelude::*;
use log::LevelFilter;
use crate::server;

#[component]
pub fn CardList() -> Element {
    rsx! {
        style { {include_str!("../styles/cardlist.css")} },
        div { class: "card",
            h1 { "Card Types:" }
            p{ "{server::types::Deck::KING_CLUB.stringify()}" }
            p{ "{server::types::Deck::KING_HEART.stringify()}" }
            p{ "{server::types::Deck::KING_SPADE.stringify()}" }
            p{ "{server::types::Deck::KING_DIA.stringify()}" }
            p{ "{server::types::Deck::QUEEN_CLUB.stringify()}" }
            p{ "{server::types::Deck::QUEEN_SPADE.stringify()}" }
            p{ "{server::types::Deck::QUEEN_HEART.stringify()}" }
            p{ "{server::types::Deck::QUEEN_DIA.stringify()}" }
            p{ "{server::types::Deck::JACK_CLUB.stringify()}" }
            p{ "{server::types::Deck::JACK_SPADE.stringify()}" }
            p{ "{server::types::Deck::JACK_HEART.stringify()}" }
            p{ "{server::types::Deck::JACK_DIAM.stringify()}" }
            p{ "{server::types::Deck::TEN_CLUB.stringify()}" }
            p{ "{server::types::Deck::TEN_SPADE.stringify()}" }
            p{ "{server::types::Deck::TEN_HEART.stringify()}" }
            p{ "{server::types::Deck::TEN_DIAM.stringify()}" }
            p{ "{server::types::Deck::NINE_CLUB.stringify()}" }
            p{ "{server::types::Deck::NINE_SPADE.stringify()}" }
            p{ "{server::types::Deck::NINE_HEART.stringify()}" }
            p{ "{server::types::Deck::NINE_DIA.stringify()}" }
            p{ "{server::types::Deck::EIGHT_CLUB.stringify()}" }
            p{ "{server::types::Deck::EIGHT_SPADE.stringify()}" }
            p{ "{server::types::Deck::EIGHT_HEART.stringify()}" }
            p{ "{server::types::Deck::EIGHT_DIA.stringify()}" }
            p{ "{server::types::Deck::SEVEN_CLUB.stringify()}" }
            p{ "{server::types::Deck::SEVEN_SPADE.stringify()}" }
            p{ "{server::types::Deck::SEVEN_HEART.stringify()}" }
            p{ "{server::types::Deck::SEVEN_DIA.stringify()}" }
            p{ "{server::types::Deck::SIX_CLUB.stringify()}" }
            p{ "{server::types::Deck::SIX_SPADE.stringify()}" }
            p{ "{server::types::Deck::SIX_HEART.stringify()}" }
            p{ "{server::types::Deck::SIX_DIA.stringify()}" }
            p{ "{server::types::Deck::FIVE_CLUB.stringify()}" }
            p{ "{server::types::Deck::FIVE_SPADE.stringify()}" }
            p{ "{server::types::Deck::FIVE_HEART.stringify()}" }
            p{ "{server::types::Deck::FIVE_DIA.stringify()}" }
            p{ "{server::types::Deck::FOUR_CLUB.stringify()}" }
            p{ "{server::types::Deck::FOUR_SPADE.stringify()}" }
            p{ "{server::types::Deck::FOUR_HEART.stringify()}" }
            p{ "{server::types::Deck::FOUR_DIA.stringify()}" }
            p{ "{server::types::Deck::THREE_CLUB.stringify()}" }
            p{ "{server::types::Deck::THREE_SPADE.stringify()}" }
            p{ "{server::types::Deck::THREE_HEART.stringify()}" }
            p{ "{server::types::Deck::THREE_DIA.stringify()}" }
            p{ "{server::types::Deck::TWO_CLUB.stringify()}" }
            p{ "{server::types::Deck::TWO_SPADE.stringify()}" }
            p{ "{server::types::Deck::TWO_HEART.stringify()}" }
            p{ "{server::types::Deck::TWO_DIA.stringify()}" }
            p{ "{server::types::Deck::ACE_CLUB.stringify()}" }
            p{ "{server::types::Deck::ACE_SPADE.stringify()}" }
            p{ "{server::types::Deck::ACE_HEART.stringify()}" }
            p{ "{server::types::Deck::ACE_DIA.stringify()}" }
        }
    }
}