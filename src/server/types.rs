#![allow(non_snake_case, unused)]
use std::fmt::{self, Debug, Display};

#[derive(Debug)]
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

impl std::fmt::Display for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum Suits {
    Spade,
    Heart,
    Diamond,
    Club
}

impl std::fmt::Display for Suits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Card {
    value: i32,
    face: CardType,
    suit: Suits,
    is_ace: bool,
}

impl Card {
    pub fn new(value : i32, face : CardType, suit : Suits, is_ace: bool ) -> Self {
        return Self { value, face, suit, is_ace }
    }
    
    pub fn stringify(&self) -> String {
        return String::from(format!("Face: {}, Suit: {}, Value: {}", self.face.to_string(), self.suit.to_string(), self.value));
    }
}

pub struct Deck {}

impl Deck {
    pub const  KING_CLUB : Card = Card { value : 10, face : CardType::King, suit : Suits::Club, is_ace : false};
    pub const  KING_HEART : Card = Card { value : 10, face : CardType::King, suit : Suits::Heart, is_ace : false};
    pub const  KING_SPADE  : Card = Card { value : 10, face : CardType::King, suit : Suits::Spade, is_ace : false};
    pub const  KING_DIA : Card = Card { value : 10, face : CardType::King, suit : Suits::Diamond, is_ace : false};
    pub const  QUEEN_CLUB : Card = Card {value: 10, face: CardType::Queen, suit : Suits::Club, is_ace : false};
    pub const  QUEEN_SPADE : Card = Card {value: 10, face: CardType::Queen, suit : Suits::Spade, is_ace : false};
    pub const  QUEEN_HEART : Card = Card {value: 10, face: CardType::Queen, suit : Suits::Heart, is_ace : false};
    pub const  QUEEN_DIA : Card = Card {value: 10, face: CardType::Queen, suit : Suits::Diamond, is_ace : false};
    pub const  JACK_CLUB : Card = Card {value: 10, face: CardType::Jack, suit : Suits::Club, is_ace : false};
    pub const  JACK_SPADE : Card = Card {value: 10, face: CardType::Jack, suit : Suits::Spade, is_ace : false};
    pub const  JACK_HEART : Card = Card {value: 10, face: CardType::Jack, suit : Suits::Heart, is_ace : false};
    pub const  JACK_DIAM : Card = Card {value: 10, face: CardType::Jack, suit : Suits::Diamond, is_ace : false};
    pub const  TEN_DIAM : Card = Card {value: 10, face: CardType::Ten, suit : Suits::Diamond, is_ace : false};
    pub const  TEN_HEART : Card = Card {value: 10, face: CardType::Ten, suit : Suits::Heart, is_ace : false};
    pub const  TEN_SPADE : Card = Card {value: 10, face: CardType::Ten, suit : Suits::Spade, is_ace : false};
    pub const  TEN_CLUB : Card = Card {value: 10, face: CardType::Ten, suit : Suits::Club, is_ace : false};
    pub const  NINE_CLUB : Card = Card {value: 9, face: CardType::Nine, suit : Suits::Club, is_ace : false};
    pub const  NINE_HEART : Card = Card {value: 9, face: CardType::Nine, suit : Suits::Heart, is_ace : false};
    pub const  NINE_SPADE : Card = Card {value: 9, face: CardType::Nine, suit : Suits::Spade, is_ace : false};
    pub const  NINE_DIA : Card = Card {value: 9, face: CardType::Nine, suit : Suits::Diamond, is_ace : false};
    pub const  EIGHT_DIA : Card = Card {value: 8, face: CardType::Eight, suit : Suits::Diamond, is_ace : false};
    pub const  EIGHT_SPADE : Card = Card {value: 8, face: CardType::Eight, suit : Suits::Spade, is_ace : false};
    pub const  EIGHT_CLUB : Card = Card {value: 8, face: CardType::Eight, suit : Suits::Club, is_ace : false};
    pub const  EIGHT_HEART : Card = Card {value: 8, face: CardType::Eight, suit : Suits::Heart, is_ace : false};
    pub const  SEVEN_HEART : Card = Card {value: 7, face: CardType::Seven, suit : Suits::Heart, is_ace : false};
    pub const  SEVEN_SPADE : Card = Card {value: 7, face: CardType::Seven, suit : Suits::Spade, is_ace : false};
    pub const  SEVEN_CLUB : Card = Card {value: 7, face: CardType::Seven, suit : Suits::Club, is_ace : false};
    pub const  SEVEN_DIA : Card = Card {value: 7, face: CardType::Seven, suit : Suits::Diamond, is_ace : false};
    pub const  SIX_SPADE : Card = Card {value: 6, face: CardType::Six, suit : Suits::Spade, is_ace : false};
    pub const  SIX_HEART : Card = Card {value: 6, face: CardType::Six, suit : Suits::Heart, is_ace : false};
    pub const  SIX_DIA : Card = Card {value: 6, face: CardType::Six, suit : Suits::Diamond, is_ace : false};
    pub const  SIX_CLUB : Card = Card {value: 6, face: CardType::Six, suit : Suits::Club, is_ace : false};
    pub const  FIVE_CLUB : Card = Card {value: 5, face: CardType::Five, suit : Suits::Club, is_ace : false};
    pub const  FIVE_HEART : Card = Card {value: 5, face: CardType::Five, suit : Suits::Heart, is_ace : false};
    pub const  FIVE_SPADE : Card = Card {value: 5, face: CardType::Five, suit : Suits::Spade, is_ace : false};
    pub const  FIVE_DIA : Card = Card {value: 5, face: CardType::Five, suit : Suits::Diamond, is_ace : false};
    pub const  FOUR_DIA : Card = Card {value: 4, face: CardType::Four, suit : Suits::Diamond, is_ace : false};
    pub const  FOUR_SPADE : Card = Card {value: 4, face: CardType::Four, suit : Suits::Spade, is_ace : false};
    pub const  FOUR_HEART : Card = Card {value: 4, face: CardType::Four, suit : Suits::Heart, is_ace : false};
    pub const  FOUR_CLUB : Card = Card {value: 4, face: CardType::Four, suit : Suits::Club, is_ace : false};
    pub const  THREE_CLUB : Card = Card {value: 3, face: CardType::Three, suit : Suits::Club, is_ace : false};
    pub const  THREE_SPADE : Card = Card {value: 3, face: CardType::Three, suit : Suits::Spade, is_ace : false};
    pub const  THREE_HEART : Card = Card {value: 3, face: CardType::Three, suit : Suits::Heart, is_ace : false};
    pub const  THREE_DIA : Card = Card {value: 3, face: CardType::Three, suit : Suits::Diamond, is_ace : false};
    pub const  TWO_DIA : Card = Card {value: 2, face: CardType::Two, suit : Suits::Diamond, is_ace : false};
    pub const  TWO_HEART : Card = Card {value: 2, face: CardType::Two, suit : Suits::Heart, is_ace : false};
    pub const  TWO_SPADE : Card = Card {value: 2, face: CardType::Two, suit : Suits::Spade, is_ace : false};
    pub const  TWO_CLUB : Card = Card {value: 2, face: CardType::Two, suit : Suits::Club, is_ace : false};
    pub const  ACE_DIA : Card = Card {value: 1, face: CardType::Ace, suit : Suits::Diamond, is_ace : true};
    pub const  ACE_HEART : Card = Card {value: 1, face: CardType::Ace, suit : Suits::Heart, is_ace : true};
    pub const  ACE_SPADE : Card = Card {value: 1, face: CardType::Ace, suit : Suits::Spade, is_ace : true};
    pub const  ACE_CLUB : Card = Card {value: 1, face: CardType::Ace, suit : Suits::Club, is_ace : true};
}

pub struct Game {

}



