//! Skip Tag Effects
//!
//! Common effect implementations and utilities for skip tags

use crate::game::Game;
use crate::rank::HandRank;
use crate::shop::packs::PackType;
use super::{SkipTagContext, SkipTagId, SkipTagResult};

/// Effect that gives immediate money reward
pub fn money_effect(context: SkipTagContext, amount: i64) -> SkipTagResult {
    let mut game = context.game;
    game.money += amount as f64;
    
    SkipTagResult {
        game,
        additional_tags: vec![],
        success: true,
        message: Some(format!("Gained ${}", amount)),
    }
}

/// Effect that gives a pack
pub fn pack_effect(context: SkipTagContext, pack_type: PackType) -> SkipTagResult {
    let mut game = context.game;
    
    // TODO: Implement pack giving in game state
    // For now, just acknowledge the effect
    
    SkipTagResult {
        game,
        additional_tags: vec![],
        success: true,
        message: Some(format!("Gained {} pack", pack_type)),
    }
}

/// Effect that modifies next shop
pub fn next_shop_modifier_effect(
    context: SkipTagContext,
    _modifier: Box<dyn Fn(&mut Game) + Send + Sync>,
) -> SkipTagResult {
    let game = context.game;
    
    // TODO: Store modifier for application on next shop
    // For now, just acknowledge the effect
    
    SkipTagResult {
        game,
        additional_tags: vec![],
        success: true,
        message: Some("Next shop will be modified".to_string()),
    }
}

/// Effect that duplicates another tag (Double tag)
pub fn duplication_effect(context: SkipTagContext, selected_tag: SkipTagId) -> SkipTagResult {
    // Exclude Double tags from duplication
    if matches!(selected_tag, SkipTagId::Double) {
        return SkipTagResult {
            game: context.game,
            additional_tags: vec![],
            success: false,
            message: Some("Cannot duplicate Double tags".to_string()),
        };
    }
    
    SkipTagResult {
        game: context.game,
        additional_tags: vec![selected_tag],
        success: true,
        message: Some(format!("Duplicated {} tag", selected_tag)),
    }
}

/// Effect that re-rolls boss blind
pub fn boss_reroll_effect(context: SkipTagContext) -> SkipTagResult {
    let game = context.game;
    
    // TODO: Implement boss blind re-roll logic
    // This should interact with the boss blind system and Director's Cut voucher
    
    SkipTagResult {
        game,
        additional_tags: vec![],
        success: true,
        message: Some("Next Boss Blind will be re-rolled".to_string()),
    }
}

/// Effect that upgrades a poker hand by levels
pub fn hand_upgrade_effect(context: SkipTagContext, levels: u32) -> SkipTagResult {
    let mut game = context.game;
    
    // Get all available hand types that can be upgraded
    let available_hands = get_upgradeable_hands(&game);
    
    if available_hands.is_empty() {
        return SkipTagResult {
            game,
            additional_tags: vec![],
            success: false,
            message: Some("No hands available to upgrade".to_string()),
        };
    }
    
    // Select a random hand to upgrade
    let random_index = game.rng.gen_range(0..available_hands.len());
    let selected_hand = available_hands[random_index];
    
    // TODO: Implement hand level upgrading
    // This should upgrade the selected hand by the specified levels
    
    SkipTagResult {
        game,
        additional_tags: vec![],
        success: true,
        message: Some(format!("Upgraded {} by {} levels", selected_hand, levels)),
    }
}

/// Effect that adds temporary hand size for next round
pub fn temporary_hand_size_effect(context: SkipTagContext, additional_size: u32) -> SkipTagResult {
    let game = context.game;
    
    // TODO: Implement temporary hand size tracking
    // This should add to a temporary modifier that's applied for one round only
    
    SkipTagResult {
        game,
        additional_tags: vec![],
        success: true,
        message: Some(format!("Added +{} hand size for next round", additional_size)),
    }
}

/// Get all hand types that can be upgraded
fn get_upgradeable_hands(_game: &Game) -> Vec<HandRank> {
    // TODO: This should check which hands have been played in the current run
    // and return all hands that can be upgraded, including secret hands
    
    // For now, return all basic hand types
    vec![
        HandRank::HighCard,
        HandRank::OnePair,
        HandRank::TwoPair,
        HandRank::ThreeOfAKind,
        HandRank::Straight,
        HandRank::Flush,
        HandRank::FullHouse,
        HandRank::FourOfAKind,
        HandRank::StraightFlush,
        HandRank::RoyalFlush,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;
    use crate::stage::Blind;

    fn create_test_context() -> SkipTagContext {
        SkipTagContext {
            game: Game::default(),
            skipped_blind: Some(Blind::Small),
            available_tags: vec![SkipTagId::Boss, SkipTagId::Orbital],
        }
    }

    #[test]
    fn test_money_effect() {
        let context = create_test_context();
        let initial_money = context.game.money;
        
        let result = money_effect(context, 50);
        
        assert!(result.success);
        assert_eq!(result.game.money, initial_money + 50.0);
        assert!(result.message.unwrap().contains("$50"));
    }

    #[test]
    fn test_duplication_effect_success() {
        let context = create_test_context();
        
        let result = duplication_effect(context, SkipTagId::Boss);
        
        assert!(result.success);
        assert_eq!(result.additional_tags.len(), 1);
        assert_eq!(result.additional_tags[0], SkipTagId::Boss);
    }

    #[test]
    fn test_duplication_effect_double_rejection() {
        let context = create_test_context();
        
        let result = duplication_effect(context, SkipTagId::Double);
        
        assert!(!result.success);
        assert!(result.additional_tags.is_empty());
        assert!(result.message.unwrap().contains("Cannot duplicate Double"));
    }

    #[test]
    fn test_hand_upgrade_effect() {
        let context = create_test_context();
        
        let result = hand_upgrade_effect(context, 3);
        
        assert!(result.success);
        let message = result.message.unwrap();
        assert!(message.contains("Upgraded"));
        assert!(message.contains("by 3 levels"));
    }

    #[test]
    fn test_temporary_hand_size_effect() {
        let context = create_test_context();
        
        let result = temporary_hand_size_effect(context, 3);
        
        assert!(result.success);
        assert!(result.message.unwrap().contains("+3 hand size"));
    }
}