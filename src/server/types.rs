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

struct Deck {}

impl Deck {
    const  king_club : Card = Card { value : 10, face : CardType::King, suit : Suits::Club, is_ace : false};
    const  king_heart : Card = Card { value : 10, face : CardType::King, suit : Suits::Heart, is_ace : false};
    const  king_spade : Card = Card { value : 10, face : CardType::King, suit : Suits::Spade, is_ace : false};
    const  king_diam : Card = Card { value : 10, face : CardType::King, suit : Suits::Diamond, is_ace : false};
    const  queen_club : Card = Card {value: 10, face: CardType::Queen, suit : Suits::Club, is_ace : false};
    const  queen_spade : Card = Card {value: 10, face: CardType::Queen, suit : Suits::Spade, is_ace : false};
    const  queen_heart : Card = Card {value: 10, face: CardType::Queen, suit : Suits::Heart, is_ace : false};
    const  queen_diam : Card = Card {value: 10, face: CardType::Queen, suit : Suits::Diamond, is_ace : false};
    const  jack_club : Card = Card {value: 10, face: CardType::Jack, suit : Suits::Club, is_ace : false};
    const  jack_spade : Card = Card {value: 10, face: CardType::Jack, suit : Suits::Spade, is_ace : false};
    const  jack_heart : Card = Card {value: 10, face: CardType::Jack, suit : Suits::Heart, is_ace : false};
    const  jack_diam : Card = Card {value: 10, face: CardType::Jack, suit : Suits::Diamond, is_ace : false};
    const  ten_diam : Card = Card {value: 10, face: CardType::Ten, suit : Suits::Diamond, is_ace : false};
    const  ten_heart : Card = Card {value: 10, face: CardType::Ten, suit : Suits::Heart, is_ace : false};
    const  ten_spade : Card = Card {value: 10, face: CardType::Ten, suit : Suits::Spade, is_ace : false};
    const  ten_club : Card = Card {value: 10, face: CardType::Ten, suit : Suits::Club, is_ace : false};
    const  nine_club : Card = Card {value: 9, face: CardType::Nine, suit : Suits::Club, is_ace : false};
    const  nine_heart : Card = Card {value: 9, face: CardType::Nine, suit : Suits::Heart, is_ace : false};
    const  nine_spade : Card = Card {value: 9, face: CardType::Nine, suit : Suits::Spade, is_ace : false};
    const  nine_diam : Card = Card {value: 9, face: CardType::Nine, suit : Suits::Diamond, is_ace : false};
    const  eight_diam : Card = Card {value: 8, face: CardType::Eight, suit : Suits::Diamond, is_ace : false};
    const  eight_spade : Card = Card {value: 8, face: CardType::Eight, suit : Suits::Spade, is_ace : false};
    const  eight_club : Card = Card {value: 8, face: CardType::Eight, suit : Suits::Club, is_ace : false};
    const  eight_heart : Card = Card {value: 8, face: CardType::Eight, suit : Suits::Heart, is_ace : false};
    const  seven_heart : Card = Card {value: 7, face: CardType::Seven, suit : Suits::Heart, is_ace : false};
    const  seven_spade : Card = Card {value: 7, face: CardType::Seven, suit : Suits::Spade, is_ace : false};
    const  seven_club : Card = Card {value: 7, face: CardType::Seven, suit : Suits::Club, is_ace : false};
    const  seven_diam : Card = Card {value: 7, face: CardType::Seven, suit : Suits::Diamond, is_ace : false};
    const  six_spade : Card = Card {value: 6, face: CardType::Six, suit : Suits::Spade, is_ace : false};
    const  six_heart : Card = Card {value: 6, face: CardType::Six, suit : Suits::Heart, is_ace : false};
    const  six_diam : Card = Card {value: 6, face: CardType::Six, suit : Suits::Diamond, is_ace : false};
    const  six_club : Card = Card {value: 6, face: CardType::Six, suit : Suits::Club, is_ace : false};
    const  five_club : Card = Card {value: 5, face: CardType::Five, suit : Suits::Club, is_ace : false};
    const  five_heart : Card = Card {value: 5, face: CardType::Five, suit : Suits::Heart, is_ace : false};
    const  five_spade : Card = Card {value: 5, face: CardType::Five, suit : Suits::Spade, is_ace : false};
    const  five_diam : Card = Card {value: 5, face: CardType::Five, suit : Suits::Diamond, is_ace : false};
    const  four_diam : Card = Card {value: 4, face: CardType::Four, suit : Suits::Diamond, is_ace : false};
    const  four_spade : Card = Card {value: 4, face: CardType::Four, suit : Suits::Spade, is_ace : false};
    const  four_heart : Card = Card {value: 4, face: CardType::Four, suit : Suits::Heart, is_ace : false};
    const  four_club : Card = Card {value: 4, face: CardType::Four, suit : Suits::Club, is_ace : false};
    const  three_club : Card = Card {value: 3, face: CardType::Three, suit : Suits::Club, is_ace : false};
    const  three_spade : Card = Card {value: 3, face: CardType::Three, suit : Suits::Spade, is_ace : false};
    const  three_heart : Card = Card {value: 3, face: CardType::Three, suit : Suits::Heart, is_ace : false};
    const  three_diamond : Card = Card {value: 3, face: CardType::Three, suit : Suits::Diamond, is_ace : false};
    const  two_diamond : Card = Card {value: 2, face: CardType::Two, suit : Suits::Diamond, is_ace : false};
    const  two_heart : Card = Card {value: 2, face: CardType::Two, suit : Suits::Heart, is_ace : false};
    const  two_spade : Card = Card {value: 2, face: CardType::Two, suit : Suits::Spade, is_ace : false};
    const  two_club : Card = Card {value: 2, face: CardType::Two, suit : Suits::Club, is_ace : false};
    const  ace_diamond : Card = Card {value: 1, face: CardType::Ace, suit : Suits::Diamond, is_ace : true};
    const  ace_heart : Card = Card {value: 1, face: CardType::Ace, suit : Suits::Heart, is_ace : true};
    const  ace_spade : Card = Card {value: 1, face: CardType::Ace, suit : Suits::Spade, is_ace : true};
    const  ace_club : Card = Card {value: 1, face: CardType::Ace, suit : Suits::Club, is_ace : true};
}

pub struct Game {   
    
}


