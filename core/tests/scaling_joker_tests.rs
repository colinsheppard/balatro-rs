use balatro_rs::scaling_joker::*;
use balatro_rs::scaling_joker_impl::*;
use balatro_rs::scaling_joker_custom::*;
use balatro_rs::joker::{Joker, JokerId, JokerRarity, JokerEffect, GameContext};
use balatro_rs::joker_state::{JokerState, JokerStateManager};
use balatro_rs::rank::HandRank;
use balatro_rs::hand::SelectHand;
use balatro_rs::card::{Card, Suit, Value};
use balatro_rs::stage::Stage;
use std::sync::Arc;
use std::collections::HashMap;

/// Helper function to create a basic test context
fn create_test_context(money: i32, ante: u8, round: u32) -> GameContext<'static> {
    let state_manager = Arc::new(JokerStateManager::new());
    let jokers: Vec<Box<dyn Joker>> = vec![];
    let hand = SelectHand::default();
    let discarded: Vec<Card> = vec![];
    let hand_type_counts = HashMap::new();
    let stage = Stage::Blind; // Default stage
    let rng = &balatro_rs::rng::GameRng::for_testing(42);

    GameContext {
        chips: 0,
        mult: 1,
        money,
        ante,
        round,
        stage: &stage,
        hands_played: 0,
        discards_used: 0,
        jokers: &jokers,
        hand: &hand,
        discarded: &discarded,
        joker_state_manager: &state_manager,
        hand_type_counts: &hand_type_counts,
        cards_in_deck: 52,
        stone_cards_in_deck: 0,
        rng,
    }
}

/// Enhanced test harness for scaling joker integration tests
struct ScalingJokerTestHarness {
    state_manager: Arc<JokerStateManager>,
    jokers: Vec<ScalingJoker>,
    stage: Stage,
    rng: balatro_rs::rng::GameRng,
}

impl ScalingJokerTestHarness {
    fn new() -> Self {
        Self {
            state_manager: Arc::new(JokerStateManager::new()),
            jokers: vec![],
            stage: Stage::Blind,
            rng: balatro_rs::rng::GameRng::for_testing(42),
        }
    }

    fn add_joker(&mut self, joker: ScalingJoker) {
        // Initialize joker state
        let initial_state = joker.initialize_state(&self.create_context());
        self.state_manager.update_state(joker.id(), |state| {
            *state = initial_state;
        });
        self.jokers.push(joker);
    }

    fn create_context(&self) -> GameContext<'_> {
        let jokers: Vec<Box<dyn Joker>> = vec![];
        let hand = SelectHand::default();
        let discarded: Vec<Card> = vec![];
        let hand_type_counts = HashMap::new();

        GameContext {
            chips: 0,
            mult: 1,
            money: 100,
            ante: 1,
            round: 1,
            stage: &self.stage,
            hands_played: 0,
            discards_used: 0,
            jokers: &jokers,
            hand: &hand,
            discarded: &discarded,
            joker_state_manager: &self.state_manager,
            hand_type_counts: &hand_type_counts,
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
            rng: &self.rng,
        }
    }

    fn create_mutable_context(&mut self) -> GameContext<'_> {
        let jokers: Vec<Box<dyn Joker>> = vec![];
        let hand = SelectHand::default();
        let discarded: Vec<Card> = vec![];
        let hand_type_counts = HashMap::new();

        GameContext {
            chips: 0,
            mult: 1,
            money: 100,
            ante: 1,
            round: 1,
            stage: &self.stage,
            hands_played: 0,
            discards_used: 0,
            jokers: &jokers,
            hand: &hand,
            discarded: &discarded,
            joker_state_manager: &self.state_manager,
            hand_type_counts: &hand_type_counts,
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
            rng: &self.rng,
        }
    }

    /// Simulate playing a hand with specific rank
    fn simulate_hand_played(&mut self, hand_rank: HandRank) -> Vec<JokerEffect> {
        let mut context = self.create_mutable_context();
        let hand = create_test_hand(hand_rank);
        let mut effects = vec![];

        for joker in &self.jokers {
            let effect = joker.on_hand_played(&mut context, &hand);
            effects.push(effect);
        }

        effects
    }

    /// Simulate discarding cards
    fn simulate_cards_discarded(&mut self, count: usize) -> Vec<JokerEffect> {
        let mut context = self.create_mutable_context();
        let cards = vec![Card::new(Value::Two, Suit::Heart); count];
        let mut effects = vec![];

        for joker in &self.jokers {
            for _ in 0..count {
                let effect = joker.on_discard(&mut context, &cards);
                effects.push(effect);
            }
        }

        effects
    }

    /// Simulate round end
    fn simulate_round_end(&mut self) -> Vec<JokerEffect> {
        let mut context = self.create_mutable_context();
        let mut effects = vec![];

        for joker in &self.jokers {
            let effect = joker.on_round_end(&mut context);
            effects.push(effect);
        }

        effects
    }

    /// Simulate shop opening
    fn simulate_shop_open(&mut self) -> Vec<JokerEffect> {
        let mut context = self.create_mutable_context();
        let mut effects = vec![];

        for joker in &self.jokers {
            let effect = joker.on_shop_open(&mut context);
            effects.push(effect);
        }

        effects
    }

    /// Process a scaling event directly
    fn process_scaling_event(&mut self, event: ScalingEvent) {
        let mut context = self.create_mutable_context();
        
        for joker in &self.jokers {
            joker.process_event(&mut context, &event);
        }
    }

    /// Get current accumulated value for a joker
    fn get_accumulated_value(&self, joker_id: JokerId) -> f64 {
        self.state_manager
            .get_accumulated_value(joker_id)
            .unwrap_or(0.0)
    }

    /// Get current effect for a joker
    fn get_current_effect(&self, joker_id: JokerId) -> JokerEffect {
        let context = self.create_context();
        
        if let Some(joker) = self.jokers.iter().find(|j| j.id() == joker_id) {
            joker.calculate_effect(&context)
        } else {
            JokerEffect::new()
        }
    }
}

/// Create a test hand with specific hand rank
fn create_test_hand(rank: HandRank) -> SelectHand {
    let cards = match rank {
        HandRank::OnePair => vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::Ace, Suit::Spade),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::Queen, Suit::Club),
            Card::new(Value::Jack, Suit::Heart),
        ],
        HandRank::TwoPair => vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::Ace, Suit::Spade),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::Queen, Suit::Heart),
        ],
        HandRank::Flush => vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Nine, Suit::Heart),
        ],
        _ => vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Club),
            Card::new(Value::Nine, Suit::Heart),
        ],
    };
    SelectHand::new(cards)
}

#[test]
fn test_scaling_joker_framework() {
    let joker = ScalingJoker::new(
        JokerId::Reserved,
        "Test Joker".to_string(),
        "Test Description".to_string(),
        JokerRarity::Common,
        0.0,
        5.0,
        ScalingTrigger::HandPlayed(HandRank::OnePair),
        ScalingEffectType::Mult,
    );

    assert_eq!(joker.id(), JokerId::Reserved);
    assert_eq!(joker.name(), "Test Joker");
    assert_eq!(joker.rarity(), JokerRarity::Common);
}

#[test]
fn test_scaling_triggers() {
    assert_eq!(
        format!("{}", ScalingTrigger::HandPlayed(HandRank::OnePair)),
        "Pair played"
    );
    assert_eq!(
        format!("{}", ScalingTrigger::CardDiscarded),
        "card discarded"
    );
    assert_eq!(
        format!("{}", ScalingTrigger::MoneyGained),
        "money gained"
    );
}

#[test]
fn test_reset_conditions() {
    assert_eq!(
        format!("{}", ResetCondition::RoundEnd),
        "reset at round end"
    );
    assert_eq!(
        format!("{}", ResetCondition::Never),
        "never resets"
    );
}

#[test] 
fn test_spare_trousers() {
    let joker = create_spare_trousers();
    assert_eq!(joker.id, JokerId::Trousers);
    assert_eq!(joker.name, "Spare Trousers");
    assert_eq!(joker.trigger, ScalingTrigger::HandPlayed(HandRank::TwoPair));
    assert_eq!(joker.increment, 2.0);
    assert_eq!(joker.effect_type, ScalingEffectType::Mult);
}

#[test]
fn test_ceremonial_dagger() {
    let joker = create_ceremonial_dagger();
    assert_eq!(joker.base_value, 1.0);
    assert_eq!(joker.effect_type, ScalingEffectType::MultMultiplier);
    assert_eq!(joker.reset_condition, Some(ResetCondition::RoundEnd));
}

#[test]
fn test_all_15_scaling_jokers() {
    let jokers = create_all_scaling_jokers();
    assert_eq!(jokers.len(), 15, "Should create exactly 15 scaling jokers");
    
    // Test that all jokers have unique IDs
    let mut ids = std::collections::HashSet::new();
    for joker in &jokers {
        assert!(ids.insert(joker.id), "Duplicate joker ID found: {:?}", joker.id);
    }
    
    // Test that all jokers have non-empty names and descriptions
    for joker in &jokers {
        assert!(!joker.name.is_empty(), "Joker {:?} has empty name", joker.id);
        assert!(!joker.description.is_empty(), "Joker {:?} has empty description", joker.id);
    }
}

#[test]
fn test_scaling_effect_types() {
    let chips_joker = ScalingJoker::new(
        JokerId::Reserved,
        "Test".to_string(),
        "Test".to_string(),
        JokerRarity::Common,
        0.0,
        10.0,
        ScalingTrigger::CardDiscarded,
        ScalingEffectType::Chips,
    );

    let mult_joker = ScalingJoker::new(
        JokerId::Reserved2,
        "Test".to_string(),
        "Test".to_string(),
        JokerRarity::Common,
        0.0,
        5.0,
        ScalingTrigger::MoneyGained,
        ScalingEffectType::Mult,
    );

    let multiplier_joker = ScalingJoker::new(
        JokerId::Reserved3,
        "Test".to_string(),
        "Test".to_string(),
        JokerRarity::Common,
        1.0,
        0.5,
        ScalingTrigger::BlindCompleted,
        ScalingEffectType::MultMultiplier,
    );

    // Test effect calculation with accumulated values
    // Note: These would require proper context setup to test fully
    assert_eq!(chips_joker.effect_type, ScalingEffectType::Chips);
    assert_eq!(mult_joker.effect_type, ScalingEffectType::Mult);
    assert_eq!(multiplier_joker.effect_type, ScalingEffectType::MultMultiplier);
}

#[test]
fn test_max_value_cap() {
    let joker = ScalingJoker::new(
        JokerId::Reserved,
        "Test".to_string(),
        "Test".to_string(),
        JokerRarity::Common,
        0.0,
        10.0,
        ScalingTrigger::CardDiscarded,
        ScalingEffectType::Chips,
    )
    .with_max_value(50.0);

    assert_eq!(joker.max_value, Some(50.0));
}

#[test]
fn test_green_joker_creation() {
    let joker = GreenJoker::new();
    assert_eq!(joker.id(), JokerId::GreenJoker);
    assert_eq!(joker.name(), "Green Joker");
    assert_eq!(joker.rarity(), JokerRarity::Common);
}

#[test]
fn test_custom_scaling_jokers() {
    let jokers = create_all_custom_scaling_jokers();
    assert_eq!(jokers.len(), 7, "Should create exactly 7 custom scaling jokers");
    
    // Test that all jokers have unique IDs
    let mut ids = std::collections::HashSet::new();
    for joker in &jokers {
        assert!(ids.insert(joker.id()), "Duplicate joker ID found: {:?}", joker.id());
    }
}

#[test]
fn test_scaling_event_matching() {
    let hand_played_event = ScalingEvent::HandPlayed(HandRank::OnePair);
    let card_discarded_event = ScalingEvent::CardDiscarded;
    let money_gained_event = ScalingEvent::MoneyGained;
    
    // Test event types can be created and compared
    assert_eq!(hand_played_event, ScalingEvent::HandPlayed(HandRank::OnePair));
    assert_eq!(card_discarded_event, ScalingEvent::CardDiscarded);
    assert_eq!(money_gained_event, ScalingEvent::MoneyGained);
    
    // Test events are not equal to different events
    assert_ne!(hand_played_event, card_discarded_event);
    assert_ne!(card_discarded_event, money_gained_event);
}

#[test]
fn test_joker_factory_functions() {
    // Test that we can get scaling jokers by ID
    assert!(get_scaling_joker_by_id(JokerId::Trousers).is_some());
    assert!(get_scaling_joker_by_id(JokerId::GreenJoker).is_some());
    assert!(get_scaling_joker_by_id(JokerId::Banner).is_some());
    assert!(get_scaling_joker_by_id(JokerId::Ceremonial).is_some());
    
    // Test that non-scaling jokers return None
    assert!(get_scaling_joker_by_id(JokerId::Joker).is_none());
    
    // Test custom scaling jokers
    assert!(get_custom_scaling_joker_by_id(JokerId::GreenJoker).is_some());
    assert!(get_custom_scaling_joker_by_id(JokerId::Square).is_some());
    assert!(get_custom_scaling_joker_by_id(JokerId::Joker).is_none());
}

#[test]
fn test_rarity_distribution() {
    let jokers = create_all_scaling_jokers();
    let mut rarity_counts = HashMap::new();
    
    for joker in jokers {
        *rarity_counts.entry(joker.rarity).or_insert(0) += 1;
    }
    
    // Ensure we have jokers of different rarities
    assert!(rarity_counts.contains_key(&JokerRarity::Common));
    assert!(rarity_counts.contains_key(&JokerRarity::Uncommon));
    
    // Most jokers should be common or uncommon
    let common_and_uncommon = rarity_counts.get(&JokerRarity::Common).unwrap_or(&0) +
                              rarity_counts.get(&JokerRarity::Uncommon).unwrap_or(&0);
    assert!(common_and_uncommon >= 10, "Most scaling jokers should be common or uncommon");
}

#[test] 
fn test_joker_descriptions_are_descriptive() {
    let jokers = create_all_scaling_jokers();
    
    for joker in jokers {
        let description = &joker.description;
        
        // Check that descriptions contain key information
        let has_trigger_info = description.contains("per") || 
                              description.contains("when") ||
                              description.contains("each");
        let has_effect_info = description.contains("Mult") || 
                             description.contains("Chips") ||
                             description.contains("X") ||
                             description.contains("$");
        
        assert!(has_trigger_info || has_effect_info, 
               "Joker {:?} description '{}' should contain trigger or effect information", 
               joker.id, description);
    }
}

// Integration tests that would require full game context
// These are placeholder tests since we can't easily create full GameContext in unit tests

#[test]
#[ignore] // Ignore until we have proper test harness
fn test_scaling_joker_state_persistence() {
    // This test would verify that joker state is properly saved and restored
    // across game sessions using the JokerStateManager
    todo!("Implement integration test for state persistence");
}

#[test]
#[ignore] // Ignore until we have proper test harness  
fn test_scaling_joker_triggers_in_game() {
    // This test would verify that jokers properly trigger and accumulate
    // value during actual gameplay
    todo!("Implement integration test for joker triggers");
}

#[test]
#[ignore] // Ignore until we have proper test harness
fn test_scaling_joker_reset_conditions() {
    // This test would verify that reset conditions work properly
    // in actual game flow
    todo!("Implement integration test for reset conditions");
}

#[test]
#[ignore] // Ignore until we have proper test harness
fn test_performance_with_many_scaling_jokers() {
    // This test would verify that having multiple scaling jokers
    // doesn't significantly impact game performance
    todo!("Implement performance test for multiple scaling jokers");
}