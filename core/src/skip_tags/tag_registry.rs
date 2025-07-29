//! Skip Tag Registry
//!
//! Central registry for all skip tags with thread-safe access

use super::{SkipTag, SkipTagId, TagRarity};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Thread-safe registry for all skip tags
#[derive(Debug)]
pub struct SkipTagRegistry {
    tags: RwLock<HashMap<SkipTagId, Arc<dyn SkipTag>>>,
}

impl SkipTagRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            tags: RwLock::new(HashMap::new()),
        }
    }

    /// Register a skip tag
    pub fn register<T: SkipTag + 'static>(&self, tag: T) -> Result<(), String> {
        let id = tag.id();
        let tag_arc = Arc::new(tag);
        
        let mut tags = self.tags.write()
            .map_err(|_| "Failed to acquire write lock on skip tag registry")?;
        
        if tags.contains_key(&id) {
            return Err(format!("Skip tag {} is already registered", id));
        }
        
        tags.insert(id, tag_arc);
        Ok(())
    }

    /// Get a skip tag by ID
    pub fn get_tag(&self, id: SkipTagId) -> Option<Arc<dyn SkipTag>> {
        let tags = self.tags.read().ok()?;
        tags.get(&id).cloned()
    }

    /// Get all registered tag IDs
    pub fn get_all_ids(&self) -> Vec<SkipTagId> {
        let tags = self.tags.read().unwrap_or_else(|e| e.into_inner());
        tags.keys().copied().collect()
    }

    /// Get tags by rarity
    pub fn get_tags_by_rarity(&self, rarity: TagRarity) -> Vec<SkipTagId> {
        let tags = self.tags.read().unwrap_or_else(|e| e.into_inner());
        tags.values()
            .filter(|tag| tag.rarity() == rarity)
            .map(|tag| tag.id())
            .collect()
    }

    /// Get weighted tag selection (for random generation)
    pub fn get_weighted_tags(&self) -> Vec<(SkipTagId, f64)> {
        let tags = self.tags.read().unwrap_or_else(|e| e.into_inner());
        tags.values()
            .map(|tag| (tag.id(), tag.rarity().weight()))
            .collect()
    }

    /// Check if a tag is registered
    pub fn is_registered(&self, id: SkipTagId) -> bool {
        let tags = self.tags.read().unwrap_or_else(|e| e.into_inner());
        tags.contains_key(&id)
    }

    /// Get the number of registered tags
    pub fn count(&self) -> usize {
        let tags = self.tags.read().unwrap_or_else(|e| e.into_inner());
        tags.len()
    }
}

impl Default for SkipTagRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global registry instance
static GLOBAL_REGISTRY: std::sync::OnceLock<SkipTagRegistry> = std::sync::OnceLock::new();

/// Get the global skip tag registry
pub fn global_registry() -> &'static SkipTagRegistry {
    GLOBAL_REGISTRY.get_or_init(|| {
        let registry = SkipTagRegistry::new();
        
        // Register all utility tags
        if let Err(e) = registry.register(crate::skip_tags::utility_tags::DoubleTag) {
            eprintln!("Failed to register Double tag: {}", e);
        }
        if let Err(e) = registry.register(crate::skip_tags::utility_tags::BossTag) {
            eprintln!("Failed to register Boss tag: {}", e);
        }
        if let Err(e) = registry.register(crate::skip_tags::utility_tags::OrbitalTag) {
            eprintln!("Failed to register Orbital tag: {}", e);
        }
        if let Err(e) = registry.register(crate::skip_tags::utility_tags::JuggleTag) {
            eprintln!("Failed to register Juggle tag: {}", e);
        }
        
        registry
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skip_tags::{SkipTagContext, SkipTagResult, TagEffectType};

    // Mock tag for testing
    #[derive(Debug)]
    struct MockTag {
        id: SkipTagId,
        rarity: TagRarity,
    }

    impl SkipTag for MockTag {
        fn id(&self) -> SkipTagId {
            self.id
        }

        fn name(&self) -> &'static str {
            "Mock Tag"
        }

        fn description(&self) -> &'static str {
            "A mock tag for testing"
        }

        fn effect_type(&self) -> TagEffectType {
            TagEffectType::ImmediateReward
        }

        fn rarity(&self) -> TagRarity {
            self.rarity
        }

        fn stackable(&self) -> bool {
            false
        }

        fn activate(&self, context: SkipTagContext) -> SkipTagResult {
            SkipTagResult {
                game: context.game,
                additional_tags: vec![],
                success: true,
                message: Some("Mock activation".to_string()),
            }
        }
    }

    #[test]
    fn test_registry_creation() {
        let registry = SkipTagRegistry::new();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_tag_registration() {
        let registry = SkipTagRegistry::new();
        let mock_tag = MockTag {
            id: SkipTagId::Double,
            rarity: TagRarity::Common,
        };

        // Register tag
        assert!(registry.register(mock_tag).is_ok());
        assert_eq!(registry.count(), 1);
        assert!(registry.is_registered(SkipTagId::Double));

        // Try to register duplicate
        let duplicate_tag = MockTag {
            id: SkipTagId::Double,
            rarity: TagRarity::Rare,
        };
        assert!(registry.register(duplicate_tag).is_err());
    }

    #[test]
    fn test_tag_retrieval() {
        let registry = SkipTagRegistry::new();
        let mock_tag = MockTag {
            id: SkipTagId::Boss,
            rarity: TagRarity::Uncommon,
        };

        registry.register(mock_tag).unwrap();

        let retrieved = registry.get_tag(SkipTagId::Boss);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id(), SkipTagId::Boss);

        let missing = registry.get_tag(SkipTagId::Orbital);
        assert!(missing.is_none());
    }

    #[test]
    fn test_rarity_filtering() {
        let registry = SkipTagRegistry::new();
        
        registry.register(MockTag {
            id: SkipTagId::Double,
            rarity: TagRarity::Common,
        }).unwrap();
        
        registry.register(MockTag {
            id: SkipTagId::Boss,
            rarity: TagRarity::Rare,
        }).unwrap();

        let common_tags = registry.get_tags_by_rarity(TagRarity::Common);
        assert_eq!(common_tags.len(), 1);
        assert!(common_tags.contains(&SkipTagId::Double));

        let rare_tags = registry.get_tags_by_rarity(TagRarity::Rare);
        assert_eq!(rare_tags.len(), 1);
        assert!(rare_tags.contains(&SkipTagId::Boss));

        let legendary_tags = registry.get_tags_by_rarity(TagRarity::Legendary);
        assert_eq!(legendary_tags.len(), 0);
    }

    #[test]
    fn test_weighted_tags() {
        let registry = SkipTagRegistry::new();
        
        registry.register(MockTag {
            id: SkipTagId::Double,
            rarity: TagRarity::Common,
        }).unwrap();
        
        registry.register(MockTag {
            id: SkipTagId::Boss,
            rarity: TagRarity::Legendary,
        }).unwrap();

        let weighted = registry.get_weighted_tags();
        assert_eq!(weighted.len(), 2);
        
        // Common should have higher weight than legendary
        let double_weight = weighted.iter()
            .find(|(id, _)| *id == SkipTagId::Double)
            .map(|(_, weight)| *weight)
            .unwrap();
        let boss_weight = weighted.iter()
            .find(|(id, _)| *id == SkipTagId::Boss)
            .map(|(_, weight)| *weight)
            .unwrap();
        
        assert!(double_weight > boss_weight);
    }
}