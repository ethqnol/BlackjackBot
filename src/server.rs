#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use log::LevelFilter;
use dioxus_fullstack::prelude::*;
use chrono::{DateTime, Utc};

enum CardType {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten, 
    Jack,
    Queen,
    King
}

enum Suits {
    Spade,
    Heart,
    Diamond,
    Club
}

struct Card {
    value: i32,
    face: CardType,
    suit: Suits,
    is_ace: bool,
}

impl Card {
    pub fn new(value : i32, face : CardType, suit : Suits, is_ace: bool ) -> Self {
        return Self { value : value, face : face, suit : suit, is_ace : is_ace }
    }
}

struct Game {
    decks : i32,
    card_pool : Vec<Card>,
    player_number : i32,
}

struct CardCount {
    hand : Vec<Card>,
    total_cards : i32,
    remaining_cards : i32,
    bet_size : i32,
}

#[server(PostServerData)]
pub async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
pub async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}