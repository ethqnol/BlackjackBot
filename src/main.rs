#![allow(non_snake_case, unused)]
use struct_iterable::Iterable;
use dioxus::prelude::*;
use log::LevelFilter;

mod server;


#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn CardList() -> Element {
    rsx! {
        style { {include_str!("styles/cardlist.css")} },
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


#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count += 5, "add 5!" }
            button { onclick: move |_| count /= 3, "Decimal Points"}
            CardList{}
            button {
                onclick: move |_| async move {
                    if let Ok(data) = server::get_server_data().await {
                        log::info!("Client received: {}", data);
                        text.set(data.clone());
                        server::post_server_data(data).await.unwrap();
                    }
                },
                "Get Server Data"
            }
            p { "Server data: {text}"}
        }
    }
}


