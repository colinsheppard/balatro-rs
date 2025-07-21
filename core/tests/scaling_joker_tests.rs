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
    let rng = &balatro_rs::rng::GameRng::new();

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
fn test_scaling_joker_reset_conditions() {
    // Test that reset conditions work properly for scaling jokers
    
    // Create a ceremonial dagger with round end reset condition
    let mut joker = create_ceremonial_dagger();
    let context = create_test_context(100, 1, 1);
    
    // Initialize joker state
    let initial_state = joker.initialize_state(&context);
    assert_eq!(initial_state.accumulated_value, 1.0); // Base value
    
    // Create a mutable context for testing
    let mut test_context = create_test_context(100, 1, 1);
    
    // Set up initial state in the state manager
    test_context.joker_state_manager.set_state(joker.id, initial_state);
    
    // Trigger the joker to accumulate value (blind completed)
    joker.process_event(&mut test_context, &ScalingEvent::BlindCompleted);
    
    // Verify value has increased
    let current_value = test_context.joker_state_manager
        .get_accumulated_value(joker.id)
        .unwrap_or(joker.base_value);
    assert_eq!(current_value, 2.0); // Should be base + increment (1.0 + 1.0)
    
    // Trigger again to accumulate more
    joker.process_event(&mut test_context, &ScalingEvent::BlindCompleted);
    let value_after_second_trigger = test_context.joker_state_manager
        .get_accumulated_value(joker.id)
        .unwrap_or(joker.base_value);
    assert_eq!(value_after_second_trigger, 3.0); // Should be 2.0 + 1.0
    
    // Now trigger the reset condition (round end)
    joker.process_event(&mut test_context, &ScalingEvent::RoundEnd);
    
    // Verify value has reset to base value
    let value_after_reset = test_context.joker_state_manager
        .get_accumulated_value(joker.id)
        .unwrap_or(joker.base_value);
    assert_eq!(value_after_reset, 1.0); // Should be back to base value
    
    // Test that triggering after reset starts accumulating again from base
    joker.process_event(&mut test_context, &ScalingEvent::BlindCompleted);
    let value_after_post_reset_trigger = test_context.joker_state_manager
        .get_accumulated_value(joker.id)
        .unwrap_or(joker.base_value);
    assert_eq!(value_after_post_reset_trigger, 2.0); // Should be base + increment again
}

#[test]
fn test_multiple_reset_conditions() {
    // Test different types of reset conditions work correctly
    
    // Test 1: Round End reset condition (Ceremonial Dagger)
    let mut ceremonial = create_ceremonial_dagger();
    let mut context = create_test_context(100, 1, 1);
    let initial_state = ceremonial.initialize_state(&context);
    context.joker_state_manager.set_state(ceremonial.id, initial_state);
    
    // Accumulate value
    ceremonial.process_event(&mut context, &ScalingEvent::BlindCompleted);
    ceremonial.process_event(&mut context, &ScalingEvent::BlindCompleted);
    
    let accumulated = context.joker_state_manager
        .get_accumulated_value(ceremonial.id)
        .unwrap_or(ceremonial.base_value);
    assert_eq!(accumulated, 3.0); // 1.0 + 1.0 + 1.0
    
    // Test round end reset
    ceremonial.process_event(&mut context, &ScalingEvent::RoundEnd);
    let after_reset = context.joker_state_manager
        .get_accumulated_value(ceremonial.id)
        .unwrap_or(ceremonial.base_value);
    assert_eq!(after_reset, 1.0); // Back to base
    
    // Test 2: Never reset condition
    let mut never_reset_joker = ScalingJoker::new(
        JokerId::Reserved,
        "Never Reset Test".to_string(),
        "Never resets".to_string(),
        JokerRarity::Common,
        0.0,
        5.0,
        ScalingTrigger::CardDiscarded,
        ScalingEffectType::Chips,
    ).with_reset_condition(ResetCondition::Never);
    
    let initial_state = never_reset_joker.initialize_state(&context);
    context.joker_state_manager.set_state(never_reset_joker.id, initial_state);
    
    // Accumulate value
    never_reset_joker.process_event(&mut context, &ScalingEvent::CardDiscarded);
    never_reset_joker.process_event(&mut context, &ScalingEvent::CardDiscarded);
    
    let accumulated = context.joker_state_manager
        .get_accumulated_value(never_reset_joker.id)
        .unwrap_or(never_reset_joker.base_value);
    assert_eq!(accumulated, 10.0); // 0.0 + 5.0 + 5.0
    
    // Try various reset events - none should reset
    never_reset_joker.process_event(&mut context, &ScalingEvent::RoundEnd);
    never_reset_joker.process_event(&mut context, &ScalingEvent::AnteEnd);
    never_reset_joker.process_event(&mut context, &ScalingEvent::ShopEntered);
    
    let still_accumulated = context.joker_state_manager
        .get_accumulated_value(never_reset_joker.id)
        .unwrap_or(never_reset_joker.base_value);
    assert_eq!(still_accumulated, 10.0); // Should remain unchanged
}

#[test]
fn test_reset_before_trigger_order() {
    // Test that reset happens before trigger (as per the implementation)
    
    let mut joker = ScalingJoker::new(
        JokerId::Reserved2,
        "Test Order".to_string(),
        "Tests reset/trigger order".to_string(),
        JokerRarity::Common,
        10.0, // Base value
        5.0,  // Increment
        ScalingTrigger::HandPlayed(HandRank::OnePair),
        ScalingEffectType::Mult,
    ).with_reset_condition(ResetCondition::HandPlayed(HandRank::OnePair));
    
    let mut context = create_test_context(100, 1, 1);
    let initial_state = joker.initialize_state(&context);
    context.joker_state_manager.set_state(joker.id, initial_state);
    
    // First accumulate some value with a different trigger
    joker.process_event(&mut context, &ScalingEvent::BlindCompleted); // This won't trigger
    let after_non_trigger = context.joker_state_manager
        .get_accumulated_value(joker.id)
        .unwrap_or(joker.base_value);
    assert_eq!(after_non_trigger, 10.0); // Should remain at base (no trigger)
    
    // Manually increment to test reset order
    context.joker_state_manager.update_state(joker.id, |state| {
        state.accumulated_value = 25.0; // Set to accumulated value
    });
    
    // Now trigger an event that both resets AND triggers
    joker.process_event(&mut context, &ScalingEvent::HandPlayed(HandRank::OnePair));
    
    // This should reset FIRST (to base value 10.0) then trigger (add 5.0) = 15.0
    let final_value = context.joker_state_manager
        .get_accumulated_value(joker.id)
        .unwrap_or(joker.base_value);
    assert_eq!(final_value, 15.0); // 10.0 (reset to base) + 5.0 (trigger increment)
}

#[test]
fn test_reset_conditions_with_different_events() {
    // Test various reset conditions with their corresponding events
    
    let mut context = create_test_context(100, 1, 1);
    
    // Test ante end reset
    let mut ante_reset_joker = ScalingJoker::new(
        JokerId::Reserved3,
        "Ante Reset Test".to_string(),
        "Resets at ante end".to_string(),
        JokerRarity::Common,
        5.0,
        3.0,
        ScalingTrigger::MoneyGained,
        ScalingEffectType::Chips,
    ).with_reset_condition(ResetCondition::AnteEnd);
    
    let initial_state = ante_reset_joker.initialize_state(&context);
    context.joker_state_manager.set_state(ante_reset_joker.id, initial_state);
    
    // Accumulate value
    ante_reset_joker.process_event(&mut context, &ScalingEvent::MoneyGained);
    ante_reset_joker.process_event(&mut context, &ScalingEvent::MoneyGained);
    
    let accumulated = context.joker_state_manager
        .get_accumulated_value(ante_reset_joker.id)
        .unwrap_or(ante_reset_joker.base_value);
    assert_eq!(accumulated, 11.0); // 5.0 + 3.0 + 3.0
    
    // Round end should not reset this joker
    ante_reset_joker.process_event(&mut context, &ScalingEvent::RoundEnd);
    let after_round_end = context.joker_state_manager
        .get_accumulated_value(ante_reset_joker.id)
        .unwrap_or(ante_reset_joker.base_value);
    assert_eq!(after_round_end, 11.0); // Should remain unchanged
    
    // Ante end should reset this joker
    ante_reset_joker.process_event(&mut context, &ScalingEvent::AnteEnd);
    let after_ante_end = context.joker_state_manager
        .get_accumulated_value(ante_reset_joker.id)
        .unwrap_or(ante_reset_joker.base_value);
    assert_eq!(after_ante_end, 5.0); // Back to base value
}

#[test]
#[ignore] // Ignore until we have proper test harness
fn test_performance_with_many_scaling_jokers() {
    // This test would verify that having multiple scaling jokers
    // doesn't significantly impact game performance
    todo!("Implement performance test for multiple scaling jokers");
}