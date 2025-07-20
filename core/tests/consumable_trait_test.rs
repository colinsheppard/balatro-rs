use balatro_rs::consumables::{
    Consumable, ConsumableEffect, ConsumableError, ConsumableType, Target, TargetType,
};
use balatro_rs::game::Game;

#[test]
fn test_consumable_trait_object_compatibility() {
    #[derive(Debug)]
    struct MockConsumable;

    impl Consumable for MockConsumable {
        fn consumable_type(&self) -> ConsumableType {
            ConsumableType::Tarot
        }

        fn can_use(&self, _game_state: &Game, _target: &Target) -> bool {
            true
        }

        fn use_effect(
            &self,
            _game_state: &mut Game,
            _target: Target,
        ) -> Result<(), ConsumableError> {
            Ok(())
        }

        fn get_description(&self) -> String {
            "Mock consumable for testing".to_string()
        }

        fn get_target_type(&self) -> TargetType {
            TargetType::None
        }

        fn get_effect_category(&self) -> ConsumableEffect {
            ConsumableEffect::Enhancement
        }
    }

    // Test trait object compatibility with Send + Sync + Debug bounds
    let consumable: Box<dyn Consumable + Send + Sync> = Box::new(MockConsumable);

    // Test basic trait methods work through trait object
    assert_eq!(consumable.consumable_type(), ConsumableType::Tarot);
    assert_eq!(consumable.get_target_type(), TargetType::None);
    assert_eq!(
        consumable.get_effect_category(),
        ConsumableEffect::Enhancement
    );
    assert_eq!(consumable.get_description(), "Mock consumable for testing");
}

#[test]
fn test_consumable_effect_enum_categories() {
    // Test all effect categories are properly defined
    let enhancement = ConsumableEffect::Enhancement;
    let destruction = ConsumableEffect::Destruction;
    let generation = ConsumableEffect::Generation;
    let modification = ConsumableEffect::Modification;
    let utility = ConsumableEffect::Utility;

    // Test Debug implementation
    assert!(format!("{:?}", enhancement).contains("Enhancement"));
    assert!(format!("{:?}", destruction).contains("Destruction"));
    assert!(format!("{:?}", generation).contains("Generation"));
    assert!(format!("{:?}", modification).contains("Modification"));
    assert!(format!("{:?}", utility).contains("Utility"));
}

#[test]
fn test_target_type_definitions() {
    // Test all target types are properly defined
    let none = TargetType::None;
    let cards = TargetType::Cards(2);
    let hand_type = TargetType::HandType;
    let joker = TargetType::Joker;
    let deck = TargetType::Deck;
    let shop = TargetType::Shop;

    // Test Debug implementation
    assert!(format!("{:?}", none).contains("None"));
    assert!(format!("{:?}", cards).contains("Cards"));
    assert!(format!("{:?}", hand_type).contains("HandType"));
    assert!(format!("{:?}", joker).contains("Joker"));
    assert!(format!("{:?}", deck).contains("Deck"));
    assert!(format!("{:?}", shop).contains("Shop"));
}

#[test]
fn test_target_validation() {
    let game = Game::default();

    // Test different target types
    let no_target = Target::None;
    let card_targets = Target::Cards(vec![0, 1]);
    let hand_target = Target::HandType(balatro_rs::rank::HandRank::OnePair);
    let joker_target = Target::Joker(0);
    let deck_target = Target::Deck;
    let shop_target = Target::Shop(2);

    assert!(matches!(no_target, Target::None));
    assert!(matches!(card_targets, Target::Cards(_)));
    assert!(matches!(hand_target, Target::HandType(_)));
    assert!(matches!(joker_target, Target::Joker(_)));
    assert!(matches!(deck_target, Target::Deck));
    assert!(matches!(shop_target, Target::Shop(_)));

    // Test validation methods
    assert!(no_target.is_valid(&game));
    // Card targets would need game state validation
    assert!(hand_target.is_valid(&game));
    assert!(deck_target.is_valid(&game));
    assert!(shop_target.is_valid(&game));
}

#[test]
fn test_consumable_error_types() {
    // Test error type definitions
    let invalid_target = ConsumableError::InvalidTarget("Mock error".to_string());
    let insufficient_resources = ConsumableError::InsufficientResources;
    let invalid_game_state = ConsumableError::InvalidGameState("Mock state error".to_string());
    let effect_failed = ConsumableError::EffectFailed("Mock effect error".to_string());

    // Test error display
    assert!(invalid_target.to_string().contains("Invalid target"));
    assert!(insufficient_resources
        .to_string()
        .contains("Insufficient resources"));
    assert!(invalid_game_state
        .to_string()
        .contains("Invalid game state"));
    assert!(effect_failed.to_string().contains("Effect failed"));
}

#[test]
fn test_consumable_type_existing_variants() {
    // Test existing ConsumableType variants still work
    let tarot = ConsumableType::Tarot;
    let planet = ConsumableType::Planet;
    let spectral = ConsumableType::Spectral;

    assert_eq!(tarot.to_string(), "Tarot");
    assert_eq!(planet.to_string(), "Planet");
    assert_eq!(spectral.to_string(), "Spectral");
}

// TODO: Add async test when tokio dependency is available
// #[tokio::test]
// async fn test_async_effect_application() {
//     // Async support will be added in future iterations
// }

#[test]
fn test_enhanced_consumable_trait_methods() {
    #[derive(Debug)]
    struct EnhancedMockConsumable;

    impl Consumable for EnhancedMockConsumable {
        fn consumable_type(&self) -> ConsumableType {
            ConsumableType::Planet
        }

        fn can_use(&self, game_state: &Game, target: &Target) -> bool {
            // Mock validation logic
            match target {
                Target::None => true,
                Target::Cards(cards) => {
                    !cards.is_empty() && cards.len() <= game_state.available.cards().len()
                }
                Target::HandType(_) => true,
                Target::Joker(_) => game_state.jokers.len() > 0,
                Target::Deck => true,
                Target::Shop(_) => true,
            }
        }

        fn use_effect(
            &self,
            _game_state: &mut Game,
            _target: Target,
        ) -> Result<(), ConsumableError> {
            Ok(())
        }

        fn get_description(&self) -> String {
            "Enhanced mock consumable with validation".to_string()
        }

        fn get_target_type(&self) -> TargetType {
            TargetType::HandType
        }

        fn get_effect_category(&self) -> ConsumableEffect {
            ConsumableEffect::Modification
        }
    }

    let consumable = EnhancedMockConsumable;
    let game = Game::default();

    // Test validation methods
    assert!(consumable.can_use(&game, &Target::None));
    assert!(consumable.can_use(
        &game,
        &Target::HandType(balatro_rs::rank::HandRank::OnePair)
    ));

    // Test metadata methods
    assert_eq!(consumable.get_target_type(), TargetType::HandType);
    assert_eq!(
        consumable.get_effect_category(),
        ConsumableEffect::Modification
    );
    assert!(consumable.get_description().contains("Enhanced mock"));
}

#[test]
fn test_target_type_method() {
    // Test target_type() method for all variants
    assert_eq!(Target::None.target_type(), TargetType::None);
    assert_eq!(
        Target::Cards(vec![0, 1, 2]).target_type(),
        TargetType::Cards(3)
    );
    assert_eq!(Target::Cards(vec![]).target_type(), TargetType::Cards(0));
    assert_eq!(
        Target::HandType(balatro_rs::rank::HandRank::OnePair).target_type(),
        TargetType::HandType
    );
    assert_eq!(Target::Joker(5).target_type(), TargetType::Joker);
    assert_eq!(Target::Deck.target_type(), TargetType::Deck);
    assert_eq!(Target::Shop(3).target_type(), TargetType::Shop);
}

#[test]
fn test_target_is_valid_type_method() {
    // Test is_valid_type() method for matching types
    assert!(Target::None.is_valid_type(TargetType::None));
    assert!(Target::Cards(vec![0, 1]).is_valid_type(TargetType::Cards(2)));
    assert!(Target::Cards(vec![]).is_valid_type(TargetType::Cards(0)));
    assert!(
        Target::HandType(balatro_rs::rank::HandRank::FullHouse).is_valid_type(TargetType::HandType)
    );
    assert!(Target::Joker(0).is_valid_type(TargetType::Joker));
    assert!(Target::Deck.is_valid_type(TargetType::Deck));
    assert!(Target::Shop(2).is_valid_type(TargetType::Shop));

    // Test is_valid_type() method for mismatched types
    assert!(!Target::None.is_valid_type(TargetType::Cards(1)));
    assert!(!Target::Cards(vec![0]).is_valid_type(TargetType::Cards(2))); // Wrong count
    assert!(!Target::Cards(vec![0, 1]).is_valid_type(TargetType::HandType));
    assert!(!Target::HandType(balatro_rs::rank::HandRank::OnePair).is_valid_type(TargetType::Joker));
    assert!(!Target::Joker(0).is_valid_type(TargetType::Deck));
    assert!(!Target::Deck.is_valid_type(TargetType::Shop));
    assert!(!Target::Shop(1).is_valid_type(TargetType::None));
}

#[test]
fn test_target_card_count_method() {
    // Test card_count() method for all variants
    assert_eq!(Target::None.card_count(), 0);
    assert_eq!(Target::Cards(vec![]).card_count(), 0);
    assert_eq!(Target::Cards(vec![0]).card_count(), 1);
    assert_eq!(Target::Cards(vec![0, 1, 2]).card_count(), 3);
    assert_eq!(
        Target::HandType(balatro_rs::rank::HandRank::Straight).card_count(),
        0
    );
    assert_eq!(Target::Joker(10).card_count(), 0);
    assert_eq!(Target::Deck.card_count(), 0);
    assert_eq!(Target::Shop(5).card_count(), 0);
}

#[test]
fn test_target_all_variants_comprehensive() {
    use balatro_rs::rank::HandRank;

    // Test all Target variants can be created and used
    let targets = vec![
        Target::None,
        Target::Cards(vec![]),
        Target::Cards(vec![0, 1, 2]),
        Target::HandType(HandRank::HighCard),
        Target::HandType(HandRank::OnePair),
        Target::HandType(HandRank::TwoPair),
        Target::HandType(HandRank::ThreeOfAKind),
        Target::HandType(HandRank::Straight),
        Target::HandType(HandRank::Flush),
        Target::HandType(HandRank::FullHouse),
        Target::HandType(HandRank::FourOfAKind),
        Target::HandType(HandRank::StraightFlush),
        Target::HandType(HandRank::RoyalFlush),
        Target::HandType(HandRank::FiveOfAKind),
        Target::HandType(HandRank::FlushHouse),
        Target::HandType(HandRank::FlushFive),
        Target::Joker(0),
        Target::Joker(5),
        Target::Deck,
        Target::Shop(0),
        Target::Shop(1),
        Target::Shop(5),
    ];

    // Test all variants can be debugged
    for target in &targets {
        let debug_string = format!("{:?}", target);
        assert!(!debug_string.is_empty());
    }

    // Test all variants have consistent target_type() -> is_valid_type() behavior
    for target in &targets {
        let target_type = target.target_type();
        assert!(target.is_valid_type(target_type));
    }
}

#[test]
fn test_target_type_all_variants_comprehensive() {
    // Test all TargetType variants
    let target_types = vec![
        TargetType::None,
        TargetType::Cards(0),
        TargetType::Cards(1),
        TargetType::Cards(5),
        TargetType::HandType,
        TargetType::Joker,
        TargetType::Deck,
        TargetType::Shop,
    ];

    // Test all variants can be debugged
    for target_type in &target_types {
        let debug_string = format!("{:?}", target_type);
        assert!(!debug_string.is_empty());
    }

    // Test Hash and Eq traits work properly
    use std::collections::HashSet;
    let set: HashSet<TargetType> = target_types.into_iter().collect();
    assert!(set.len() >= 7); // At least 7 unique variants
}

#[test]
fn test_shop_target_specific_functionality() {
    // Test Shop-specific functionality
    let shop_targets = vec![
        Target::Shop(0),
        Target::Shop(1),
        Target::Shop(2),
        Target::Shop(10),
    ];

    for target in &shop_targets {
        assert_eq!(target.target_type(), TargetType::Shop);
        assert!(target.is_valid_type(TargetType::Shop));
        assert_eq!(target.card_count(), 0);
    }

    let game = Game::default();
    for target in &shop_targets {
        assert!(target.is_valid(&game));
    }
}
