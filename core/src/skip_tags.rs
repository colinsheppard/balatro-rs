//! Skip Tag System for Balatro
//!
//! Implements the skip blind reward system where players can skip blinds
//! to receive various reward tags with immediate or next-shop effects.

use crate::error::GameError;
use crate::game::Game;
use crate::joker::JokerRarity;
use crate::shop::packs::PackType;
use std::fmt;

/// Identifiers for different skip tags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyo3::pyclass(eq))]
pub enum TagId {
    // Immediate pack opening tags
    Charm,    // Free Mega Arcana Pack (choose 2 of 5)
    Ethereal, // Free Spectral Pack (choose 1 of 2)
    Buffoon,  // Free Mega Buffoon Pack (choose 2 of 5)
    Standard, // Free Mega Standard Pack (choose 2 of 5)
    Meteor,   // Free Mega Celestial Pack (choose 2 of 5)

    // Next shop modifier tags
    Rare,     // Next shop will have a free Rare Joker
    Uncommon, // Next shop will have a free Uncommon Joker

    // Immediate creation tags
    TopUp, // Create up to 2 Common Jokers immediately
}

impl fmt::Display for TagId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagId::Charm => write!(f, "Charm"),
            TagId::Ethereal => write!(f, "Ethereal"),
            TagId::Buffoon => write!(f, "Buffoon"),
            TagId::Standard => write!(f, "Standard"),
            TagId::Meteor => write!(f, "Meteor"),
            TagId::Rare => write!(f, "Rare"),
            TagId::Uncommon => write!(f, "Uncommon"),
            TagId::TopUp => write!(f, "Top-up"),
        }
    }
}

/// Types of tag effects for performance optimization
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagEffectType {
    /// Immediate reward that triggers when tag is received
    ImmediateReward,
    /// Modifier that affects the next shop
    NextShopModifier,
}

/// Configuration for reward pack tags
#[derive(Debug, Clone)]
pub struct RewardPackConfig {
    pub pack_type: PackType,
    pub choose_count: usize,
    pub total_options: usize,
}

impl RewardPackConfig {
    /// Create config for mega pack (2 of 5)
    pub fn mega_pack(pack_type: PackType) -> Self {
        Self {
            pack_type,
            choose_count: 2,
            total_options: 5,
        }
    }

    /// Create config for spectral pack (1 of 2)
    pub fn spectral_pack() -> Self {
        Self {
            pack_type: PackType::Spectral,
            choose_count: 1,
            total_options: 2,
        }
    }
}

/// Effect data for skip tags
#[derive(Debug, Clone)]
pub enum TagEffect {
    /// Open a reward pack immediately
    RewardPack(RewardPackConfig),
    /// Add a free joker to next shop
    NextShopFreeJoker(JokerRarity),
    /// Create jokers immediately (up to N, requires open slots)
    CreateJokers { count: usize, rarity: JokerRarity },
}

/// Core trait for skip tags - optimized for performance
pub trait SkipTag {
    /// Get the tag's unique identifier
    fn tag_id(&self) -> TagId;

    /// Get the tag's effect type for optimization
    fn effect_type(&self) -> TagEffectType;

    /// Get the tag's effect data
    fn effect(&self) -> TagEffect;

    /// Apply the tag effect to the game state
    /// Returns true if effect was applied successfully
    fn apply_effect(&self, game: &mut Game) -> Result<bool, GameError>;

    /// Get human-readable description of the tag
    fn description(&self) -> &'static str;

    /// Check if the tag can be applied in current game state
    fn can_apply(&self, game: &Game) -> bool;
}

/// Registry for skip tags - thread-safe and performance optimized
pub struct SkipTagRegistry {
    tags: std::collections::HashMap<TagId, Box<dyn SkipTag + Send + Sync>>,
}

impl fmt::Debug for SkipTagRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SkipTagRegistry")
            .field("tags", &self.tags.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl SkipTagRegistry {
    /// Create a new registry with all reward tags
    pub fn new() -> Self {
        let mut registry = Self {
            tags: std::collections::HashMap::new(),
        };

        // Register all reward tags
        registry.register_tag(Box::new(CharmTag));
        registry.register_tag(Box::new(EtherealTag));
        registry.register_tag(Box::new(BuffoonTag));
        registry.register_tag(Box::new(StandardTag));
        registry.register_tag(Box::new(MeteorTag));
        registry.register_tag(Box::new(RareTag));
        registry.register_tag(Box::new(UncommonTag));
        registry.register_tag(Box::new(TopUpTag));

        registry
    }

    /// Register a tag in the registry
    pub fn register_tag(&mut self, tag: Box<dyn SkipTag + Send + Sync>) {
        self.tags.insert(tag.tag_id(), tag);
    }

    /// Get a tag by ID
    pub fn get_tag(&self, tag_id: TagId) -> Option<&(dyn SkipTag + Send + Sync)> {
        self.tags.get(&tag_id).map(|tag| tag.as_ref())
    }

    /// Get all available tag IDs
    pub fn available_tags(&self) -> Vec<TagId> {
        self.tags.keys().copied().collect()
    }

    /// Apply a tag effect by ID
    pub fn apply_tag_effect(&self, tag_id: TagId, game: &mut Game) -> Result<bool, GameError> {
        if let Some(tag) = self.get_tag(tag_id) {
            tag.apply_effect(game)
        } else {
            Err(GameError::InvalidAction)
        }
    }
}

impl Default for SkipTagRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Tag implementations for the 8 reward tags

/// Charm Tag: Immediately open a free Mega Arcana Pack (choose 2 of 5 Tarot cards)
pub struct CharmTag;

impl SkipTag for CharmTag {
    fn tag_id(&self) -> TagId {
        TagId::Charm
    }
    fn effect_type(&self) -> TagEffectType {
        TagEffectType::ImmediateReward
    }
    fn effect(&self) -> TagEffect {
        TagEffect::RewardPack(RewardPackConfig::mega_pack(PackType::MegaArcana))
    }
    fn description(&self) -> &'static str {
        "Immediately open a free Mega Arcana Pack (choose 2 of 5 Tarot cards)"
    }
    fn can_apply(&self, _game: &Game) -> bool {
        true
    }
    fn apply_effect(&self, _game: &mut Game) -> Result<bool, GameError> {
        // Implementation will be added when integrating with pack system
        Ok(true)
    }
}

/// Ethereal Tag: Immediately open a free Spectral Pack (choose 1 of 2 Spectral cards)
pub struct EtherealTag;

impl SkipTag for EtherealTag {
    fn tag_id(&self) -> TagId {
        TagId::Ethereal
    }
    fn effect_type(&self) -> TagEffectType {
        TagEffectType::ImmediateReward
    }
    fn effect(&self) -> TagEffect {
        TagEffect::RewardPack(RewardPackConfig::spectral_pack())
    }
    fn description(&self) -> &'static str {
        "Immediately open a free Spectral Pack (choose 1 of 2 Spectral cards)"
    }
    fn can_apply(&self, _game: &Game) -> bool {
        true
    }
    fn apply_effect(&self, _game: &mut Game) -> Result<bool, GameError> {
        // Implementation will be added when integrating with pack system
        Ok(true)
    }
}

/// Buffoon Tag: Immediately open a free Mega Buffoon Pack (choose 2 of 5 Jokers)
pub struct BuffoonTag;

impl SkipTag for BuffoonTag {
    fn tag_id(&self) -> TagId {
        TagId::Buffoon
    }
    fn effect_type(&self) -> TagEffectType {
        TagEffectType::ImmediateReward
    }
    fn effect(&self) -> TagEffect {
        TagEffect::RewardPack(RewardPackConfig::mega_pack(PackType::MegaBuffoon))
    }
    fn description(&self) -> &'static str {
        "Immediately open a free Mega Buffoon Pack (choose 2 of 5 Jokers)"
    }
    fn can_apply(&self, _game: &Game) -> bool {
        true
    }
    fn apply_effect(&self, _game: &mut Game) -> Result<bool, GameError> {
        // Implementation will be added when integrating with pack system
        Ok(true)
    }
}

/// Standard Tag: Immediately open a free Mega Standard Pack (choose 2 of 5 playing cards)
pub struct StandardTag;

impl SkipTag for StandardTag {
    fn tag_id(&self) -> TagId {
        TagId::Standard
    }
    fn effect_type(&self) -> TagEffectType {
        TagEffectType::ImmediateReward
    }
    fn effect(&self) -> TagEffect {
        TagEffect::RewardPack(RewardPackConfig::mega_pack(PackType::Mega))
    }
    fn description(&self) -> &'static str {
        "Immediately open a free Mega Standard Pack (choose 2 of 5 playing cards)"
    }
    fn can_apply(&self, _game: &Game) -> bool {
        true
    }
    fn apply_effect(&self, _game: &mut Game) -> Result<bool, GameError> {
        // Implementation will be added when integrating with pack system
        Ok(true)
    }
}

/// Meteor Tag: Immediately open a free Mega Celestial Pack (choose 2 of 5 Planet cards)
pub struct MeteorTag;

impl SkipTag for MeteorTag {
    fn tag_id(&self) -> TagId {
        TagId::Meteor
    }
    fn effect_type(&self) -> TagEffectType {
        TagEffectType::ImmediateReward
    }
    fn effect(&self) -> TagEffect {
        TagEffect::RewardPack(RewardPackConfig::mega_pack(PackType::MegaCelestial))
    }
    fn description(&self) -> &'static str {
        "Immediately open a free Mega Celestial Pack (choose 2 of 5 Planet cards)"
    }
    fn can_apply(&self, _game: &Game) -> bool {
        true
    }
    fn apply_effect(&self, _game: &mut Game) -> Result<bool, GameError> {
        // Implementation will be added when integrating with pack system
        Ok(true)
    }
}

/// Rare Tag: Next shop will have a free Rare Joker
pub struct RareTag;

impl SkipTag for RareTag {
    fn tag_id(&self) -> TagId {
        TagId::Rare
    }
    fn effect_type(&self) -> TagEffectType {
        TagEffectType::NextShopModifier
    }
    fn effect(&self) -> TagEffect {
        TagEffect::NextShopFreeJoker(JokerRarity::Rare)
    }
    fn description(&self) -> &'static str {
        "Next shop will have a free Rare Joker"
    }
    fn can_apply(&self, _game: &Game) -> bool {
        true
    }
    fn apply_effect(&self, _game: &mut Game) -> Result<bool, GameError> {
        // Implementation will be added when integrating with shop system
        Ok(true)
    }
}

/// Uncommon Tag: Next shop will have a free Uncommon Joker
pub struct UncommonTag;

impl SkipTag for UncommonTag {
    fn tag_id(&self) -> TagId {
        TagId::Uncommon
    }
    fn effect_type(&self) -> TagEffectType {
        TagEffectType::NextShopModifier
    }
    fn effect(&self) -> TagEffect {
        TagEffect::NextShopFreeJoker(JokerRarity::Uncommon)
    }
    fn description(&self) -> &'static str {
        "Next shop will have a free Uncommon Joker"
    }
    fn can_apply(&self, _game: &Game) -> bool {
        true
    }
    fn apply_effect(&self, _game: &mut Game) -> Result<bool, GameError> {
        // Implementation will be added when integrating with shop system
        Ok(true)
    }
}

/// Top-up Tag: Create up to 2 Common Jokers immediately (requires open Joker slots)
pub struct TopUpTag;

impl SkipTag for TopUpTag {
    fn tag_id(&self) -> TagId {
        TagId::TopUp
    }
    fn effect_type(&self) -> TagEffectType {
        TagEffectType::ImmediateReward
    }
    fn effect(&self) -> TagEffect {
        TagEffect::CreateJokers {
            count: 2,
            rarity: JokerRarity::Common,
        }
    }
    fn description(&self) -> &'static str {
        "Create up to 2 Common Jokers immediately (requires open Joker slots)"
    }
    fn can_apply(&self, game: &Game) -> bool {
        // Check if there are open joker slots
        game.jokers.len() < game.config.joker_slots
    }
    fn apply_effect(&self, _game: &mut Game) -> Result<bool, GameError> {
        // Implementation will be added when integrating with joker system
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_tag_registry() {
        let registry = SkipTagRegistry::new();

        // Test all 8 reward tags are registered
        assert_eq!(registry.available_tags().len(), 8);

        // Test specific tags exist
        assert!(registry.get_tag(TagId::Charm).is_some());
        assert!(registry.get_tag(TagId::Ethereal).is_some());
        assert!(registry.get_tag(TagId::Buffoon).is_some());
        assert!(registry.get_tag(TagId::Standard).is_some());
        assert!(registry.get_tag(TagId::Meteor).is_some());
        assert!(registry.get_tag(TagId::Rare).is_some());
        assert!(registry.get_tag(TagId::Uncommon).is_some());
        assert!(registry.get_tag(TagId::TopUp).is_some());
    }

    #[test]
    fn test_tag_effect_types() {
        let registry = SkipTagRegistry::new();

        // Test immediate reward tags
        let immediate_tags = [
            TagId::Charm,
            TagId::Ethereal,
            TagId::Buffoon,
            TagId::Standard,
            TagId::Meteor,
            TagId::TopUp,
        ];
        for tag_id in immediate_tags {
            let tag = registry.get_tag(tag_id).unwrap();
            assert_eq!(tag.effect_type(), TagEffectType::ImmediateReward);
        }

        // Test next shop modifier tags
        let shop_modifier_tags = [TagId::Rare, TagId::Uncommon];
        for tag_id in shop_modifier_tags {
            let tag = registry.get_tag(tag_id).unwrap();
            assert_eq!(tag.effect_type(), TagEffectType::NextShopModifier);
        }
    }

    #[test]
    fn test_reward_pack_configs() {
        // Test mega pack config (2 of 5)
        let mega_config = RewardPackConfig::mega_pack(PackType::MegaArcana);
        assert_eq!(mega_config.choose_count, 2);
        assert_eq!(mega_config.total_options, 5);

        // Test spectral pack config (1 of 2)
        let spectral_config = RewardPackConfig::spectral_pack();
        assert_eq!(spectral_config.choose_count, 1);
        assert_eq!(spectral_config.total_options, 2);
    }
}
