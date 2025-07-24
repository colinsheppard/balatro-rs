//! Comprehensive unit tests for the JokerIdentity trait
//!
//! This module tests all aspects of the JokerIdentity trait, including:
//! - Unique type identifiers
//! - Name and description consistency
//! - Rarity validation
//! - Cost calculations
//! - Uniqueness flags

use super::traits::{JokerIdentity, Rarity};
use std::collections::HashSet;
use std::sync::Arc;

/// Mock implementation for testing JokerIdentity
struct MockJokerIdentity {
    joker_type: &'static str,
    name: String,
    description: String,
    rarity: Rarity,
    base_cost: u64,
    is_unique: bool,
}

impl MockJokerIdentity {
    fn new(joker_type: &'static str) -> Self {
        Self {
            joker_type,
            name: format!("Mock {}", joker_type),
            description: format!("A mock {} joker", joker_type),
            rarity: Rarity::Common,
            base_cost: 3,
            is_unique: false,
        }
    }

    fn with_rarity(mut self, rarity: Rarity) -> Self {
        self.rarity = rarity;
        // Update base cost according to rarity
        self.base_cost = match rarity {
            Rarity::Common => 3,
            Rarity::Uncommon => 6,
            Rarity::Rare => 8,
            Rarity::Legendary => 20,
        };
        self
    }

    fn with_custom_cost(mut self, cost: u64) -> Self {
        self.base_cost = cost;
        self
    }

    fn with_unique(mut self) -> Self {
        self.is_unique = true;
        self
    }

    fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

impl JokerIdentity for MockJokerIdentity {
    fn joker_type(&self) -> &'static str {
        self.joker_type
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
        self.base_cost
    }

    fn is_unique(&self) -> bool {
        self.is_unique
    }
}

#[cfg(test)]
mod contract_tests {
    use super::*;

    #[test]
    fn test_joker_type_uniqueness() {
        // Test that joker types should be unique identifiers
        let joker1 = MockJokerIdentity::new("test_joker_1");
        let joker2 = MockJokerIdentity::new("test_joker_2");
        let joker3 = MockJokerIdentity::new("test_joker_1"); // Duplicate type

        assert_ne!(joker1.joker_type(), joker2.joker_type());
        assert_eq!(joker1.joker_type(), joker3.joker_type());
    }

    #[test]
    fn test_joker_type_consistency() {
        // Test that joker_type() returns consistent values
        let joker = MockJokerIdentity::new("consistent_joker");

        let type1 = joker.joker_type();
        let type2 = joker.joker_type();
        let type3 = joker.joker_type();

        assert_eq!(type1, type2);
        assert_eq!(type2, type3);
        assert_eq!(type1, "consistent_joker");
    }

    #[test]
    fn test_name_and_description_non_empty() {
        // Test that name and description are never empty
        let joker = MockJokerIdentity::new("named_joker");

        assert!(!joker.name().is_empty());
        assert!(!joker.description().is_empty());
    }

    #[test]
    fn test_rarity_values_valid() {
        // Test all valid rarity values
        let rarities = vec![
            Rarity::Common,
            Rarity::Uncommon,
            Rarity::Rare,
            Rarity::Legendary,
        ];

        for rarity in rarities {
            let joker = MockJokerIdentity::new("rarity_test").with_rarity(rarity);
            match joker.rarity() {
                Rarity::Common | Rarity::Uncommon | Rarity::Rare | Rarity::Legendary => {
                    // Valid rarity
                }
            }
        }
    }

    #[test]
    fn test_base_cost_matches_rarity() {
        // Test that base cost aligns with rarity expectations
        let common = MockJokerIdentity::new("common").with_rarity(Rarity::Common);
        let uncommon = MockJokerIdentity::new("uncommon").with_rarity(Rarity::Uncommon);
        let rare = MockJokerIdentity::new("rare").with_rarity(Rarity::Rare);
        let legendary = MockJokerIdentity::new("legendary").with_rarity(Rarity::Legendary);

        assert_eq!(common.base_cost(), 3);
        assert_eq!(uncommon.base_cost(), 6);
        assert_eq!(rare.base_cost(), 8);
        assert_eq!(legendary.base_cost(), 20);
    }

    #[test]
    fn test_is_unique_default() {
        // Test that is_unique defaults to false
        let joker = MockJokerIdentity::new("default_unique");
        assert!(!joker.is_unique());
    }

    #[test]
    fn test_is_unique_legendary_relationship() {
        // Test that legendary jokers can be unique
        let legendary_unique = MockJokerIdentity::new("legendary_unique")
            .with_rarity(Rarity::Legendary)
            .with_unique();

        let legendary_normal =
            MockJokerIdentity::new("legendary_normal").with_rarity(Rarity::Legendary);

        assert!(legendary_unique.is_unique());
        assert!(!legendary_normal.is_unique());
    }

    #[test]
    fn test_trait_object_compatibility() {
        // Test that JokerIdentity can be used as a trait object
        let jokers: Vec<Box<dyn JokerIdentity>> = vec![
            Box::new(MockJokerIdentity::new("obj1")),
            Box::new(MockJokerIdentity::new("obj2").with_rarity(Rarity::Rare)),
            Box::new(MockJokerIdentity::new("obj3").with_unique()),
        ];

        for joker in &jokers {
            assert!(!joker.joker_type().is_empty());
            assert!(!joker.name().is_empty());
        }
    }

    #[test]
    fn test_send_sync_bounds() {
        // Test that JokerIdentity implements Send + Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Box<dyn JokerIdentity>>();

        // Test concurrent access
        let joker: Arc<dyn JokerIdentity> = Arc::new(MockJokerIdentity::new("concurrent"));
        let joker_clone = Arc::clone(&joker);

        std::thread::spawn(move || {
            assert_eq!(joker_clone.joker_type(), "concurrent");
        })
        .join()
        .unwrap();
    }
}

#[cfg(test)]
mod boundary_tests {
    use super::*;

    #[test]
    fn test_empty_strings_edge_case() {
        // Test behavior with empty strings (should not be allowed in practice)
        let joker = MockJokerIdentity::new("empty_test")
            .with_name(String::new())
            .with_description(String::new());

        // Even with empty strings set, the trait should handle it gracefully
        assert_eq!(joker.name(), "");
        assert_eq!(joker.description(), "");
    }

    #[test]
    fn test_very_long_strings() {
        // Test with very long strings
        let long_name = "A".repeat(1000);
        let long_desc = "B".repeat(10000);

        let joker = MockJokerIdentity::new("long_strings")
            .with_name(long_name.clone())
            .with_description(long_desc.clone());

        assert_eq!(joker.name(), long_name);
        assert_eq!(joker.description(), long_desc);
    }

    #[test]
    fn test_unicode_strings() {
        // Test with unicode characters
        let joker = MockJokerIdentity::new("unicode")
            .with_name("🃏 Joker 🎭".to_string())
            .with_description("This joker uses 🎲 dice and ♠️♥️♣️♦️ suits".to_string());

        assert_eq!(joker.name(), "🃏 Joker 🎭");
        assert!(joker.description().contains("🎲"));
    }

    #[test]
    fn test_cost_boundaries() {
        // Test extreme cost values
        let free_joker = MockJokerIdentity::new("free").with_custom_cost(0);
        let expensive_joker = MockJokerIdentity::new("expensive").with_custom_cost(u64::MAX);

        assert_eq!(free_joker.base_cost(), 0);
        assert_eq!(expensive_joker.base_cost(), u64::MAX);
    }

    #[test]
    fn test_multiple_jokers_same_type() {
        // Test multiple instances with same type (allowed but should be tracked)
        let jokers: Vec<Box<dyn JokerIdentity>> = vec![
            Box::new(MockJokerIdentity::new("duplicate")),
            Box::new(MockJokerIdentity::new("duplicate")),
            Box::new(MockJokerIdentity::new("duplicate")),
        ];

        let types: Vec<&str> = jokers.iter().map(|j| j.joker_type()).collect();
        assert_eq!(types, vec!["duplicate", "duplicate", "duplicate"]);
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;

    #[test]
    fn test_cost_rarity_invariant() {
        // Property: Higher rarity should generally mean higher cost
        let common = MockJokerIdentity::new("prop_common").with_rarity(Rarity::Common);
        let uncommon = MockJokerIdentity::new("prop_uncommon").with_rarity(Rarity::Uncommon);
        let rare = MockJokerIdentity::new("prop_rare").with_rarity(Rarity::Rare);
        let legendary = MockJokerIdentity::new("prop_legendary").with_rarity(Rarity::Legendary);

        assert!(common.base_cost() < uncommon.base_cost());
        assert!(uncommon.base_cost() < rare.base_cost());
        assert!(rare.base_cost() < legendary.base_cost());
    }

    #[test]
    fn test_unique_implies_special() {
        // Property: Unique jokers should have special characteristics
        let unique_joker = MockJokerIdentity::new("unique_special")
            .with_rarity(Rarity::Legendary)
            .with_unique();

        // Unique jokers should be at least rare
        match unique_joker.rarity() {
            Rarity::Common | Rarity::Uncommon => panic!("Unique jokers should be at least Rare"),
            Rarity::Rare | Rarity::Legendary => {} // OK
        }
    }

    #[test]
    fn test_type_name_consistency() {
        // Property: Joker type should be reflected in name (convention)
        let jokers = vec![
            MockJokerIdentity::new("test_consistency_1"),
            MockJokerIdentity::new("test_consistency_2"),
            MockJokerIdentity::new("test_consistency_3"),
        ];

        for joker in jokers {
            // Name should contain some reference to the type
            assert!(joker.name().to_lowercase().contains("mock"));
        }
    }

    #[test]
    fn test_collection_uniqueness() {
        // Property: In a collection, joker_type should be unique
        let mut type_set = HashSet::new();
        let jokers = vec![
            MockJokerIdentity::new("unique_1"),
            MockJokerIdentity::new("unique_2"),
            MockJokerIdentity::new("unique_3"),
            MockJokerIdentity::new("unique_4"),
        ];

        for joker in jokers {
            let was_inserted = type_set.insert(joker.joker_type());
            assert!(
                was_inserted,
                "Duplicate joker type found: {}",
                joker.joker_type()
            );
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_trait_composition_ready() {
        // Test that JokerIdentity can be composed with other traits
        trait ComposedJoker: JokerIdentity + Send + Sync {
            fn custom_method(&self) -> i32;
        }

        struct FullJoker {
            identity: MockJokerIdentity,
        }

        impl JokerIdentity for FullJoker {
            fn joker_type(&self) -> &'static str {
                self.identity.joker_type()
            }

            fn name(&self) -> &str {
                self.identity.name()
            }

            fn description(&self) -> &str {
                self.identity.description()
            }

            fn rarity(&self) -> Rarity {
                self.identity.rarity()
            }

            fn base_cost(&self) -> u64 {
                self.identity.base_cost()
            }

            fn is_unique(&self) -> bool {
                self.identity.is_unique()
            }
        }

        impl ComposedJoker for FullJoker {
            fn custom_method(&self) -> i32 {
                42
            }
        }

        let full_joker = FullJoker {
            identity: MockJokerIdentity::new("composed"),
        };

        assert_eq!(full_joker.joker_type(), "composed");
        assert_eq!(full_joker.custom_method(), 42);
    }

    #[test]
    fn test_sell_value_calculation() {
        // Test sell value calculation (usually base_cost / 2 + bonuses)
        let joker = MockJokerIdentity::new("sell_test").with_custom_cost(10);

        // Simulated sell value calculation
        let sell_value = |j: &dyn JokerIdentity, bonus: u64| -> u64 { j.base_cost() / 2 + bonus };

        assert_eq!(sell_value(&joker, 0), 5);
        assert_eq!(sell_value(&joker, 3), 8);
    }

    #[test]
    fn test_shop_display_information() {
        // Test that all information needed for shop display is available
        let jokers: Vec<Box<dyn JokerIdentity>> = vec![
            Box::new(MockJokerIdentity::new("shop1").with_rarity(Rarity::Common)),
            Box::new(MockJokerIdentity::new("shop2").with_rarity(Rarity::Rare)),
            Box::new(
                MockJokerIdentity::new("shop3")
                    .with_rarity(Rarity::Legendary)
                    .with_unique(),
            ),
        ];

        for joker in jokers {
            // All required shop info should be available
            assert!(!joker.name().is_empty());
            assert!(!joker.description().is_empty());
            assert!(joker.base_cost() > 0);

            // Rarity affects display
            match joker.rarity() {
                Rarity::Common => assert!(joker.base_cost() <= 5),
                Rarity::Uncommon => assert!(joker.base_cost() <= 10),
                Rarity::Rare => assert!(joker.base_cost() <= 15),
                Rarity::Legendary => assert!(joker.base_cost() >= 15),
            }
        }
    }
}

#[cfg(test)]
mod coverage_tests {
    use super::*;

    /// Helper to create various joker configurations for coverage
    fn create_test_jokers() -> Vec<Box<dyn JokerIdentity>> {
        vec![
            // Basic configurations
            Box::new(MockJokerIdentity::new("coverage_basic")),
            // All rarities
            Box::new(MockJokerIdentity::new("coverage_common").with_rarity(Rarity::Common)),
            Box::new(MockJokerIdentity::new("coverage_uncommon").with_rarity(Rarity::Uncommon)),
            Box::new(MockJokerIdentity::new("coverage_rare").with_rarity(Rarity::Rare)),
            Box::new(MockJokerIdentity::new("coverage_legendary").with_rarity(Rarity::Legendary)),
            // Unique variants
            Box::new(MockJokerIdentity::new("coverage_unique").with_unique()),
            Box::new(
                MockJokerIdentity::new("coverage_legendary_unique")
                    .with_rarity(Rarity::Legendary)
                    .with_unique(),
            ),
            // Custom costs
            Box::new(MockJokerIdentity::new("coverage_free").with_custom_cost(0)),
            Box::new(MockJokerIdentity::new("coverage_expensive").with_custom_cost(100)),
            // String variations
            Box::new(
                MockJokerIdentity::new("coverage_long_name")
                    .with_name("A Very Long Joker Name That Tests String Handling".to_string()),
            ),
            Box::new(
                MockJokerIdentity::new("coverage_special_chars")
                    .with_description("Special chars: !@#$%^&*()_+-=[]{}|;':\",./<>?".to_string()),
            ),
        ]
    }

    #[test]
    fn test_all_methods_called() {
        // Ensure all trait methods are exercised
        let jokers = create_test_jokers();

        for joker in jokers {
            // Call every method at least once
            let _ = joker.joker_type();
            let _ = joker.name();
            let _ = joker.description();
            let _ = joker.rarity();
            let _ = joker.base_cost();
            let _ = joker.is_unique();
        }
    }

    #[test]
    fn test_default_implementation_coverage() {
        // Test default implementation of is_unique
        struct MinimalJoker;

        impl JokerIdentity for MinimalJoker {
            fn joker_type(&self) -> &'static str {
                "minimal"
            }
            fn name(&self) -> &str {
                "Minimal"
            }
            fn description(&self) -> &str {
                "Minimal implementation"
            }
            fn rarity(&self) -> Rarity {
                Rarity::Common
            }
            fn base_cost(&self) -> u64 {
                1
            }
            // is_unique uses default implementation
        }

        let minimal = MinimalJoker;
        assert!(!minimal.is_unique()); // Should use default of false
    }

    #[test]
    fn test_edge_case_combinations() {
        // Test unusual but valid combinations
        let edge_cases = vec![
            // Common but unique (unusual)
            MockJokerIdentity::new("edge_common_unique")
                .with_rarity(Rarity::Common)
                .with_unique(),
            // Legendary but cheap
            MockJokerIdentity::new("edge_cheap_legendary")
                .with_rarity(Rarity::Legendary)
                .with_custom_cost(1),
            // Rare but free
            MockJokerIdentity::new("edge_free_rare")
                .with_rarity(Rarity::Rare)
                .with_custom_cost(0),
        ];

        for joker in edge_cases {
            // All combinations should work without panic
            assert!(!joker.joker_type().is_empty());
            let _ = format!(
                "{} - {} ({}g)",
                joker.name(),
                joker.description(),
                joker.base_cost()
            );
        }
    }
}

/// Module for testing real joker implementations once they implement JokerIdentity
#[cfg(test)]
mod real_joker_tests {
    use super::*;

    // Placeholder for testing actual joker implementations
    // These tests will be populated once real jokers implement JokerIdentity

    #[test]
    fn test_all_jokers_have_unique_types() {
        // TODO: Once real jokers implement JokerIdentity, test uniqueness
        let mut type_set = HashSet::new();

        // This will be populated with actual joker instances
        let jokers: Vec<Box<dyn JokerIdentity>> = vec![
            // Box::new(Joker::new()),
            // Box::new(GreedyJoker::new()),
            // etc...
        ];

        if !jokers.is_empty() {
            for joker in jokers {
                let joker_type = joker.joker_type();
                assert!(
                    type_set.insert(joker_type),
                    "Duplicate joker type found: {}",
                    joker_type
                );
            }
        }
    }
}
