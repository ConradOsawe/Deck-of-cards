use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>, //vectors are like arrays but not fixed

}

impl Deck {
    fn new() -> Self {

// Lists of 'suits' - 'hearts', 'spades'
let suits = ["Hearts", "Spades", "Diamonds"];
// List of 'values' - 'ace', 'two', 'three'
let values = ["Ace", "Two", "Three"];

let mut cards = vec![];

//double nested for loop

for suit in suits {
    for value in values {
        let card = format!("{value} of {suit}"); //format macro is used to join together strings
        cards.push(card);
        }
}
//    let deck = Deck { cards };  //vec![] is an empty vector; vec::new()
//    return deck;
Deck { cards}
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(
            self.cards.len() - num_cards)

    }
}

fn main () {
let mut deck = Deck::new();

deck.shuffle();
let cards = deck.deal(3);
//Error handling

   println!("Heres your hand: {:#?}", cards);
   println!("Heres your deck: {:#?}", deck);

}