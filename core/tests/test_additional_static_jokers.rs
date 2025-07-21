// Tests for additional static jokers (Issue #90)
// Note: Runner is implemented as RunnerJoker in joker_impl.rs, not as a static joker
// This file tests 9 jokers: 5 fully implemented + 4 placeholders

use balatro_rs::card::{Card, Suit, Value};
use balatro_rs::hand::SelectHand;
use balatro_rs::joker::{GameContext, Joker, JokerId, JokerRarity};
use balatro_rs::static_joker_factory::StaticJokerFactory;

#[test]
fn test_red_card_joker() {
    let joker = StaticJokerFactory::create_red_card();
    assert_eq!(joker.id(), JokerId::RedCard);
    assert_eq!(joker.name(), "Red Card");
    assert_eq!(
        joker.description(),
        "Red cards (Hearts and Diamonds) give +3 Mult when scored"
    );
    assert_eq!(joker.rarity(), JokerRarity::Uncommon);
    assert_eq!(joker.cost(), 6);
}

#[test]
fn test_blue_joker() {
    let joker = StaticJokerFactory::create_blue_joker();
    assert_eq!(joker.id(), JokerId::BlueJoker);
    assert_eq!(joker.name(), "Blue Joker");
    assert_eq!(
        joker.description(),
        "Black cards (Clubs and Spades) give +3 Mult when scored"
    );
    assert_eq!(joker.rarity(), JokerRarity::Uncommon);
    assert_eq!(joker.cost(), 6);
}

#[test]
fn test_faceless_joker() {
    let joker = StaticJokerFactory::create_faceless_joker();
    assert_eq!(joker.id(), JokerId::FacelessJoker);
    assert_eq!(joker.name(), "Faceless Joker");
    assert_eq!(
        joker.description(),
        "Face cards (Jack, Queen, King) give +5 Mult when scored"
    );
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);
}

#[test]
fn test_square_joker() {
    let joker = StaticJokerFactory::create_square();
    assert_eq!(joker.id(), JokerId::Square);
    assert_eq!(joker.name(), "Square");
    assert_eq!(
        joker.description(),
        "Number cards (2, 3, 4, 5, 6, 7, 8, 9, 10) give +4 Chips when scored"
    );
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);
}

#[test]
fn test_walkie_joker() {
    let joker = StaticJokerFactory::create_walkie();
    assert_eq!(joker.id(), JokerId::Walkie);
    assert_eq!(joker.name(), "Walkie");
    assert_eq!(
        joker.description(),
        "+10 Chips and +4 Mult if played hand contains a Straight"
    );
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);
}

// Note: Runner is implemented as RunnerJoker in joker_impl.rs, not as a static joker

// Tests for jokers that need framework extensions
#[test]
fn test_half_joker() {
    let joker = StaticJokerFactory::create_half_joker();
    assert_eq!(joker.id(), JokerId::HalfJoker);
    assert_eq!(joker.name(), "Half Joker");
    assert_eq!(
        joker.description(),
        "+20 Mult if played hand has 4 or fewer cards"
    );
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);
}

#[test]
fn test_half_joker_behavior_with_4_cards() {
    let joker = StaticJokerFactory::create_half_joker();
    let mut context = GameContext::default();
    
    // Test with exactly 4 cards (should trigger)
    let four_card_hand = SelectHand::new(vec![
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::Queen, Suit::Diamond),
        Card::new(Value::Jack, Suit::Club),
        Card::new(Value::Ten, Suit::Spade),
    ]);
    
    let effect = joker.on_hand_played(&mut context, &four_card_hand);
    assert_eq!(effect.mult, 20, "Half Joker should provide +20 Mult with 4 cards");
    assert_eq!(effect.chips, 0, "Half Joker should not provide chips");
    assert_eq!(effect.mult_multiplier, 1.0, "Half Joker should not provide mult multiplier");
}

#[test]
fn test_half_joker_behavior_with_3_cards() {
    let joker = StaticJokerFactory::create_half_joker();
    let mut context = GameContext::default();
    
    // Test with 3 cards (should trigger)
    let three_card_hand = SelectHand::new(vec![
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::Queen, Suit::Diamond),
        Card::new(Value::Jack, Suit::Club),
    ]);
    
    let effect = joker.on_hand_played(&mut context, &three_card_hand);
    assert_eq!(effect.mult, 20, "Half Joker should provide +20 Mult with 3 cards");
}

#[test]
fn test_half_joker_behavior_with_2_cards() {
    let joker = StaticJokerFactory::create_half_joker();
    let mut context = GameContext::default();
    
    // Test with 2 cards (should trigger)
    let two_card_hand = SelectHand::new(vec![
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::Queen, Suit::Diamond),
    ]);
    
    let effect = joker.on_hand_played(&mut context, &two_card_hand);
    assert_eq!(effect.mult, 20, "Half Joker should provide +20 Mult with 2 cards");
}

#[test]
fn test_half_joker_behavior_with_1_card() {
    let joker = StaticJokerFactory::create_half_joker();
    let mut context = GameContext::default();
    
    // Test with 1 card (should trigger)
    let one_card_hand = SelectHand::new(vec![
        Card::new(Value::King, Suit::Heart),
    ]);
    
    let effect = joker.on_hand_played(&mut context, &one_card_hand);
    assert_eq!(effect.mult, 20, "Half Joker should provide +20 Mult with 1 card");
}

#[test]
fn test_half_joker_behavior_with_5_cards() {
    let joker = StaticJokerFactory::create_half_joker();
    let mut context = GameContext::default();
    
    // Test with 5 cards (should NOT trigger)
    let five_card_hand = SelectHand::new(vec![
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::Queen, Suit::Diamond),
        Card::new(Value::Jack, Suit::Club),
        Card::new(Value::Ten, Suit::Spade),
        Card::new(Value::Nine, Suit::Heart),
    ]);
    
    let effect = joker.on_hand_played(&mut context, &five_card_hand);
    assert_eq!(effect.mult, 0, "Half Joker should provide no mult with 5 cards");
    assert_eq!(effect.chips, 0, "Half Joker should provide no chips with 5 cards");
    assert_eq!(effect.mult_multiplier, 1.0, "Half Joker should provide no mult multiplier with 5 cards");
}

#[test]
fn test_half_joker_behavior_with_6_cards() {
    let joker = StaticJokerFactory::create_half_joker();
    let mut context = GameContext::default();
    
    // Test with 6 cards (should NOT trigger)
    let six_card_hand = SelectHand::new(vec![
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::Queen, Suit::Diamond),
        Card::new(Value::Jack, Suit::Club),
        Card::new(Value::Ten, Suit::Spade),
        Card::new(Value::Nine, Suit::Heart),
        Card::new(Value::Eight, Suit::Diamond),
    ]);
    
    let effect = joker.on_hand_played(&mut context, &six_card_hand);
    assert_eq!(effect.mult, 0, "Half Joker should provide no mult with 6 cards");
}

#[test]
fn test_half_joker_behavior_per_hand_not_per_card() {
    let joker = StaticJokerFactory::create_half_joker();
    let mut context = GameContext::default();
    
    // Test that Half Joker is per-hand, not per-card
    let three_card_hand = SelectHand::new(vec![
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::Queen, Suit::Diamond),
        Card::new(Value::Jack, Suit::Club),
    ]);
    
    // Test on_card_scored - should return no effect since it's per-hand
    let card = Card::new(Value::King, Suit::Heart);
    let card_effect = joker.on_card_scored(&mut context, &card);
    assert_eq!(card_effect.mult, 0, "Half Joker should not trigger on individual cards");
    
    // Test on_hand_played - should return effect since it's per-hand
    let hand_effect = joker.on_hand_played(&mut context, &three_card_hand);
    assert_eq!(hand_effect.mult, 20, "Half Joker should trigger on hands with ≤4 cards");
}

#[test]
fn test_half_joker_behavior_edge_case_empty_hand() {
    let joker = StaticJokerFactory::create_half_joker();
    let mut context = GameContext::default();
    
    // Test with empty hand (should trigger as 0 ≤ 4)
    let empty_hand = SelectHand::new(vec![]);
    
    let effect = joker.on_hand_played(&mut context, &empty_hand);
    assert_eq!(effect.mult, 20, "Half Joker should provide +20 Mult with empty hand");
}

#[test]
#[ignore] // Ignore until framework supports discard count
fn test_banner_joker() {
    let joker = StaticJokerFactory::create_banner();
    assert_eq!(joker.id(), JokerId::Banner);
    assert_eq!(joker.name(), "Banner");
    assert_eq!(joker.description(), "+30 Chips for each remaining discard");
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);
}

#[test]
#[ignore] // Ignore until framework supports joker interactions
fn test_abstract_joker() {
    let joker = StaticJokerFactory::create_abstract_joker();
    assert_eq!(joker.id(), JokerId::AbstractJoker);
    assert_eq!(joker.name(), "Abstract Joker");
    assert_eq!(joker.description(), "All Jokers give X0.25 more Mult");
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);
}

#[test]
#[ignore] // Ignore until framework supports deck composition
fn test_steel_joker() {
    let joker = StaticJokerFactory::create_steel_joker();
    assert_eq!(joker.id(), JokerId::SteelJoker);
    assert_eq!(joker.name(), "Steel Joker");
    assert_eq!(
        joker.description(),
        "This Joker gains X0.25 Mult for each Steel Card in your full deck"
    );
    assert_eq!(joker.rarity(), JokerRarity::Uncommon);
    assert_eq!(joker.cost(), 6);
}
