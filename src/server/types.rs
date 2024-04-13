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
    const  KING_CLUB : Card = Card { value : 10, face : CardType::King, suit : Suits::Club, is_ace : false};
    const  KING_HEART : Card = Card { value : 10, face : CardType::King, suit : Suits::Heart, is_ace : false};
    const  KING_SPADE  : Card = Card { value : 10, face : CardType::King, suit : Suits::Spade, is_ace : false};
    const  KING_DIA : Card = Card { value : 10, face : CardType::King, suit : Suits::Diamond, is_ace : false};
    const  QUEEN_CLUB : Card = Card {value: 10, face: CardType::Queen, suit : Suits::Club, is_ace : false};
    const  QUEEN_SPADE : Card = Card {value: 10, face: CardType::Queen, suit : Suits::Spade, is_ace : false};
    const  QUEEN_HEART : Card = Card {value: 10, face: CardType::Queen, suit : Suits::Heart, is_ace : false};
    const  QUEEN_DIA : Card = Card {value: 10, face: CardType::Queen, suit : Suits::Diamond, is_ace : false};
    const  JACK_CLUB : Card = Card {value: 10, face: CardType::Jack, suit : Suits::Club, is_ace : false};
    const  JACK_SPADE : Card = Card {value: 10, face: CardType::Jack, suit : Suits::Spade, is_ace : false};
    const  JACK_HEART : Card = Card {value: 10, face: CardType::Jack, suit : Suits::Heart, is_ace : false};
    const  JACK_DIAM : Card = Card {value: 10, face: CardType::Jack, suit : Suits::Diamond, is_ace : false};
    const  TEN_DIAM : Card = Card {value: 10, face: CardType::Ten, suit : Suits::Diamond, is_ace : false};
    const  TEN_HEART : Card = Card {value: 10, face: CardType::Ten, suit : Suits::Heart, is_ace : false};
    const  TEN_SPADE : Card = Card {value: 10, face: CardType::Ten, suit : Suits::Spade, is_ace : false};
    const  TEN_CLUB : Card = Card {value: 10, face: CardType::Ten, suit : Suits::Club, is_ace : false};
    const  NINE_CLUB : Card = Card {value: 9, face: CardType::Nine, suit : Suits::Club, is_ace : false};
    const  NINE_HEART : Card = Card {value: 9, face: CardType::Nine, suit : Suits::Heart, is_ace : false};
    const  NINE_SPADE : Card = Card {value: 9, face: CardType::Nine, suit : Suits::Spade, is_ace : false};
    const  NINE_DIA : Card = Card {value: 9, face: CardType::Nine, suit : Suits::Diamond, is_ace : false};
    const  EIGHT_DIA : Card = Card {value: 8, face: CardType::Eight, suit : Suits::Diamond, is_ace : false};
    const  EIGHT_SPADE : Card = Card {value: 8, face: CardType::Eight, suit : Suits::Spade, is_ace : false};
    const  EIGHT_CLUB : Card = Card {value: 8, face: CardType::Eight, suit : Suits::Club, is_ace : false};
    const  EIGHT_HEART : Card = Card {value: 8, face: CardType::Eight, suit : Suits::Heart, is_ace : false};
    const  SEVEN_HEART : Card = Card {value: 7, face: CardType::Seven, suit : Suits::Heart, is_ace : false};
    const  SEVEN_SPADE : Card = Card {value: 7, face: CardType::Seven, suit : Suits::Spade, is_ace : false};
    const  SEVEN_CLUB : Card = Card {value: 7, face: CardType::Seven, suit : Suits::Club, is_ace : false};
    const  SEVEN_DIA : Card = Card {value: 7, face: CardType::Seven, suit : Suits::Diamond, is_ace : false};
    const  SIX_SPADE : Card = Card {value: 6, face: CardType::Six, suit : Suits::Spade, is_ace : false};
    const  SIX_HEART : Card = Card {value: 6, face: CardType::Six, suit : Suits::Heart, is_ace : false};
    const  SIX_DIA : Card = Card {value: 6, face: CardType::Six, suit : Suits::Diamond, is_ace : false};
    const  SIX_CLUB : Card = Card {value: 6, face: CardType::Six, suit : Suits::Club, is_ace : false};
    const  FIVE_CLUB : Card = Card {value: 5, face: CardType::Five, suit : Suits::Club, is_ace : false};
    const  FIVE_HEART : Card = Card {value: 5, face: CardType::Five, suit : Suits::Heart, is_ace : false};
    const  FIVE_SPADE : Card = Card {value: 5, face: CardType::Five, suit : Suits::Spade, is_ace : false};
    const  FIVE_DIA : Card = Card {value: 5, face: CardType::Five, suit : Suits::Diamond, is_ace : false};
    const  FOUR_DIA : Card = Card {value: 4, face: CardType::Four, suit : Suits::Diamond, is_ace : false};
    const  FOUR_SPADE : Card = Card {value: 4, face: CardType::Four, suit : Suits::Spade, is_ace : false};
    const  FOUR_HEART : Card = Card {value: 4, face: CardType::Four, suit : Suits::Heart, is_ace : false};
    const  FOUR_CLUB : Card = Card {value: 4, face: CardType::Four, suit : Suits::Club, is_ace : false};
    const  THREE_CLUB : Card = Card {value: 3, face: CardType::Three, suit : Suits::Club, is_ace : false};
    const  THREE_SPADE : Card = Card {value: 3, face: CardType::Three, suit : Suits::Spade, is_ace : false};
    const  THREE_HEART : Card = Card {value: 3, face: CardType::Three, suit : Suits::Heart, is_ace : false};
    const  THREE_DIA : Card = Card {value: 3, face: CardType::Three, suit : Suits::Diamond, is_ace : false};
    const  TWO_DIA : Card = Card {value: 2, face: CardType::Two, suit : Suits::Diamond, is_ace : false};
    const  TWO_HEART : Card = Card {value: 2, face: CardType::Two, suit : Suits::Heart, is_ace : false};
    const  TWO_SPADE : Card = Card {value: 2, face: CardType::Two, suit : Suits::Spade, is_ace : false};
    const  TWO_CLUB : Card = Card {value: 2, face: CardType::Two, suit : Suits::Club, is_ace : false};
    const  ACE_DIA : Card = Card {value: 1, face: CardType::Ace, suit : Suits::Diamond, is_ace : true};
    const  ACE_HEART : Card = Card {value: 1, face: CardType::Ace, suit : Suits::Heart, is_ace : true};
    const  ACE_SPADE : Card = Card {value: 1, face: CardType::Ace, suit : Suits::Spade, is_ace : true};
    const  ACE_CLUB : Card = Card {value: 1, face: CardType::Ace, suit : Suits::Club, is_ace : true};
}

pub struct Game {   

}


