/// Comprehensive tests for process_joker_effects function (Issue #370)
/// These tests document the current behavior before refactoring to ensure backward compatibility
use balatro_rs::card::{Card, Suit, Value};
use balatro_rs::game::Game;
use balatro_rs::hand::SelectHand;
use balatro_rs::joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity};
use balatro_rs::stage::{Blind, Stage};

#[test]
fn test_empty_joker_list() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Ensure no jokers
    game.jokers.clear();

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, messages) = game.process_joker_effects(&hand);

    // With no jokers, should return zero values
    assert_eq!(chips, 0);
    assert_eq!(mult, 0);
    assert_eq!(money, 0);
    assert_eq!(messages.len(), 0);
}

#[test]
fn test_single_hand_level_joker() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add joker that only affects hand-level
    let joker = Box::new(TestHandLevelJoker::new(10, 5, 2, 1.5));
    game.jokers = vec![joker];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, messages) = game.process_joker_effects(&hand);

    // Should include hand-level effects
    // Note: mult gets multiplied by mult_multiplier: 5 * 1.5 = 7.5 -> 7 (as i32)
    assert_eq!(chips, 10);
    assert_eq!(mult, 7); // 5 * 1.5 = 7.5, cast to i32 = 7
    assert_eq!(money, 2);
    
    // Debug messages are generated in debug builds
    #[cfg(debug_assertions)]
    {
        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("Test Hand Level Joker"));
        assert!(messages[0].contains("+10 chips"));
        assert!(messages[0].contains("+5 mult"));
        assert!(messages[0].contains("+2 money"));
    }
    
    #[cfg(not(debug_assertions))]
    {
        assert_eq!(messages.len(), 0);
    }
}

#[test]
fn test_single_card_level_joker() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add joker that only affects card-level
    let joker = Box::new(TestCardLevelJoker::new(5, 3, 1));
    game.jokers = vec![joker];

    // Create a pair so both cards are included in the made hand
    let cards = vec![
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::Ace, Suit::Spade),
    ];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, messages) = game.process_joker_effects(&hand);

    // Should include effects from both cards (2 cards * joker effects)
    assert_eq!(chips, 10); // 5 chips * 2 cards
    assert_eq!(mult, 6); // 3 mult * 2 cards (no mult_multiplier applied)
    assert_eq!(money, 2); // 1 money * 2 cards
    
    // Debug messages are generated in debug builds (one per card)
    #[cfg(debug_assertions)]
    {
        assert_eq!(messages.len(), 2); // One message per card
        for message in &messages {
            assert!(message.contains("Test Card Level Joker"));
        }
    }
    
    #[cfg(not(debug_assertions))]
    {
        assert_eq!(messages.len(), 0);
    }
}

#[test]
fn test_mixed_hand_and_card_level_jokers() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add both hand-level and card-level jokers
    let hand_joker = Box::new(TestHandLevelJoker::new(10, 5, 1, 1.0));
    let card_joker = Box::new(TestCardLevelJoker::new(3, 2, 0));
    game.jokers = vec![hand_joker, card_joker];

    // Create a pair so both cards are included in the made hand
    let cards = vec![
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::Ace, Suit::Spade),
    ];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, messages) = game.process_joker_effects(&hand);

    // Should include both hand-level and card-level effects
    assert_eq!(chips, 16); // 10 (hand) + 6 (cards: 3*2)
    assert_eq!(mult, 9); // 5 (hand) + 4 (cards: 2*2), no mult_multiplier since hand joker has 1.0
    assert_eq!(money, 1); // 1 (hand) + 0 (cards)
    
    // Debug messages are generated in debug builds
    #[cfg(debug_assertions)]
    {
        assert_eq!(messages.len(), 3); // 1 hand message + 2 card messages
    }
    
    #[cfg(not(debug_assertions))]
    {
        assert_eq!(messages.len(), 0);
    }
}

#[test]
fn test_retrigger_effects() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add joker with retrigger effects
    let joker = Box::new(TestRetriggerJoker::new(10, 5, 0, 2)); // 2 retriggers = 3 total triggers
    game.jokers = vec![joker];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, messages) = game.process_joker_effects(&hand);

    // Should apply effects 3 times (1 + 2 retriggers)
    assert_eq!(chips, 30); // 10 * 3
    assert_eq!(mult, 15); // 5 * 3, no mult_multiplier applied since TestRetriggerJoker doesn't set one
    assert_eq!(money, 0); // 0 * 3
    
    // Debug messages are generated in debug builds
    #[cfg(debug_assertions)]
    {
        assert_eq!(messages.len(), 1); // One message per joker (even with retriggers)
        assert!(messages[0].contains("Test Retrigger Joker"));
        assert!(messages[0].contains("+30 chips")); // Shows total effect including retriggers
        assert!(messages[0].contains("retrigger x2"));
    }
    
    #[cfg(not(debug_assertions))]
    {
        assert_eq!(messages.len(), 0);
    }
}

#[test]
fn test_mult_multiplier_accumulation() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add jokers with mult multipliers
    let joker1 = Box::new(TestHandLevelJoker::new(0, 10, 0, 2.0));
    let joker2 = Box::new(TestHandLevelJoker::new(0, 5, 0, 1.5));
    game.jokers = vec![joker1, joker2];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, messages) = game.process_joker_effects(&hand);

    // Base mult: 10 + 5 = 15
    // Multipliers: 2.0 * 1.5 = 3.0
    // Final mult: 15 * 3.0 = 45
    assert_eq!(chips, 0);
    assert_eq!(mult, 45);
    assert_eq!(money, 0);
    
    // Debug messages are generated in debug builds
    #[cfg(debug_assertions)]
    {
        assert_eq!(messages.len(), 2); // Two jokers, two messages
    }
    
    #[cfg(not(debug_assertions))]
    {
        assert_eq!(messages.len(), 0);
    }
}

#[test]
fn test_killscreen_detection() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add jokers with extreme multipliers that cause overflow
    let joker1 = Box::new(TestHandLevelJoker::new(0, 10, 0, 1e200));
    let joker2 = Box::new(TestHandLevelJoker::new(0, 5, 0, 1e200));
    game.jokers = vec![joker1, joker2];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, messages) = game.process_joker_effects(&hand);

    // Should detect killscreen - but current implementation still applies infinite multiplier
    assert_eq!(chips, 0); // Both jokers have 0 chips
    assert_eq!(mult, i32::MAX); // Killscreen detected but final multiplication still applied, resulting in i32::MAX
    assert_eq!(money, 0); // Both jokers have 0 money
    
    // In debug builds, should have killscreen message
    #[cfg(debug_assertions)]
    {
        let has_killscreen_msg = messages.iter().any(|msg| msg.contains("KILLSCREEN"));
        assert!(
            has_killscreen_msg,
            "Should have killscreen message in debug build"
        );
    }
}

#[test]
fn test_debug_message_generation() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add joker that generates debug messages
    let joker = Box::new(TestDebugJoker::new(5, 3, 0));
    game.jokers = vec![joker];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, messages) = game.process_joker_effects(&hand);

    assert_eq!(chips, 5);
    assert_eq!(mult, 3);
    assert_eq!(money, 0);

    // Debug messages only generated in debug builds
    #[cfg(debug_assertions)]
    {
        assert!(
            messages.len() > 0,
            "Should have debug messages in debug build"
        );
        assert!(messages.iter().any(|msg| msg.contains("TestDebugJoker")));
    }

    #[cfg(not(debug_assertions))]
    {
        assert_eq!(messages.len(), 0, "No debug messages in release build");
    }
}

#[test]
fn test_zero_mult_multiplier_edge_case() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add joker with 0.0 mult multiplier
    let joker = Box::new(TestHandLevelJoker::new(0, 10, 0, 0.0));
    game.jokers = vec![joker];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, messages) = game.process_joker_effects(&hand);

    // When mult_multiplier is 0.0, it's treated as "no multiplier" (ignored)
    assert_eq!(chips, 0);
    assert_eq!(mult, 10); // mult_multiplier of 0.0 is ignored, so no multiplication applied
    assert_eq!(money, 0);
    
    // Debug messages are generated in debug builds
    #[cfg(debug_assertions)]
    {
        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("Test Hand Level Joker"));
    }
    
    #[cfg(not(debug_assertions))]
    {
        assert_eq!(messages.len(), 0);
    }
}

#[test]
fn test_large_number_handling() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add joker with large but finite values
    let joker = Box::new(TestHandLevelJoker::new(
        i32::MAX / 2,
        i32::MAX / 2,
        i32::MAX / 2,
        1.0,
    ));
    game.jokers = vec![joker];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, messages) = game.process_joker_effects(&hand);

    // Should handle large numbers without overflow
    assert_eq!(chips, i32::MAX / 2);
    assert_eq!(mult, i32::MAX / 2); // No mult_multiplier, so no change
    assert_eq!(money, i32::MAX / 2);
    
    // Debug messages are generated in debug builds
    #[cfg(debug_assertions)]
    {
        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("Test Hand Level Joker"));
    }
    
    #[cfg(not(debug_assertions))]
    {
        assert_eq!(messages.len(), 0);
    }
}

#[test]
fn test_multiple_jokers_with_retriggers() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add multiple jokers with different retrigger counts
    let joker1 = Box::new(TestRetriggerJoker::new(10, 5, 1, 1)); // 1 retrigger = 2 total
    let joker2 = Box::new(TestRetriggerJoker::new(5, 3, 0, 2)); // 2 retriggers = 3 total
    game.jokers = vec![joker1, joker2];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, messages) = game.process_joker_effects(&hand);

    // Joker1: (10 chips + 5 mult + 1 money) * 2 triggers
    // Joker2: (5 chips + 3 mult + 0 money) * 3 triggers
    assert_eq!(chips, 35); // (10 * 2) + (5 * 3) = 20 + 15
    assert_eq!(mult, 19); // (5 * 2) + (3 * 3) = 10 + 9, no mult_multipliers
    assert_eq!(money, 2); // (1 * 2) + (0 * 3) = 2 + 0
    
    // Debug messages are generated in debug builds
    #[cfg(debug_assertions)]
    {
        assert_eq!(messages.len(), 2); // Two jokers, two messages
        assert!(messages.iter().any(|msg| msg.contains("Test Retrigger Joker")));
    }
    
    #[cfg(not(debug_assertions))]
    {
        assert_eq!(messages.len(), 0);
    }
}

#[test]
fn test_joker_evaluation_order() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add jokers in specific order that affects mult multipliers
    let joker1 = Box::new(TestHandLevelJoker::new(0, 10, 0, 1.0)); // Adds base mult
    let joker2 = Box::new(TestHandLevelJoker::new(0, 0, 0, 2.0)); // Multiplies existing mult
    game.jokers = vec![joker1, joker2];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, messages) = game.process_joker_effects(&hand);

    // First joker adds 10 mult, second joker multiplies by 2.0
    // Final mult: 10 * 2.0 = 20
    assert_eq!(chips, 0);
    assert_eq!(mult, 20);
    assert_eq!(money, 0);
    
    // Debug messages are generated in debug builds
    #[cfg(debug_assertions)]
    {
        assert_eq!(messages.len(), 1); // Only first joker generates message (second has all zero effects)
        assert!(messages[0].contains("Test Hand Level Joker"));
        assert!(messages[0].contains("+10"));
    }
    
    #[cfg(not(debug_assertions))]
    {
        assert_eq!(messages.len(), 0);
    }
}

// Test helper jokers for comprehensive testing

#[derive(Debug)]
struct TestHandLevelJoker {
    chips: i32,
    mult: i32,
    money: i32,
    mult_multiplier: f64,
}

impl TestHandLevelJoker {
    fn new(chips: i32, mult: i32, money: i32, mult_multiplier: f64) -> Self {
        Self {
            chips,
            mult,
            money,
            mult_multiplier,
        }
    }
}

impl Joker for TestHandLevelJoker {
    fn id(&self) -> JokerId {
        JokerId::Joker
    }
    fn name(&self) -> &str {
        "Test Hand Level Joker"
    }
    fn description(&self) -> &str {
        "Test joker for hand-level effects"
    }
    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        JokerEffect::new()
            .with_chips(self.chips)
            .with_mult(self.mult)
            .with_money(self.money)
            .with_mult_multiplier(self.mult_multiplier)
    }
}

#[derive(Debug)]
struct TestCardLevelJoker {
    chips_per_card: i32,
    mult_per_card: i32,
    money_per_card: i32,
}

impl TestCardLevelJoker {
    fn new(chips_per_card: i32, mult_per_card: i32, money_per_card: i32) -> Self {
        Self {
            chips_per_card,
            mult_per_card,
            money_per_card,
        }
    }
}

impl Joker for TestCardLevelJoker {
    fn id(&self) -> JokerId {
        JokerId::Joker
    }
    fn name(&self) -> &str {
        "Test Card Level Joker"
    }
    fn description(&self) -> &str {
        "Test joker for card-level effects"
    }
    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn on_card_scored(&self, _context: &mut GameContext, _card: &Card) -> JokerEffect {
        JokerEffect::new()
            .with_chips(self.chips_per_card)
            .with_mult(self.mult_per_card)
            .with_money(self.money_per_card)
    }
}

#[derive(Debug)]
struct TestRetriggerJoker {
    chips: i32,
    mult: i32,
    money: i32,
    retrigger_count: u32,
}

impl TestRetriggerJoker {
    fn new(chips: i32, mult: i32, money: i32, retrigger_count: u32) -> Self {
        Self {
            chips,
            mult,
            money,
            retrigger_count,
        }
    }
}

impl Joker for TestRetriggerJoker {
    fn id(&self) -> JokerId {
        JokerId::Joker
    }
    fn name(&self) -> &str {
        "Test Retrigger Joker"
    }
    fn description(&self) -> &str {
        "Test joker for retrigger mechanics"
    }
    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        JokerEffect::new()
            .with_chips(self.chips)
            .with_mult(self.mult)
            .with_money(self.money)
            .with_retrigger(self.retrigger_count)
    }
}

#[derive(Debug)]
struct TestDebugJoker {
    chips: i32,
    mult: i32,
    money: i32,
}

impl TestDebugJoker {
    fn new(chips: i32, mult: i32, money: i32) -> Self {
        Self { chips, mult, money }
    }
}

impl Joker for TestDebugJoker {
    fn id(&self) -> JokerId {
        JokerId::Joker
    }
    fn name(&self) -> &str {
        "TestDebugJoker"
    }
    fn description(&self) -> &str {
        "Test joker for debug message generation"
    }
    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        JokerEffect::new()
            .with_chips(self.chips)
            .with_mult(self.mult)
            .with_money(self.money)
    }
}
