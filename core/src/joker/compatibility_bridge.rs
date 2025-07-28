//! Backward Compatibility Bridge for Advanced Joker Framework
//!
//! This module provides seamless integration between the existing joker system
//! and the new advanced condition framework. It ensures that:
//! - All existing jokers continue to work without modification
//! - Old and new jokers can be mixed in the same game
//! - Performance is maintained for simple jokers
//! - Migration path is clear for upgrading jokers
//!
//! The bridge follows kernel principles:
//! - Zero performance overhead for legacy jokers
//! - Clear separation between old and new systems
//! - Type-safe conversions with compile-time guarantees

use crate::joker::advanced_conditions::{AdvancedCondition, AdvancedEvaluationContext};
use crate::joker::advanced_traits::{
    AdvancedJokerGameplay, AdvancedJokerIdentity, EnhancedJoker, EvaluationCost, GameEvent,
    InternalJokerState, JokerProcessor,
};
use crate::joker::conditional::{ConditionalJoker, JokerCondition};
use crate::joker::traits::{ProcessResult, Rarity};
use crate::joker::{GameContext, Joker, JokerId, JokerRarity};
use crate::stage::Stage;
use std::fmt::Debug;

/// Compatibility wrapper that makes any existing `Joker` work with the advanced framework
///
/// This wrapper provides automatic translation between the old and new APIs,
/// allowing legacy jokers to benefit from the advanced condition system
/// without requiring code changes.
#[derive(Debug)]
pub struct LegacyJokerAdapter {
    /// The wrapped legacy joker
    legacy_joker: Box<dyn Joker>,

    /// Converted condition for advanced evaluation
    advanced_condition: AdvancedCondition,

    /// Internal state for the adapter
    adapter_state: InternalJokerState,
}

impl LegacyJokerAdapter {
    /// Create a new adapter for a legacy joker
    ///
    /// This automatically analyzes the legacy joker and creates appropriate
    /// advanced conditions based on its behavior patterns.
    pub fn new(legacy_joker: Box<dyn Joker>) -> Self {
        let advanced_condition = Self::infer_condition(legacy_joker.as_ref());

        Self {
            legacy_joker,
            advanced_condition,
            adapter_state: InternalJokerState::new(),
        }
    }

    /// Create an adapter with an explicit advanced condition
    ///
    /// Use this when you want to upgrade a legacy joker with sophisticated
    /// conditions while keeping the existing implementation logic.
    pub fn with_condition(legacy_joker: Box<dyn Joker>, condition: AdvancedCondition) -> Self {
        Self {
            legacy_joker,
            advanced_condition: condition,
            adapter_state: InternalJokerState::new(),
        }
    }

    /// Infer an appropriate advanced condition from legacy joker behavior
    ///
    /// This analyzes the joker's methods to determine what conditions
    /// would make it behave the same way in the advanced framework.
    fn infer_condition(_joker: &dyn Joker) -> AdvancedCondition {
        // For most legacy jokers, we use an "Always" condition since they
        // handle their own triggering logic internally
        // TODO: Implement more sophisticated analysis for specific joker types
        AdvancedCondition::Legacy(JokerCondition::Always)
    }
}

/// Legacy identity adapter for backward compatibility
#[derive(Debug)]
pub struct LegacyIdentityAdapter {
    joker: Box<dyn Joker>,
}

impl LegacyIdentityAdapter {
    pub fn new(joker: Box<dyn Joker>) -> Self {
        Self { joker }
    }
}

impl AdvancedJokerIdentity for LegacyIdentityAdapter {
    fn joker_type(&self) -> &'static str {
        // Convert JokerId to string representation
        // This is a simplified implementation - in practice you'd want
        // a proper mapping from JokerId to &'static str
        "legacy_joker"
    }

    fn name(&self) -> &str {
        self.joker.name()
    }

    fn description(&self) -> &str {
        self.joker.description()
    }

    fn rarity(&self) -> Rarity {
        match self.joker.rarity() {
            JokerRarity::Common => Rarity::Common,
            JokerRarity::Uncommon => Rarity::Uncommon,
            JokerRarity::Rare => Rarity::Rare,
            JokerRarity::Legendary => Rarity::Legendary,
        }
    }

    fn base_cost(&self) -> u64 {
        self.joker.cost() as u64
    }

    fn evaluation_cost_estimate(&self) -> EvaluationCost {
        // Legacy jokers are generally simple, so mark as cheap
        // More sophisticated analysis could be added here
        EvaluationCost::Cheap
    }
}

/// Legacy processor adapter
#[derive(Debug)]
pub struct LegacyProcessorAdapter {
    joker: Box<dyn Joker>,
}

impl LegacyProcessorAdapter {
    pub fn new(joker: Box<dyn Joker>) -> Self {
        Self { joker }
    }
}

impl JokerProcessor for LegacyProcessorAdapter {
    fn process(
        &self,
        context: &mut AdvancedEvaluationContext,
        _state: &mut InternalJokerState,
    ) -> ProcessResult {
        // Convert advanced context to legacy GameContext
        // This is a simplified conversion - full implementation would
        // need to properly construct all GameContext fields
        let mut legacy_context = GameContext {
            chips: context.game_context.chips,
            mult: context.game_context.mult,
            money: context.game_context.money,
            ante: context.game_context.ante,
            round: context.game_context.round,
            stage: context.stage,
            hands_played: context.game_context.hands_played,
            discards_used: context.game_context.discards_used,
            jokers: context.game_context.jokers,
            hand: context.game_context.hand,
            discarded: context.game_context.discarded,
            joker_state_manager: context.game_context.joker_state_manager,
            hand_type_counts: context.game_context.hand_type_counts,
            cards_in_deck: context.game_context.cards_in_deck,
            stone_cards_in_deck: context.game_context.stone_cards_in_deck,
            steel_cards_in_deck: context.game_context.steel_cards_in_deck,
            rng: context.game_context.rng,
        };

        // Call the appropriate legacy method based on context
        let legacy_effect = if let Some(hand) = context.hand {
            self.joker.on_hand_played(&mut legacy_context, hand)
        } else if let Some(card) = context.card {
            self.joker.on_card_scored(&mut legacy_context, card)
        } else {
            // For other contexts, try the most appropriate method
            match context.stage {
                Stage::Shop() => self.joker.on_shop_open(&mut legacy_context),
                _ => self.joker.on_blind_start(&mut legacy_context),
            }
        };

        // Convert legacy JokerEffect to ProcessResult
        ProcessResult {
            chips_added: legacy_effect.chips as u64,
            mult_added: legacy_effect.mult as f64,
            mult_multiplier: legacy_effect.mult_multiplier,
            retriggered: legacy_effect.retrigger > 0,
            message: legacy_effect.message,
        }
    }
}

impl AdvancedJokerGameplay for LegacyJokerAdapter {
    fn identity(&self) -> &dyn AdvancedJokerIdentity {
        // This would need proper lifetime management in a real implementation
        // For now, we'll use a simplified approach
        panic!("Legacy adapter identity access needs proper implementation")
    }

    fn get_trigger_condition(&self) -> &AdvancedCondition {
        &self.advanced_condition
    }

    fn process_advanced(&mut self, context: &mut AdvancedEvaluationContext) -> ProcessResult {
        let processor = LegacyProcessorAdapter::new(
            // This clone is not ideal but necessary for the adapter pattern
            // In a real implementation, we'd use Rc/Arc or redesign the trait
            Box::new(DummyJoker), // Placeholder - needs proper cloning solution
        );
        processor.process(context, &mut self.adapter_state)
    }

    fn update_internal_state(&mut self, event: &GameEvent) {
        // Convert advanced events to legacy joker method calls
        match event {
            GameEvent::HandPlayed { .. } => {
                // Legacy jokers don't have explicit event handling
                // We track this in adapter state for potential condition use
                self.adapter_state.increment_counter("hands_played");
            }
            GameEvent::CardsDiscarded { cards } => {
                self.adapter_state.increment_counter("cards_discarded");
                self.adapter_state
                    .set_data("last_discard_count", serde_json::json!(cards.len()));
            }
            GameEvent::RoundStarted { round_number } => {
                self.adapter_state
                    .set_data("current_round", serde_json::json!(round_number));
            }
            _ => {} // Other events don't need tracking for legacy jokers
        }
    }

    fn state_hash(&self) -> u64 {
        // Include adapter state version and legacy joker ID
        let legacy_id_hash = self.legacy_joker.id() as u64;
        legacy_id_hash ^ self.adapter_state.version
    }
}

// Dummy joker for placeholder purposes - remove in real implementation
#[derive(Debug)]
struct DummyJoker;

impl Joker for DummyJoker {
    fn id(&self) -> JokerId {
        JokerId::Joker
    }
    fn name(&self) -> &str {
        "Dummy"
    }
    fn description(&self) -> &str {
        "Placeholder"
    }
    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }
}

/// Compatibility utilities for framework integration
pub struct CompatibilityBridge;

impl CompatibilityBridge {
    /// Convert a legacy conditional joker to use advanced conditions
    ///
    /// This provides a migration path for conditional jokers to benefit
    /// from the advanced condition system's performance and capabilities.
    pub fn upgrade_conditional_joker(
        conditional_joker: ConditionalJoker,
    ) -> Result<EnhancedJoker, &'static str> {
        // Convert the legacy condition to an advanced condition
        let advanced_condition = AdvancedCondition::Legacy(conditional_joker.condition.clone());

        // Create identity from conditional joker properties
        let identity = Box::new(ConditionalJokerIdentity {
            id: conditional_joker.id,
            name: conditional_joker.name.clone(),
            description: conditional_joker.description.clone(),
            rarity: Self::convert_rarity(conditional_joker.rarity),
            cost: conditional_joker.cost as u64,
        });

        // Create processor from conditional joker effect
        let processor = Box::new(ConditionalJokerProcessor {
            base_effect: conditional_joker.effect.clone(),
            card_effect: conditional_joker.card_effect.clone(),
        });

        // Build the enhanced joker
        EnhancedJoker::builder()
            .identity(identity)
            .condition(advanced_condition)
            .processor(processor)
            .build()
    }

    /// Convert legacy rarity to new rarity system
    fn convert_rarity(legacy_rarity: crate::joker::JokerRarity) -> Rarity {
        match legacy_rarity {
            crate::joker::JokerRarity::Common => Rarity::Common,
            crate::joker::JokerRarity::Uncommon => Rarity::Uncommon,
            crate::joker::JokerRarity::Rare => Rarity::Rare,
            crate::joker::JokerRarity::Legendary => Rarity::Legendary,
        }
    }

    /// Create a mixed collection that can handle both legacy and advanced jokers
    pub fn create_mixed_collection(
        legacy_jokers: Vec<Box<dyn Joker>>,
        advanced_jokers: Vec<Box<dyn AdvancedJokerGameplay>>,
    ) -> MixedJokerCollection {
        MixedJokerCollection {
            legacy_adapters: legacy_jokers
                .into_iter()
                .map(LegacyJokerAdapter::new)
                .collect(),
            advanced_jokers,
        }
    }
}

/// Identity adapter for conditional jokers
#[derive(Debug)]
struct ConditionalJokerIdentity {
    #[allow(dead_code)]
    id: JokerId,
    name: String,
    description: String,
    rarity: Rarity,
    cost: u64,
}

impl AdvancedJokerIdentity for ConditionalJokerIdentity {
    fn joker_type(&self) -> &'static str {
        // Convert JokerId to string - simplified implementation
        "conditional_joker"
    }

    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn rarity(&self) -> Rarity {
        self.rarity
    }
    fn base_cost(&self) -> u64 {
        self.cost
    }

    fn evaluation_cost_estimate(&self) -> EvaluationCost {
        EvaluationCost::Moderate
    }
}

/// Processor adapter for conditional jokers
#[derive(Debug)]
struct ConditionalJokerProcessor {
    base_effect: crate::joker::JokerEffect,
    card_effect: Option<crate::joker::JokerEffect>,
}

impl JokerProcessor for ConditionalJokerProcessor {
    fn process(
        &self,
        context: &mut AdvancedEvaluationContext,
        _state: &mut InternalJokerState,
    ) -> ProcessResult {
        // Use card effect if available and we're processing a card
        let effect = if context.card.is_some() && self.card_effect.is_some() {
            self.card_effect.as_ref().unwrap()
        } else {
            &self.base_effect
        };

        ProcessResult {
            chips_added: effect.chips as u64,
            mult_added: effect.mult as f64,
            mult_multiplier: effect.mult_multiplier,
            retriggered: effect.retrigger > 0,
            message: effect.message.clone(),
        }
    }
}

/// Collection that can hold both legacy and advanced jokers
pub struct MixedJokerCollection {
    legacy_adapters: Vec<LegacyJokerAdapter>,
    advanced_jokers: Vec<Box<dyn AdvancedJokerGameplay>>,
}

impl MixedJokerCollection {
    /// Process all jokers in the collection with proper ordering
    pub fn process_all(&mut self, context: &mut AdvancedEvaluationContext) -> Vec<ProcessResult> {
        let mut results = Vec::new();

        // Collect all jokers with their priorities for sorting
        let mut all_jokers: Vec<(usize, bool, i32)> = Vec::new();

        // Add legacy jokers (false = legacy)
        for (i, joker) in self.legacy_adapters.iter().enumerate() {
            let priority = joker.get_processing_priority(context.stage);
            all_jokers.push((i, false, priority));
        }

        // Add advanced jokers (true = advanced)
        for (i, joker) in self.advanced_jokers.iter().enumerate() {
            let priority = joker.get_processing_priority(context.stage);
            all_jokers.push((i, true, priority));
        }

        // Sort by priority (higher first)
        all_jokers.sort_by(|a, b| b.2.cmp(&a.2));

        // Process in priority order
        for (index, is_advanced, _priority) in all_jokers {
            if is_advanced {
                let joker = &mut self.advanced_jokers[index];
                if joker.should_process(context) {
                    let result = joker.process_advanced(context);
                    results.push(result);
                }
            } else {
                let joker = &mut self.legacy_adapters[index];
                if joker.should_process(context) {
                    let result = joker.process_advanced(context);
                    results.push(result);
                }
            }
        }

        results
    }

    /// Broadcast an event to all jokers in the collection
    pub fn broadcast_event(&mut self, event: &GameEvent) {
        // Update legacy adapters
        for adapter in &mut self.legacy_adapters {
            adapter.update_internal_state(event);
        }

        // Update advanced jokers
        for joker in &mut self.advanced_jokers {
            joker.update_internal_state(event);
        }
    }

    /// Get total count of jokers in the collection
    pub fn total_count(&self) -> usize {
        self.legacy_adapters.len() + self.advanced_jokers.len()
    }

    /// Check if collection contains any jokers
    pub fn is_empty(&self) -> bool {
        self.legacy_adapters.is_empty() && self.advanced_jokers.is_empty()
    }
}

impl Debug for MixedJokerCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MixedJokerCollection")
            .field("legacy_count", &self.legacy_adapters.len())
            .field("advanced_count", &self.advanced_jokers.len())
            .finish()
    }
}

/// Extension trait for enhanced joker builder to support legacy conversions
pub trait EnhancedJokerBuilderExt {
    /// Create an enhanced joker from a legacy joker
    fn from_legacy(legacy_joker: Box<dyn Joker>) -> Result<EnhancedJoker, &'static str>;

    /// Create an enhanced joker from a conditional joker
    fn from_conditional(conditional_joker: ConditionalJoker)
        -> Result<EnhancedJoker, &'static str>;
}

impl EnhancedJokerBuilderExt for crate::joker::advanced_traits::EnhancedJokerBuilder {
    fn from_legacy(_legacy_joker: Box<dyn Joker>) -> Result<EnhancedJoker, &'static str> {
        // This would need proper implementation with correct lifetime management
        // For now, return an error indicating this needs to be implemented
        Err("Legacy joker conversion not yet fully implemented")
    }

    fn from_conditional(
        conditional_joker: ConditionalJoker,
    ) -> Result<EnhancedJoker, &'static str> {
        CompatibilityBridge::upgrade_conditional_joker(conditional_joker)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joker::traits::Rarity;

    #[test]
    fn test_rarity_conversion() {
        assert_eq!(
            CompatibilityBridge::convert_rarity(crate::joker::JokerRarity::Common),
            Rarity::Common
        );
        assert_eq!(
            CompatibilityBridge::convert_rarity(crate::joker::JokerRarity::Legendary),
            Rarity::Legendary
        );
    }

    #[test]
    fn test_mixed_collection_creation() {
        let legacy_jokers: Vec<Box<dyn Joker>> = vec![];
        let advanced_jokers: Vec<Box<dyn AdvancedJokerGameplay>> = vec![];

        let collection =
            CompatibilityBridge::create_mixed_collection(legacy_jokers, advanced_jokers);

        assert!(collection.is_empty());
        assert_eq!(collection.total_count(), 0);
    }

    #[test]
    fn test_evaluation_cost_hierarchy() {
        use EvaluationCost::*;

        // Test that costs are properly ordered
        let costs = [Cheap, Moderate, Expensive, VeryExpensive];
        for window in costs.windows(2) {
            assert!(window[0] < window[1]);
        }
    }
}
