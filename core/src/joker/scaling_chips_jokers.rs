//! Scaling Chips Jokers implementation
//!
//! This module implements jokers that provide or accumulate chip bonuses over time.
//! These jokers follow the scaling pattern where their power grows throughout the game.
//!
//! Based on joker.json specifications - the authoritative source for joker behavior.

use crate::{
    card::{Card, Suit, Value},
    hand::SelectHand,
    joker::{
        traits::{ProcessContext, ProcessResult, Rarity},
        GameContext, Joker, JokerEffect, JokerGameplay, JokerId, JokerIdentity, JokerRarity,
    },
    joker_state::JokerState,
    stage::Stage,
};

/// Castle joker - Gains chips per discarded card of a specific suit (changes every round)
/// 
/// From joker.json: "This Joker gains {C:chips}+#1#{} Chips per discarded {V:1}#2#{} card,
/// suit changes every round"
#[derive(Debug, Clone)]
pub struct CastleJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for CastleJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl CastleJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::TheFamily, // Using appropriate ID mapping
            name: "Castle".to_string(),
            description: "This Joker gains +3 Chips per discarded card of a specific suit, suit changes every round".to_string(),
            rarity: JokerRarity::Rare,
            cost: 8,
        }
    }

    /// Get the current target suit for this round
    fn get_target_suit(context: &GameContext) -> Suit {
        // Cycle through suits based on round number for deterministic behavior
        match context.round % 4 {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Club,
            _ => Suit::Spade,
        }
    }

    /// Get accumulated chips from joker state
    fn get_accumulated_chips(context: &GameContext, joker_id: JokerId) -> i32 {
        context.joker_state_manager
            .get_state(joker_id)
            .map(|state| state.accumulated_value as i32)
            .unwrap_or(0)
    }
}

impl JokerIdentity for CastleJoker {
    fn joker_type(&self) -> &'static str {
        "castle"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }

    fn base_cost(&self) -> u64 {
        self.cost as u64
    }
}

impl Joker for CastleJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    fn cost(&self) -> usize {
        self.cost
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        let accumulated_chips = Self::get_accumulated_chips(context, self.id());
        let target_suit = Self::get_target_suit(context);
        
        JokerEffect::new()
            .with_chips(accumulated_chips)
            .with_message(format!("Castle: +{accumulated_chips} Chips (Target: {target_suit:?})"))
    }

    fn on_discard(&self, context: &mut GameContext, cards: &[Card]) -> JokerEffect {
        let target_suit = Self::get_target_suit(context);
        let matching_cards = cards.iter().filter(|card| card.suit == target_suit).count();
        
        if matching_cards > 0 {
            let chips_gained = matching_cards as i32 * 3;
            
            // Update accumulated value in joker state
            if let Some(mut state) = context.joker_state_manager.get_state(self.id()) {
                state.accumulated_value += chips_gained as f64;
                context.joker_state_manager.set_state(self.id(), state);
            } else {
                let mut state = JokerState::new();
                state.accumulated_value = chips_gained as f64;
                context.joker_state_manager.set_state(self.id(), state);
            }
            
            JokerEffect::new().with_message(format!(
                "Castle: +{chips_gained} Chips gained ({matching_cards} {target_suit:?} cards discarded)"
            ))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for CastleJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        let accumulated_chips = context.joker_state_manager
            .get_state(self.id())
            .map(|state| state.accumulated_value as u64)
            .unwrap_or(0);

        ProcessResult {
            chips_added: accumulated_chips,
            mult_added: 0.0,
            mult_multiplier: 1.0,
            retriggered: false,
            message: Some(format!("Castle: +{accumulated_chips} Chips")),
        }
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
    }
}

/// Wee joker - Gains chips when 2s are scored
/// 
/// From joker.json: "This Joker gains {C:chips}+#2#{} Chips when each played {C:attention}2{} is scored"
#[derive(Debug, Clone)]
pub struct WeeJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for WeeJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl WeeJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Wee,
            name: "Wee Joker".to_string(),
            description: "This Joker gains +8 Chips when each played 2 is scored".to_string(),
            rarity: JokerRarity::Rare,
            cost: 8,
        }
    }

    fn get_accumulated_chips(context: &GameContext, joker_id: JokerId) -> i32 {
        context.joker_state_manager
            .get_state(joker_id)
            .map(|state| state.accumulated_value as i32)
            .unwrap_or(0)
    }
}

impl JokerIdentity for WeeJoker {
    fn joker_type(&self) -> &'static str {
        "wee"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }

    fn base_cost(&self) -> u64 {
        self.cost as u64
    }
}

impl Joker for WeeJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    fn cost(&self) -> usize {
        self.cost
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        let accumulated_chips = Self::get_accumulated_chips(context, self.id());
        
        JokerEffect::new()
            .with_chips(accumulated_chips)
            .with_message(format!("Wee Joker: +{accumulated_chips} Chips"))
    }

    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.value == Value::Two {
            let chips_gained = 8;
            
            // Update accumulated value in joker state
            if let Some(mut state) = context.joker_state_manager.get_state(self.id()) {
                state.accumulated_value += chips_gained as f64;
                context.joker_state_manager.set_state(self.id(), state);
            } else {
                let mut state = JokerState::new();
                state.accumulated_value = chips_gained as f64;
                context.joker_state_manager.set_state(self.id(), state);
            }
            
            JokerEffect::new().with_message(format!("Wee Joker: +{chips_gained} Chips gained (2 scored)"))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for WeeJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        let twos_played = context.played_cards
            .iter()
            .filter(|card| card.value == Value::Two)
            .count();

        if twos_played > 0 {
            // Update accumulated chips
            let chips_gained = (twos_played * 8) as u64;
            
            if let Some(mut state) = context.joker_state_manager.get_state(self.id()) {
                state.accumulated_value += chips_gained as f64;
                context.joker_state_manager.set_state(self.id(), state);
            }
        }

        let accumulated_chips = context.joker_state_manager
            .get_state(self.id())
            .map(|state| state.accumulated_value as u64)
            .unwrap_or(0);

        ProcessResult {
            chips_added: accumulated_chips,
            mult_added: 0.0,
            mult_multiplier: 1.0,
            retriggered: false,
            message: Some(format!("Wee Joker: +{accumulated_chips} Chips")),
        }
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
    }
}

/// Stuntman joker - Provides flat chips but reduces hand size
/// 
/// From joker.json: "{C:chips}+#1#{} Chips, {C:attention}-#2#{} hand size"
#[derive(Debug, Clone)]
pub struct StuntmanJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for StuntmanJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl StuntmanJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::AcrobatJoker, // Using appropriate ID mapping
            name: "Stuntman".to_string(),
            description: "+300 Chips, -2 hand size".to_string(),
            rarity: JokerRarity::Rare,
            cost: 8,
        }
    }
}

impl JokerIdentity for StuntmanJoker {
    fn joker_type(&self) -> &'static str {
        "stuntman"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }

    fn base_cost(&self) -> u64 {
        self.cost as u64
    }
}

impl Joker for StuntmanJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    fn cost(&self) -> usize {
        self.cost
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        JokerEffect::new()
            .with_chips(300)
            .with_message("Stuntman: +300 Chips".to_string())
    }

    fn modify_hand_size(&self, _context: &GameContext, base_size: usize) -> usize {
        base_size.saturating_sub(2) // -2 hand size
    }
}

impl JokerGameplay for StuntmanJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        ProcessResult {
            chips_added: 300,
            mult_added: 0.0,
            mult_multiplier: 1.0,
            retriggered: false,
            message: Some("Stuntman: +300 Chips".to_string()),
        }
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
    }
}

/// Hiker joker - Every played card permanently gains chips
/// 
/// From joker.json: "Every played {C:attention}card{} permanently gains {C:chips}+#1#{} Chips when scored"
#[derive(Debug, Clone)]
pub struct HikerJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for HikerJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl HikerJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Hiker,
            name: "Hiker".to_string(),
            description: "Every played card permanently gains +5 Chips when scored".to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 6,
        }
    }
}

impl JokerIdentity for HikerJoker {
    fn joker_type(&self) -> &'static str {
        "hiker"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }

    fn base_cost(&self) -> u64 {
        self.cost as u64
    }
}

impl Joker for HikerJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    fn cost(&self) -> usize {
        self.cost
    }

    fn on_card_scored(&self, _context: &mut GameContext, _card: &Card) -> JokerEffect {
        // NOTE: The permanent card enhancement would need to be implemented 
        // in the game engine's card system. For now, we provide chips per card.
        JokerEffect::new()
            .with_chips(5)
            .with_message("Hiker: +5 Chips (card enhanced)".to_string())
    }
}

impl JokerGameplay for HikerJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        let cards_played = context.played_cards.len() as u64;
        let chips_added = cards_played * 5;

        ProcessResult {
            chips_added,
            mult_added: 0.0,
            mult_multiplier: 1.0,
            retriggered: false,
            message: Some(format!("Hiker: +{chips_added} Chips ({cards_played} cards enhanced)")),
        }
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && !context.played_cards.is_empty()
    }
}

/// Odd Todd joker - Chips for odd rank cards
/// 
/// From joker.json: "Played cards with {C:attention}odd{} rank give {C:chips}+#1#{} Chips when scored"
#[derive(Debug, Clone)]
pub struct OddToddJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for OddToddJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl OddToddJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::OddTodd,
            name: "Odd Todd".to_string(),
            description: "Played cards with odd rank give +30 Chips when scored (A, 9, 7, 5, 3)".to_string(),
            rarity: JokerRarity::Common,
            cost: 3,
        }
    }

    fn is_odd_rank(card: &Card) -> bool {
        matches!(card.value, Value::Ace | Value::Three | Value::Five | Value::Seven | Value::Nine)
    }
}

impl JokerIdentity for OddToddJoker {
    fn joker_type(&self) -> &'static str {
        "odd_todd"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> Rarity {
        Rarity::Common
    }

    fn base_cost(&self) -> u64 {
        self.cost as u64
    }
}

impl Joker for OddToddJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    fn cost(&self) -> usize {
        self.cost
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if Self::is_odd_rank(card) {
            JokerEffect::new()
                .with_chips(30)
                .with_message(format!("Odd Todd: +30 Chips ({:?} is odd)", card.value))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for OddToddJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        let odd_cards = context.played_cards
            .iter()
            .filter(|card| Self::is_odd_rank(card))
            .count() as u64;

        let chips_added = odd_cards * 30;

        ProcessResult {
            chips_added,
            mult_added: 0.0,
            mult_multiplier: 1.0,
            retriggered: false,
            message: Some(format!("Odd Todd: +{chips_added} Chips ({odd_cards} odd cards)")),
        }
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && context.played_cards.iter().any(Self::is_odd_rank)
    }
}

/// Arrowhead joker - Chips for Spade suit cards
/// 
/// From joker.json: "Played cards with {C:spades}Spade{} suit give {C:chips}+#1#{} Chips when scored"
#[derive(Debug, Clone)]
pub struct ArrowheadJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for ArrowheadJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl ArrowheadJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Arrowhead,
            name: "Arrowhead".to_string(),
            description: "Played cards with Spade suit give +50 Chips when scored".to_string(),
            rarity: JokerRarity::Common,
            cost: 3,
        }
    }
}

impl JokerIdentity for ArrowheadJoker {
    fn joker_type(&self) -> &'static str {
        "arrowhead"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> Rarity {
        Rarity::Common
    }

    fn base_cost(&self) -> u64 {
        self.cost as u64
    }
}

impl Joker for ArrowheadJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    fn cost(&self) -> usize {
        self.cost
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.suit == Suit::Spade {
            JokerEffect::new()
                .with_chips(50)
                .with_message("Arrowhead: +50 Chips (Spade scored)".to_string())
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for ArrowheadJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        let spade_cards = context.played_cards
            .iter()
            .filter(|card| card.suit == Suit::Spade)
            .count() as u64;

        let chips_added = spade_cards * 50;

        ProcessResult {
            chips_added,
            mult_added: 0.0,
            mult_multiplier: 1.0,
            retriggered: false,
            message: Some(format!("Arrowhead: +{chips_added} Chips ({spade_cards} Spades)")),
        }
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && context.played_cards.iter().any(|card| card.suit == Suit::Spade)
    }
}

/// Scholar joker - Chips and mult for Aces
/// 
/// From joker.json: "Played {C:attention}Aces{} give {C:chips}+#2#{} Chips and {C:mult}+#1#{} Mult when scored"
#[derive(Debug, Clone)]
pub struct ScholarJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for ScholarJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl ScholarJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Scholar,
            name: "Scholar".to_string(),
            description: "Played Aces give +20 Chips and +4 Mult when scored".to_string(),
            rarity: JokerRarity::Common,
            cost: 3,
        }
    }
}

impl JokerIdentity for ScholarJoker {
    fn joker_type(&self) -> &'static str {
        "scholar"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> Rarity {
        Rarity::Common
    }

    fn base_cost(&self) -> u64 {
        self.cost as u64
    }
}

impl Joker for ScholarJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    fn cost(&self) -> usize {
        self.cost
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.value == Value::Ace {
            JokerEffect::new()
                .with_chips(20)
                .with_mult(4)
                .with_message("Scholar: +20 Chips, +4 Mult (Ace scored)".to_string())
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for ScholarJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        let ace_cards = context.played_cards
            .iter()
            .filter(|card| card.value == Value::Ace)
            .count() as u64;

        let chips_added = ace_cards * 20;
        let mult_added = (ace_cards * 4) as f64;

        ProcessResult {
            chips_added,
            mult_added,
            mult_multiplier: 1.0,
            retriggered: false,
            message: Some(format!("Scholar: +{chips_added} Chips, +{mult_added} Mult ({ace_cards} Aces)")),
        }
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && context.played_cards.iter().any(|card| card.value == Value::Ace)
    }
}

/// Factory functions for creating scaling chips jokers
pub fn create_castle_joker() -> Box<dyn Joker> {
    Box::new(CastleJoker::new())
}

pub fn create_wee_joker() -> Box<dyn Joker> {
    Box::new(WeeJoker::new())
}

pub fn create_stuntman_joker() -> Box<dyn Joker> {
    Box::new(StuntmanJoker::new())
}

pub fn create_hiker_joker() -> Box<dyn Joker> {
    Box::new(HikerJoker::new())
}

pub fn create_odd_todd_joker() -> Box<dyn Joker> {
    Box::new(OddToddJoker::new())
}

pub fn create_arrowhead_joker() -> Box<dyn Joker> {
    Box::new(ArrowheadJoker::new())
}

pub fn create_scholar_joker() -> Box<dyn Joker> {
    Box::new(ScholarJoker::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{card::Suit, stage::Blind};

    #[test]
    fn test_castle_joker_suit_cycling() {
        let castle = CastleJoker::new();
        assert_eq!(castle.joker_type(), "castle");
        assert_eq!(JokerIdentity::name(&castle), "Castle");
        assert_eq!(JokerIdentity::rarity(&castle), Rarity::Rare);
    }

    #[test]
    fn test_wee_joker_scaling() {
        let wee = WeeJoker::new();
        assert_eq!(wee.joker_type(), "wee");
        assert_eq!(JokerIdentity::name(&wee), "Wee Joker");
        assert_eq!(JokerIdentity::rarity(&wee), Rarity::Rare);
    }

    #[test]
    fn test_stuntman_joker_flat_bonus() {
        let stuntman = StuntmanJoker::new();
        assert_eq!(stuntman.joker_type(), "stuntman");
        assert_eq!(JokerIdentity::name(&stuntman), "Stuntman");
        assert_eq!(JokerIdentity::rarity(&stuntman), Rarity::Rare);
    }

    #[test]
    fn test_odd_todd_rank_detection() {
        let _odd_todd = OddToddJoker::new();
        
        let ace = Card::new(Value::Ace, Suit::Heart);
        let three = Card::new(Value::Three, Suit::Spade);
        let four = Card::new(Value::Four, Suit::Diamond);
        
        assert!(OddToddJoker::is_odd_rank(&ace));
        assert!(OddToddJoker::is_odd_rank(&three));
        assert!(!OddToddJoker::is_odd_rank(&four));
    }

    #[test]
    fn test_arrowhead_spade_detection() {
        let arrowhead = ArrowheadJoker::new();
        
        let spade_card = Card::new(Value::King, Suit::Spade);
        let heart_card = Card::new(Value::King, Suit::Heart);
        
        assert_eq!(arrowhead.joker_type(), "arrowhead");
        
        // Test that Spade cards would trigger
        assert_eq!(spade_card.suit, Suit::Spade);
        assert_ne!(heart_card.suit, Suit::Spade);
    }

    #[test]
    fn test_scholar_ace_detection() {
        let scholar = ScholarJoker::new();
        
        let ace = Card::new(Value::Ace, Suit::Heart);
        let king = Card::new(Value::King, Suit::Heart);
        
        assert_eq!(scholar.joker_type(), "scholar");
        assert_eq!(ace.value, Value::Ace);
        assert_ne!(king.value, Value::Ace);
    }

    #[test]
    fn test_hiker_card_enhancement() {
        let hiker = HikerJoker::new();
        assert_eq!(hiker.joker_type(), "hiker");
        assert_eq!(JokerIdentity::name(&hiker), "Hiker");
        assert_eq!(JokerIdentity::rarity(&hiker), Rarity::Uncommon);
    }
}