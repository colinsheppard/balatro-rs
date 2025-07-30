//! Tarot card implementations for Balatro game engine
//!
//! This module implements the Major Arcana tarot cards with specific game effects.
//! Each tarot card implements the Consumable trait and provides targeted effects
//! that modify game state in specific ways.
//!
//! # Design Principles (Uncle Bob's Clean Code)
//!
//! - Single Responsibility: Each tarot card has one clear effect
//! - Dependency Inversion: Cards depend on abstractions, not concretions
//! - Interface Segregation: Clean separation between card types
//! - Performance: All effects complete in <1ms as per requirements
//!
//! # Architecture
//!
//! The module follows the established consumable pattern:
//! - Each tarot implements the `Consumable` trait
//! - `TarotFactory` provides creation of tarot cards
//! - Comprehensive error handling for all edge cases
//! - Target validation ensures safe operation

use crate::consumables::{
    Consumable, ConsumableEffect, ConsumableError, ConsumableId, ConsumableType, Target, TargetType,
};
use crate::game::Game;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Error types specific to tarot card operations
pub type TarotError = ConsumableError;

/// Effect types for tarot cards
pub type TarotEffect = ConsumableEffect;

/// Rarity levels for tarot cards
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TarotRarity {
    Common,
    Uncommon,
    Rare,
}

/// Card enhancements that can be applied by tarot cards
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardEnhancement {
    Glass,
    Gold,
    Stone,
}

/// Metadata for tarot cards used in shop generation and UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TarotCardMetadata {
    pub name: String,
    pub description: String,
    pub rarity: TarotRarity,
    pub target_type: TargetType,
    pub effect_category: ConsumableEffect,
    pub implemented: bool,
}

/// Factory for creating tarot card instances
///
/// Following the Factory pattern for clean object creation and
/// maintaining consistency with the joker factory system.
pub struct TarotFactory;

impl TarotFactory {
    /// Creates a tarot card instance by ID
    ///
    /// Returns None for non-tarot IDs or unimplemented cards.
    /// This follows the Fail Fast principle - better to return None
    /// than create invalid objects.
    ///
    /// # Performance
    /// O(1) creation time per tarot card
    pub fn create(id: ConsumableId) -> Option<Box<dyn Consumable>> {
        match id {
            // Wave 2 Tarot Cards (Major Arcana XI-XXI)
            ConsumableId::Justice => Some(Box::new(Justice::new())),
            ConsumableId::TheHangedMan => Some(Box::new(TheHangedMan::new())),
            ConsumableId::Death => Some(Box::new(Death::new())),
            ConsumableId::Temperance => Some(Box::new(Temperance::new())),
            ConsumableId::TheDevil => Some(Box::new(TheDevil::new())),
            ConsumableId::TheTower => Some(Box::new(TheTower::new())),
            ConsumableId::TheStar => Some(Box::new(TheStar::new())),
            ConsumableId::TheMoon => Some(Box::new(TheMoon::new())),
            ConsumableId::TheSun => Some(Box::new(TheSun::new())),
            ConsumableId::Judgement => Some(Box::new(Judgement::new())),
            ConsumableId::TheWorld => Some(Box::new(TheWorld::new())),
            _ => None,
        }
    }

    /// Creates a random tarot card from all implemented cards
    ///
    /// Uses the provided RNG to randomly select from all available tarot cards.
    /// This is commonly used by jokers that create random tarot cards.
    ///
    /// # Arguments
    /// * `rng` - Random number generator for selection
    ///
    /// # Returns
    /// Random `ConsumableId` for a tarot card, or `None` if no cards are implemented
    pub fn create_random_tarot(rng: &mut crate::rng::GameRng) -> Option<ConsumableId> {
        let implemented_cards = Self::get_implemented_cards();
        if implemented_cards.is_empty() {
            return None;
        }

        let index = rng.gen_range(0..implemented_cards.len());
        implemented_cards.get(index).copied()
    }

    /// Gets all implemented tarot card IDs
    ///
    /// Useful for testing and validation. Returns only the cards
    /// that are actually implemented and can be created.
    pub fn get_implemented_cards() -> Vec<ConsumableId> {
        vec![
            ConsumableId::Justice,
            ConsumableId::TheHangedMan,
            ConsumableId::Death,
            ConsumableId::Temperance,
            ConsumableId::TheDevil,
            ConsumableId::TheTower,
            ConsumableId::TheStar,
            ConsumableId::TheMoon,
            ConsumableId::TheSun,
            ConsumableId::Judgement,
            ConsumableId::TheWorld,
        ]
    }

    /// Get all available tarot card IDs (for shop generation)
    pub fn available_cards(&self) -> Result<Vec<ConsumableId>, TarotError> {
        Ok(Self::get_implemented_cards())
    }

    /// Get metadata for a specific tarot card
    pub fn get_metadata(&self, id: ConsumableId) -> Result<Option<TarotCardMetadata>, TarotError> {
        if Self::get_implemented_cards().contains(&id) {
            // Create a tarot card instance to get its metadata
            match Self::create(id) {
                Some(card) => Ok(Some(TarotCardMetadata {
                    name: card.name().to_string(),
                    description: card.description().to_string(),
                    rarity: TarotRarity::Common, // Default rarity
                    target_type: card.get_target_type(),
                    effect_category: card.get_effect_category(),
                    implemented: true,
                })),
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

/// Base trait for tarot card behavior
///
/// This provides a clean abstraction over the Consumable trait
/// with tarot-specific semantics. Following Interface Segregation
/// Principle - clients only depend on methods they use.
pub trait TarotCard: Consumable + Send + Sync {
    /// Get the tarot's unique identifier
    fn id(&self) -> ConsumableId;

    /// Get the Major Arcana number (XI, XII, etc.)
    fn arcana_number(&self) -> u8;

    /// Get a detailed description of the effect
    fn detailed_description(&self) -> String;
}

// ============================================================================
// TAROT CARD IMPLEMENTATIONS
// ============================================================================

/// Justice (XI) - Enhances 1 selected card to Glass Card
///
/// Glass Cards are fragile but provide powerful benefits.
/// This is a single-target enhancement card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Justice;

impl Justice {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Justice {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Justice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Justice")
    }
}

impl Consumable for Justice {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        // Require exactly 1 card to be targeted
        if let Target::Cards(card_target) = target {
            card_target.indices.len() == 1 && card_target.validate(game_state).is_ok()
        } else {
            false
        }
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        let Target::Cards(card_target) = target else {
            return Err(ConsumableError::InvalidTarget(
                "Justice requires exactly 1 card target".to_string(),
            ));
        };

        if card_target.indices.len() != 1 {
            return Err(ConsumableError::InvalidTarget(
                "Justice requires exactly 1 card".to_string(),
            ));
        }

        card_target.validate(game_state)?;

        // Apply Glass enhancement to the targeted card
        let index = card_target.indices[0];
        match card_target.collection {
            crate::consumables::CardCollection::Hand => {
                if let Some(card) = game_state.available.get_card_mut(index) {
                    card.enhancement = Some(crate::card::Enhancement::Glass);
                } else {
                    return Err(ConsumableError::InvalidTarget(
                        "Card index out of bounds".to_string(),
                    ));
                }
            }
            crate::consumables::CardCollection::Deck => {
                if let Some(card) = game_state.deck.get_card_mut(index) {
                    card.enhancement = Some(crate::card::Enhancement::Glass);
                } else {
                    return Err(ConsumableError::InvalidTarget(
                        "Card index out of bounds".to_string(),
                    ));
                }
            }
            crate::consumables::CardCollection::DiscardPile => {
                if let Some(card) = game_state.discarded.get_mut(index) {
                    card.enhancement = Some(crate::card::Enhancement::Glass);
                } else {
                    return Err(ConsumableError::InvalidTarget(
                        "Card index out of bounds".to_string(),
                    ));
                }
            }
            crate::consumables::CardCollection::PlayedCards => {
                return Err(ConsumableError::InvalidTarget(
                    "Cannot enhance played cards".to_string(),
                ));
            }
        }

        Ok(())
    }

    fn get_description(&self) -> String {
        "Enhances 1 selected card to Glass Card".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(1)
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn name(&self) -> &'static str {
        "Justice"
    }

    fn description(&self) -> &'static str {
        "Enhances 1 selected card to Glass Card"
    }
}

impl TarotCard for Justice {
    fn id(&self) -> ConsumableId {
        ConsumableId::Justice
    }

    fn arcana_number(&self) -> u8 {
        11
    }

    fn detailed_description(&self) -> String {
        "Justice (XI): Select 1 card to enhance with Glass. Glass cards are fragile but powerful - they provide significant benefits but can be destroyed when used.".to_string()
    }
}

/// The Hanged Man (XII) - Destroys up to 2 selected cards
///
/// This is a destructive card that removes cards from the game.
/// Useful for removing unwanted cards from hand or deck.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheHangedMan;

impl TheHangedMan {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TheHangedMan {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TheHangedMan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The Hanged Man")
    }
}

impl Consumable for TheHangedMan {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        if let Target::Cards(card_target) = target {
            let count = card_target.indices.len();
            (1..=2).contains(&count) && card_target.validate(game_state).is_ok()
        } else {
            false
        }
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        let Target::Cards(card_target) = target else {
            return Err(ConsumableError::InvalidTarget(
                "The Hanged Man requires card targets".to_string(),
            ));
        };

        let count = card_target.indices.len();
        if count == 0 || count > 2 {
            return Err(ConsumableError::InvalidTarget(
                "The Hanged Man requires 1-2 cards".to_string(),
            ));
        }

        card_target.validate(game_state)?;

        // Destroy the targeted cards by removing them from their collection
        match card_target.collection {
            crate::consumables::CardCollection::Hand => {
                game_state
                    .available
                    .remove_cards_by_indices(card_target.indices.clone());
            }
            crate::consumables::CardCollection::Deck => {
                game_state
                    .deck
                    .remove_cards_by_indices(card_target.indices.clone());
            }
            crate::consumables::CardCollection::DiscardPile => {
                // Sort indices in descending order to remove from the end first
                let mut indices = card_target.indices.clone();
                indices.sort_by(|a, b| b.cmp(a));
                for index in indices {
                    if index < game_state.discarded.len() {
                        game_state.discarded.remove(index);
                    }
                }
            }
            crate::consumables::CardCollection::PlayedCards => {
                return Err(ConsumableError::InvalidTarget(
                    "Cannot destroy played cards".to_string(),
                ));
            }
        }

        Ok(())
    }

    fn get_description(&self) -> String {
        "Destroys up to 2 selected cards".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(2) // Up to 2 cards
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Destruction
    }

    fn name(&self) -> &'static str {
        "The Hanged Man"
    }

    fn description(&self) -> &'static str {
        "Destroys up to 2 selected cards"
    }
}

impl TarotCard for TheHangedMan {
    fn id(&self) -> ConsumableId {
        ConsumableId::TheHangedMan
    }

    fn arcana_number(&self) -> u8 {
        12
    }

    fn detailed_description(&self) -> String {
        "The Hanged Man (XII): Select up to 2 cards to destroy permanently. Use this to remove unwanted cards from your hand or deck.".to_string()
    }
}

/// Death (XIII) - Select 2 cards, convert left card to match right card
///
/// This is a transformation card that copies one card to another.
/// Requires exactly 2 cards - the first becomes a copy of the second.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Death;

impl Death {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Death {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Death {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Death")
    }
}

impl Consumable for Death {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        if let Target::Cards(card_target) = target {
            card_target.indices.len() == 2 && card_target.validate(game_state).is_ok()
        } else {
            false
        }
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        let Target::Cards(card_target) = target else {
            return Err(ConsumableError::InvalidTarget(
                "Death requires exactly 2 card targets".to_string(),
            ));
        };

        if card_target.indices.len() != 2 {
            return Err(ConsumableError::InvalidTarget(
                "Death requires exactly 2 cards".to_string(),
            ));
        }

        card_target.validate(game_state)?;

        // Convert left card (first index) to match right card (second index)
        let left_index = card_target.indices[0];
        let right_index = card_target.indices[1];

        // Get the right card to copy its properties
        let right_card = match card_target.collection {
            crate::consumables::CardCollection::Hand => {
                game_state.available.cards().get(right_index).copied()
            }
            crate::consumables::CardCollection::Deck => {
                game_state.deck.cards().get(right_index).copied()
            }
            crate::consumables::CardCollection::DiscardPile => {
                game_state.discarded.get(right_index).copied()
            }
            crate::consumables::CardCollection::PlayedCards => {
                return Err(ConsumableError::InvalidTarget(
                    "Cannot transform played cards".to_string(),
                ));
            }
        };

        let Some(right_card) = right_card else {
            return Err(ConsumableError::InvalidTarget(
                "Right card index out of bounds".to_string(),
            ));
        };

        // Now modify the left card to match the right card
        match card_target.collection {
            crate::consumables::CardCollection::Hand => {
                if let Some(left_card) = game_state.available.get_card_mut(left_index) {
                    // Copy all properties except the ID (keep original ID for tracking)
                    let original_id = left_card.id;
                    *left_card = right_card;
                    left_card.id = original_id;
                } else {
                    return Err(ConsumableError::InvalidTarget(
                        "Left card index out of bounds".to_string(),
                    ));
                }
            }
            crate::consumables::CardCollection::Deck => {
                if let Some(left_card) = game_state.deck.get_card_mut(left_index) {
                    // Copy all properties except the ID (keep original ID for tracking)
                    let original_id = left_card.id;
                    *left_card = right_card;
                    left_card.id = original_id;
                } else {
                    return Err(ConsumableError::InvalidTarget(
                        "Left card index out of bounds".to_string(),
                    ));
                }
            }
            crate::consumables::CardCollection::DiscardPile => {
                if let Some(left_card) = game_state.discarded.get_mut(left_index) {
                    // Copy all properties except the ID (keep original ID for tracking)
                    let original_id = left_card.id;
                    *left_card = right_card;
                    left_card.id = original_id;
                } else {
                    return Err(ConsumableError::InvalidTarget(
                        "Left card index out of bounds".to_string(),
                    ));
                }
            }
            crate::consumables::CardCollection::PlayedCards => {
                return Err(ConsumableError::InvalidTarget(
                    "Cannot transform played cards".to_string(),
                ));
            }
        }

        Ok(())
    }

    fn get_description(&self) -> String {
        "Select 2 cards, convert left card to right card".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(2)
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Modification
    }

    fn name(&self) -> &'static str {
        "Death"
    }

    fn description(&self) -> &'static str {
        "Select 2 cards, convert left card to right card"
    }
}

impl TarotCard for Death {
    fn id(&self) -> ConsumableId {
        ConsumableId::Death
    }

    fn arcana_number(&self) -> u8 {
        13
    }

    fn detailed_description(&self) -> String {
        "Death (XIII): Select 2 cards. The first card will be transformed to become an exact copy of the second card.".to_string()
    }
}

/// Temperance (XIV) - Gives the total sell value of all current Jokers
///
/// This is a utility card that converts joker value to money without destroying them.
/// Requires no targeting - affects all jokers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Temperance;

impl Temperance {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Temperance {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Temperance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Temperance")
    }
}

impl Consumable for Temperance {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, _game_state: &Game, target: &Target) -> bool {
        matches!(target, Target::None)
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        if !matches!(target, Target::None) {
            return Err(ConsumableError::InvalidTarget(
                "Temperance requires no target".to_string(),
            ));
        }

        // Calculate total sell value of all jokers and add to money
        let mut total_sell_value = 0.0;

        for joker in &game_state.jokers {
            // Get the joker's rarity and calculate its sell value
            let rarity = joker.rarity();
            let sell_value = match rarity {
                crate::joker::JokerRarity::Common => 3.0,
                crate::joker::JokerRarity::Uncommon => 6.0,
                crate::joker::JokerRarity::Rare => 8.0,
                crate::joker::JokerRarity::Legendary => 20.0,
            };
            total_sell_value += sell_value;
        }

        // Add the total sell value to the player's money
        game_state.money += total_sell_value;

        Ok(())
    }

    fn get_description(&self) -> String {
        "Gives the total sell value of all current Jokers".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::None
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Utility
    }

    fn name(&self) -> &'static str {
        "Temperance"
    }

    fn description(&self) -> &'static str {
        "Gives the total sell value of all current Jokers"
    }
}

impl TarotCard for Temperance {
    fn id(&self) -> ConsumableId {
        ConsumableId::Temperance
    }

    fn arcana_number(&self) -> u8 {
        14
    }

    fn detailed_description(&self) -> String {
        "Temperance (XIV): Grants money equal to the total sell value of all your current Jokers without destroying them.".to_string()
    }
}

/// The Devil (XV) - Enhances 1 selected card to Gold Card
///
/// Gold Cards provide money-based benefits.
/// This is a single-target enhancement card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheDevil;

impl TheDevil {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TheDevil {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TheDevil {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The Devil")
    }
}

impl Consumable for TheDevil {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        if let Target::Cards(card_target) = target {
            card_target.indices.len() == 1 && card_target.validate(game_state).is_ok()
        } else {
            false
        }
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        let Target::Cards(card_target) = target else {
            return Err(ConsumableError::InvalidTarget(
                "The Devil requires exactly 1 card target".to_string(),
            ));
        };

        if card_target.indices.len() != 1 {
            return Err(ConsumableError::InvalidTarget(
                "The Devil requires exactly 1 card".to_string(),
            ));
        }

        card_target.validate(game_state)?;

        // Apply Gold enhancement to the targeted card
        let index = card_target.indices[0];
        match card_target.collection {
            crate::consumables::CardCollection::Hand => {
                if let Some(card) = game_state.available.get_card_mut(index) {
                    card.enhancement = Some(crate::card::Enhancement::Gold);
                } else {
                    return Err(ConsumableError::InvalidTarget(
                        "Card index out of bounds".to_string(),
                    ));
                }
            }
            crate::consumables::CardCollection::Deck => {
                if let Some(card) = game_state.deck.get_card_mut(index) {
                    card.enhancement = Some(crate::card::Enhancement::Gold);
                } else {
                    return Err(ConsumableError::InvalidTarget(
                        "Card index out of bounds".to_string(),
                    ));
                }
            }
            crate::consumables::CardCollection::DiscardPile => {
                if let Some(card) = game_state.discarded.get_mut(index) {
                    card.enhancement = Some(crate::card::Enhancement::Gold);
                } else {
                    return Err(ConsumableError::InvalidTarget(
                        "Card index out of bounds".to_string(),
                    ));
                }
            }
            crate::consumables::CardCollection::PlayedCards => {
                return Err(ConsumableError::InvalidTarget(
                    "Cannot enhance played cards".to_string(),
                ));
            }
        }

        Ok(())
    }

    fn get_description(&self) -> String {
        "Enhances 1 selected card to Gold Card".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(1)
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn name(&self) -> &'static str {
        "The Devil"
    }

    fn description(&self) -> &'static str {
        "Enhances 1 selected card to Gold Card"
    }
}

impl TarotCard for TheDevil {
    fn id(&self) -> ConsumableId {
        ConsumableId::TheDevil
    }

    fn arcana_number(&self) -> u8 {
        15
    }

    fn detailed_description(&self) -> String {
        "The Devil (XV): Select 1 card to enhance with Gold. Gold cards provide money when played or used.".to_string()
    }
}

/// The Tower (XVI) - Enhances 1 selected card to Stone Card
///
/// Stone Cards provide defensive benefits.
/// This is a single-target enhancement card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheTower;

impl TheTower {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TheTower {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TheTower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The Tower")
    }
}

impl Consumable for TheTower {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        if let Target::Cards(card_target) = target {
            card_target.indices.len() == 1 && card_target.validate(game_state).is_ok()
        } else {
            false
        }
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        let Target::Cards(card_target) = target else {
            return Err(ConsumableError::InvalidTarget(
                "The Tower requires exactly 1 card target".to_string(),
            ));
        };

        if card_target.indices.len() != 1 {
            return Err(ConsumableError::InvalidTarget(
                "The Tower requires exactly 1 card".to_string(),
            ));
        }

        card_target.validate(game_state)?;

        // Apply Stone enhancement to the targeted card
        let index = card_target.indices[0];
        match card_target.collection {
            crate::consumables::CardCollection::Hand => {
                if let Some(card) = game_state.available.get_card_mut(index) {
                    card.enhancement = Some(crate::card::Enhancement::Stone);
                } else {
                    return Err(ConsumableError::InvalidTarget(
                        "Card index out of bounds".to_string(),
                    ));
                }
            }
            crate::consumables::CardCollection::Deck => {
                if let Some(card) = game_state.deck.get_card_mut(index) {
                    card.enhancement = Some(crate::card::Enhancement::Stone);
                } else {
                    return Err(ConsumableError::InvalidTarget(
                        "Card index out of bounds".to_string(),
                    ));
                }
            }
            crate::consumables::CardCollection::DiscardPile => {
                if let Some(card) = game_state.discarded.get_mut(index) {
                    card.enhancement = Some(crate::card::Enhancement::Stone);
                } else {
                    return Err(ConsumableError::InvalidTarget(
                        "Card index out of bounds".to_string(),
                    ));
                }
            }
            crate::consumables::CardCollection::PlayedCards => {
                return Err(ConsumableError::InvalidTarget(
                    "Cannot enhance played cards".to_string(),
                ));
            }
        }

        Ok(())
    }

    fn get_description(&self) -> String {
        "Enhances 1 selected card to Stone Card".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(1)
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn name(&self) -> &'static str {
        "The Tower"
    }

    fn description(&self) -> &'static str {
        "Enhances 1 selected card to Stone Card"
    }
}

impl TarotCard for TheTower {
    fn id(&self) -> ConsumableId {
        ConsumableId::TheTower
    }

    fn arcana_number(&self) -> u8 {
        16
    }

    fn detailed_description(&self) -> String {
        "The Tower (XVI): Select 1 card to enhance with Stone. Stone cards provide defensive benefits and cannot be destroyed.".to_string()
    }
}

/// The Star (XVII) - Converts up to 3 selected cards to Diamonds
///
/// This is a suit conversion card that changes cards to Diamond suit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheStar;

impl TheStar {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TheStar {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TheStar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The Star")
    }
}

impl Consumable for TheStar {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        if let Target::Cards(card_target) = target {
            let count = card_target.indices.len();
            (1..=3).contains(&count) && card_target.validate(game_state).is_ok()
        } else {
            false
        }
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        let Target::Cards(card_target) = target else {
            return Err(ConsumableError::InvalidTarget(
                "The Star requires card targets".to_string(),
            ));
        };

        let count = card_target.indices.len();
        if count == 0 || count > 3 {
            return Err(ConsumableError::InvalidTarget(
                "The Star requires 1-3 cards".to_string(),
            ));
        }

        card_target.validate(game_state)?;

        // Convert targeted cards to Diamond suit
        for &index in &card_target.indices {
            match card_target.collection {
                crate::consumables::CardCollection::Hand => {
                    if let Some(card) = game_state.available.get_card_mut(index) {
                        card.suit = crate::card::Suit::Diamond;
                    } else {
                        return Err(ConsumableError::InvalidTarget(
                            "Card index out of bounds".to_string(),
                        ));
                    }
                }
                crate::consumables::CardCollection::Deck => {
                    if let Some(card) = game_state.deck.get_card_mut(index) {
                        card.suit = crate::card::Suit::Diamond;
                    } else {
                        return Err(ConsumableError::InvalidTarget(
                            "Card index out of bounds".to_string(),
                        ));
                    }
                }
                crate::consumables::CardCollection::DiscardPile => {
                    if let Some(card) = game_state.discarded.get_mut(index) {
                        card.suit = crate::card::Suit::Diamond;
                    } else {
                        return Err(ConsumableError::InvalidTarget(
                            "Card index out of bounds".to_string(),
                        ));
                    }
                }
                crate::consumables::CardCollection::PlayedCards => {
                    return Err(ConsumableError::InvalidTarget(
                        "Cannot convert played cards".to_string(),
                    ));
                }
            }
        }

        Ok(())
    }

    fn get_description(&self) -> String {
        "Converts up to 3 selected cards to Diamonds".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(3) // Up to 3 cards
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Modification
    }

    fn name(&self) -> &'static str {
        "The Star"
    }

    fn description(&self) -> &'static str {
        "Converts up to 3 selected cards to Diamonds"
    }
}

impl TarotCard for TheStar {
    fn id(&self) -> ConsumableId {
        ConsumableId::TheStar
    }

    fn arcana_number(&self) -> u8 {
        17
    }

    fn detailed_description(&self) -> String {
        "The Star (XVII): Select up to 3 cards to convert to Diamond suit. Useful for building flush hands.".to_string()
    }
}

/// The Moon (XVIII) - Converts up to 3 selected cards to Clubs
///
/// This is a suit conversion card that changes cards to Club suit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheMoon;

impl TheMoon {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TheMoon {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TheMoon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The Moon")
    }
}

impl Consumable for TheMoon {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        if let Target::Cards(card_target) = target {
            let count = card_target.indices.len();
            (1..=3).contains(&count) && card_target.validate(game_state).is_ok()
        } else {
            false
        }
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        let Target::Cards(card_target) = target else {
            return Err(ConsumableError::InvalidTarget(
                "The Moon requires card targets".to_string(),
            ));
        };

        let count = card_target.indices.len();
        if count == 0 || count > 3 {
            return Err(ConsumableError::InvalidTarget(
                "The Moon requires 1-3 cards".to_string(),
            ));
        }

        card_target.validate(game_state)?;

        // Convert targeted cards to Club suit
        for &index in &card_target.indices {
            match card_target.collection {
                crate::consumables::CardCollection::Hand => {
                    if let Some(card) = game_state.available.get_card_mut(index) {
                        card.suit = crate::card::Suit::Club;
                    } else {
                        return Err(ConsumableError::InvalidTarget(
                            "Card index out of bounds".to_string(),
                        ));
                    }
                }
                crate::consumables::CardCollection::Deck => {
                    if let Some(card) = game_state.deck.get_card_mut(index) {
                        card.suit = crate::card::Suit::Club;
                    } else {
                        return Err(ConsumableError::InvalidTarget(
                            "Card index out of bounds".to_string(),
                        ));
                    }
                }
                crate::consumables::CardCollection::DiscardPile => {
                    if let Some(card) = game_state.discarded.get_mut(index) {
                        card.suit = crate::card::Suit::Club;
                    } else {
                        return Err(ConsumableError::InvalidTarget(
                            "Card index out of bounds".to_string(),
                        ));
                    }
                }
                crate::consumables::CardCollection::PlayedCards => {
                    return Err(ConsumableError::InvalidTarget(
                        "Cannot convert played cards".to_string(),
                    ));
                }
            }
        }

        Ok(())
    }

    fn get_description(&self) -> String {
        "Converts up to 3 selected cards to Clubs".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(3) // Up to 3 cards
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Modification
    }

    fn name(&self) -> &'static str {
        "The Moon"
    }

    fn description(&self) -> &'static str {
        "Converts up to 3 selected cards to Clubs"
    }
}

impl TarotCard for TheMoon {
    fn id(&self) -> ConsumableId {
        ConsumableId::TheMoon
    }

    fn arcana_number(&self) -> u8 {
        18
    }

    fn detailed_description(&self) -> String {
        "The Moon (XVIII): Select up to 3 cards to convert to Club suit. Useful for building flush hands.".to_string()
    }
}

/// The Sun (XIX) - Converts up to 3 selected cards to Hearts
///
/// This is a suit conversion card that changes cards to Heart suit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheSun;

impl TheSun {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TheSun {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TheSun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The Sun")
    }
}

impl Consumable for TheSun {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        if let Target::Cards(card_target) = target {
            let count = card_target.indices.len();
            (1..=3).contains(&count) && card_target.validate(game_state).is_ok()
        } else {
            false
        }
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        let Target::Cards(card_target) = target else {
            return Err(ConsumableError::InvalidTarget(
                "The Sun requires card targets".to_string(),
            ));
        };

        let count = card_target.indices.len();
        if count == 0 || count > 3 {
            return Err(ConsumableError::InvalidTarget(
                "The Sun requires 1-3 cards".to_string(),
            ));
        }

        card_target.validate(game_state)?;

        // Convert targeted cards to Heart suit
        for &index in &card_target.indices {
            match card_target.collection {
                crate::consumables::CardCollection::Hand => {
                    if let Some(card) = game_state.available.get_card_mut(index) {
                        card.suit = crate::card::Suit::Heart;
                    } else {
                        return Err(ConsumableError::InvalidTarget(
                            "Card index out of bounds".to_string(),
                        ));
                    }
                }
                crate::consumables::CardCollection::Deck => {
                    if let Some(card) = game_state.deck.get_card_mut(index) {
                        card.suit = crate::card::Suit::Heart;
                    } else {
                        return Err(ConsumableError::InvalidTarget(
                            "Card index out of bounds".to_string(),
                        ));
                    }
                }
                crate::consumables::CardCollection::DiscardPile => {
                    if let Some(card) = game_state.discarded.get_mut(index) {
                        card.suit = crate::card::Suit::Heart;
                    } else {
                        return Err(ConsumableError::InvalidTarget(
                            "Card index out of bounds".to_string(),
                        ));
                    }
                }
                crate::consumables::CardCollection::PlayedCards => {
                    return Err(ConsumableError::InvalidTarget(
                        "Cannot convert played cards".to_string(),
                    ));
                }
            }
        }

        Ok(())
    }

    fn get_description(&self) -> String {
        "Converts up to 3 selected cards to Hearts".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(3) // Up to 3 cards
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Modification
    }

    fn name(&self) -> &'static str {
        "The Sun"
    }

    fn description(&self) -> &'static str {
        "Converts up to 3 selected cards to Hearts"
    }
}

impl TarotCard for TheSun {
    fn id(&self) -> ConsumableId {
        ConsumableId::TheSun
    }

    fn arcana_number(&self) -> u8 {
        19
    }

    fn detailed_description(&self) -> String {
        "The Sun (XIX): Select up to 3 cards to convert to Heart suit. Useful for building flush hands.".to_string()
    }
}

/// Judgement (XX) - Creates a random Joker card
///
/// This is a generation card that adds a new joker to the game.
/// Requires space for a new joker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Judgement;

impl Judgement {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Judgement {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Judgement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Judgement")
    }
}

impl Consumable for Judgement {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        // Check if there's space for a new joker and no target required
        matches!(target, Target::None) && game_state.jokers.len() < 5 // Assuming 5 joker slots
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        if !matches!(target, Target::None) {
            return Err(ConsumableError::InvalidTarget(
                "Judgement requires no target".to_string(),
            ));
        }

        if game_state.jokers.len() >= 5 {
            return Err(ConsumableError::InsufficientResources);
        }

        // Create a random joker and add it to the game
        use crate::joker::JokerRarity;
        use crate::joker_factory::JokerFactory;

        // Define weighted rarity distribution for random joker creation
        let rarity_weights = [
            (70, JokerRarity::Common),
            (20, JokerRarity::Uncommon),
            (8, JokerRarity::Rare),
            (2, JokerRarity::Legendary),
        ];

        // Select random rarity based on weights
        let total_weight: u32 = rarity_weights.iter().map(|(weight, _)| weight).sum();
        let mut random_value = game_state.rng.gen_range(0..total_weight);

        let mut selected_rarity = JokerRarity::Common;
        for (weight, rarity) in rarity_weights.iter() {
            if random_value < *weight {
                selected_rarity = *rarity;
                break;
            }
            random_value -= weight;
        }

        // Get all jokers of the selected rarity
        let jokers_of_rarity = JokerFactory::get_by_rarity(selected_rarity);

        if jokers_of_rarity.is_empty() {
            return Err(ConsumableError::InsufficientResources);
        }

        // Select a random joker ID from the rarity
        let random_joker_id = *game_state.rng.choose(&jokers_of_rarity).unwrap();

        // Create the joker instance
        if let Some(joker) = JokerFactory::create(random_joker_id) {
            game_state.jokers.push(joker);
        } else {
            return Err(ConsumableError::InsufficientResources);
        }

        Ok(())
    }

    fn get_description(&self) -> String {
        "Creates a random Joker card".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::None
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Generation
    }

    fn name(&self) -> &'static str {
        "Judgement"
    }

    fn description(&self) -> &'static str {
        "Creates a random Joker card"
    }
}

impl TarotCard for Judgement {
    fn id(&self) -> ConsumableId {
        ConsumableId::Judgement
    }

    fn arcana_number(&self) -> u8 {
        20
    }

    fn detailed_description(&self) -> String {
        "Judgement (XX): Creates a random Joker card and adds it to your collection (must have room).".to_string()
    }
}

/// The World (XXI) - Converts up to 3 selected cards to Spades
///
/// This is a suit conversion card that changes cards to Spade suit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheWorld;

impl TheWorld {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TheWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TheWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The World")
    }
}

impl Consumable for TheWorld {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        if let Target::Cards(card_target) = target {
            let count = card_target.indices.len();
            (1..=3).contains(&count) && card_target.validate(game_state).is_ok()
        } else {
            false
        }
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        let Target::Cards(card_target) = target else {
            return Err(ConsumableError::InvalidTarget(
                "The World requires card targets".to_string(),
            ));
        };

        let count = card_target.indices.len();
        if count == 0 || count > 3 {
            return Err(ConsumableError::InvalidTarget(
                "The World requires 1-3 cards".to_string(),
            ));
        }

        card_target.validate(game_state)?;

        // Convert targeted cards to Spade suit
        for &index in &card_target.indices {
            match card_target.collection {
                crate::consumables::CardCollection::Hand => {
                    if let Some(card) = game_state.available.get_card_mut(index) {
                        card.suit = crate::card::Suit::Spade;
                    } else {
                        return Err(ConsumableError::InvalidTarget(
                            "Card index out of bounds".to_string(),
                        ));
                    }
                }
                crate::consumables::CardCollection::Deck => {
                    if let Some(card) = game_state.deck.get_card_mut(index) {
                        card.suit = crate::card::Suit::Spade;
                    } else {
                        return Err(ConsumableError::InvalidTarget(
                            "Card index out of bounds".to_string(),
                        ));
                    }
                }
                crate::consumables::CardCollection::DiscardPile => {
                    if let Some(card) = game_state.discarded.get_mut(index) {
                        card.suit = crate::card::Suit::Spade;
                    } else {
                        return Err(ConsumableError::InvalidTarget(
                            "Card index out of bounds".to_string(),
                        ));
                    }
                }
                crate::consumables::CardCollection::PlayedCards => {
                    return Err(ConsumableError::InvalidTarget(
                        "Cannot convert played cards".to_string(),
                    ));
                }
            }
        }

        Ok(())
    }

    fn get_description(&self) -> String {
        "Converts up to 3 selected cards to Spades".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(3) // Up to 3 cards
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Modification
    }

    fn name(&self) -> &'static str {
        "The World"
    }

    fn description(&self) -> &'static str {
        "Converts up to 3 selected cards to Spades"
    }
}

impl TarotCard for TheWorld {
    fn id(&self) -> ConsumableId {
        ConsumableId::TheWorld
    }

    fn arcana_number(&self) -> u8 {
        21
    }

    fn detailed_description(&self) -> String {
        "The World (XXI): Select up to 3 cards to convert to Spade suit. Useful for building flush hands.".to_string()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tarot_factory_creates_all_cards() {
        let implemented_cards = TarotFactory::get_implemented_cards();

        for card_id in implemented_cards {
            let card = TarotFactory::create(card_id);
            assert!(card.is_some(), "Failed to create card: {:?}", card_id);
        }
    }

    #[test]
    fn test_tarot_factory_returns_none_for_non_tarot() {
        assert!(TarotFactory::create(ConsumableId::Mercury).is_none());
        assert!(TarotFactory::create(ConsumableId::Familiar).is_none());
    }

    #[test]
    fn test_justice_requirements() {
        let justice = Justice::new();
        assert_eq!(justice.consumable_type(), ConsumableType::Tarot);
        assert_eq!(justice.get_target_type(), TargetType::Cards(1));
        assert_eq!(justice.get_effect_category(), ConsumableEffect::Enhancement);
        assert_eq!(justice.arcana_number(), 11);
    }

    #[test]
    fn test_temperance_no_target() {
        let temperance = Temperance::new();
        assert_eq!(temperance.get_target_type(), TargetType::None);
        assert_eq!(temperance.get_effect_category(), ConsumableEffect::Utility);
    }

    #[test]
    fn test_suit_conversion_cards() {
        let star = TheStar::new();
        let moon = TheMoon::new();
        let sun = TheSun::new();
        let world = TheWorld::new();

        // All should accept up to 3 cards and be modification effects
        assert_eq!(star.get_target_type(), TargetType::Cards(3));
        assert_eq!(star.get_effect_category(), ConsumableEffect::Modification);

        assert_eq!(moon.get_target_type(), TargetType::Cards(3));
        assert_eq!(moon.get_effect_category(), ConsumableEffect::Modification);

        assert_eq!(sun.get_target_type(), TargetType::Cards(3));
        assert_eq!(sun.get_effect_category(), ConsumableEffect::Modification);

        assert_eq!(world.get_target_type(), TargetType::Cards(3));
        assert_eq!(world.get_effect_category(), ConsumableEffect::Modification);
    }

    #[test]
    fn test_arcana_numbers_are_correct() {
        assert_eq!(Justice::new().arcana_number(), 11);
        assert_eq!(TheHangedMan::new().arcana_number(), 12);
        assert_eq!(Death::new().arcana_number(), 13);
        assert_eq!(Temperance::new().arcana_number(), 14);
        assert_eq!(TheDevil::new().arcana_number(), 15);
        assert_eq!(TheTower::new().arcana_number(), 16);
        assert_eq!(TheStar::new().arcana_number(), 17);
        assert_eq!(TheMoon::new().arcana_number(), 18);
        assert_eq!(TheSun::new().arcana_number(), 19);
        assert_eq!(Judgement::new().arcana_number(), 20);
        assert_eq!(TheWorld::new().arcana_number(), 21);
    }
}

/// Global factory instance access function
///
/// Returns a reference to the tarot factory for use throughout the codebase.
pub fn get_tarot_factory() -> &'static TarotFactory {
    static FACTORY: TarotFactory = TarotFactory;
    &FACTORY
}

/// Initialize the tarot factory system
///
/// Called during game initialization to set up the tarot card system.
/// Currently a no-op as the factory is stateless.
pub fn initialize_tarot_factory() {
    // No initialization needed for stateless factory
}
