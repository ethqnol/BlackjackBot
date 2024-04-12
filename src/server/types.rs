#![allow(non_snake_case, unused)]

pub enum CardType {
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

pub enum Suits {
    Spade,
    Heart,
    Diamond,
    Club
}

pub struct Card {
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

pub struct Game {
    decks : i32,
    card_pool : Vec<Card>,
    player_number : i32,
}

pub struct CardCount {
    hand : Vec<Card>,
    total_cards : i32,
    remaining_cards : i32,
    bet_size : i32,
}
