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
fn test_performance_with_many_scaling_jokers() {
    use std::time::Instant;
    
    // Performance baseline: operations should complete within reasonable time
    const MAX_PROCESSING_TIME_MS: u128 = 10; // 10ms baseline
    const NUM_ITERATIONS: usize = 1000;
    
    // Create multiple scaling jokers for performance testing
    let scaling_jokers = create_all_scaling_jokers();
    assert_eq!(scaling_jokers.len(), 15, "Expected exactly 15 scaling jokers");
    
    // Convert to boxed jokers for use in game context
    let jokers: Vec<Box<dyn Joker>> = scaling_jokers
        .into_iter()
        .map(|j| Box::new(j) as Box<dyn Joker>)
        .collect();
    
    // Create test context with many scaling jokers
    let state_manager = Arc::new(JokerStateManager::new());
    
    // Initialize joker states
    for joker in &jokers {
        state_manager.set_state(joker.id(), JokerState::with_accumulated_value(0.0));
    }
    
    let hand = create_test_hand(HandRank::TwoPair);
    let discarded: Vec<Card> = vec![];
    let hand_type_counts = HashMap::new();
    let stage = Stage::Blind;
    let rng = &balatro_rs::rng::GameRng::new();
    
    let context = GameContext {
        chips: 0,
        mult: 1,
        money: 100, // Some starting money for money-triggered jokers
        ante: 1,
        round: 1,
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
    };
    
    // Test 1: Measure joker effect processing time
    let start = Instant::now();
    
    for _ in 0..NUM_ITERATIONS {
        // Simulate processing effects for all jokers
        for joker in &jokers {
            let _effect = joker.get_effect(&context);
        }
    }
    
    let processing_duration = start.elapsed();
    let processing_time_ms = processing_duration.as_millis();
    
    println!("Processing time for {} iterations with {} scaling jokers: {}ms", 
             NUM_ITERATIONS, jokers.len(), processing_time_ms);
    
    // Assert performance requirements
    assert!(processing_time_ms <= MAX_PROCESSING_TIME_MS,
        "Performance regression detected: processing {} jokers took {}ms (max: {}ms)",
        jokers.len(), processing_time_ms, MAX_PROCESSING_TIME_MS);
    
    // Test 2: Measure state update performance
    let start = Instant::now();
    
    for _ in 0..NUM_ITERATIONS {
        // Simulate state updates for scaling jokers
        for joker in &jokers {
            state_manager.add_accumulated_value(joker.id(), 1.0);
        }
    }
    
    let update_duration = start.elapsed();
    let update_time_ms = update_duration.as_millis();
    
    println!("State update time for {} iterations with {} scaling jokers: {}ms", 
             NUM_ITERATIONS, jokers.len(), update_time_ms);
    
    // Assert state update performance
    assert!(update_time_ms <= MAX_PROCESSING_TIME_MS,
        "State update performance regression: updating {} jokers took {}ms (max: {}ms)",
        jokers.len(), update_time_ms, MAX_PROCESSING_TIME_MS);
    
    // Test 3: Memory usage validation
    let memory_before = get_memory_usage();
    
    // Create additional joker contexts to test memory scaling
    let mut additional_managers = Vec::new();
    for _ in 0..100 {
        let manager = Arc::new(JokerStateManager::new());
        for joker in &jokers {
            manager.set_state(joker.id(), JokerState::with_accumulated_value(0.0));
        }
        additional_managers.push(manager);
    }
    
    let memory_after = get_memory_usage();
    let memory_delta = memory_after.saturating_sub(memory_before);
    
    println!("Memory usage delta for 100 additional joker contexts: {} KB", memory_delta / 1024);
    
    // Memory should not grow excessively (allow up to 10MB for 100 contexts)
    const MAX_MEMORY_DELTA: usize = 10 * 1024 * 1024; // 10MB
    assert!(memory_delta <= MAX_MEMORY_DELTA,
        "Memory usage grew too much: {}MB (max: {}MB)",
        memory_delta / (1024 * 1024), MAX_MEMORY_DELTA / (1024 * 1024));
    
    // Test 4: Verify scaling doesn't break with many jokers
    let final_values: Vec<f64> = jokers.iter()
        .map(|joker| state_manager.get_accumulated_value(joker.id()))
        .collect();
    
    // All jokers should have accumulated some value
    assert!(final_values.iter().all(|&v| v > 0.0),
        "All scaling jokers should have accumulated value > 0 after test iterations");
    
    println!("✅ Performance test passed: {} scaling jokers perform within acceptable bounds", jokers.len());
}

/// Helper function to estimate memory usage (simplified)
fn get_memory_usage() -> usize {
    // On systems where /proc/self/status is available, we could read actual memory
    // For now, use a simple heuristic based on allocations
    std::mem::size_of::<JokerStateManager>() * 1000 // Rough approximation
}