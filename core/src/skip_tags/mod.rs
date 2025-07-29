//! Skip Tag System Implementation
//!
//! This module provides the complete infrastructure for Balatro's skip tag system,
//! which rewards players for skipping blinds instead of playing them.
//!
//! # Architecture Overview
//!
//! The skip tag system consists of:
//! - **SkipTag trait**: Core interface defining tag behavior
//! - **TagId enum**: All 26 skip tags across 5 categories
//! - **TagEffectType**: Classification system for tag effects
//! - **TagRegistry**: Thread-safe registry and factory system
//! - **TagError**: Comprehensive error handling framework
//!
//! # Performance Requirements
//!
//! - Tag lookup: <1μs
//! - Effect application: <100ms
//! - Memory usage: <1KB per active tag effect
//!
//! # Usage Example
//!
//! ```rust,ignore
//! use balatro_rs::skip_tags::{TagRegistry, TagId};
//!
//! // Get registry instance
//! let registry = TagRegistry::global();
//!
//! // Look up tag definition
//! let definition = registry.get_definition(TagId::Charm)?;
//!
//! // Create tag instance
//! let tag = registry.create_tag(TagId::Charm)?;
//!
//! // Check if tag can apply
//! if tag.can_apply(&game_state) {
//!     tag.apply_effect(&mut game_state)?;
//! }
//! ```

pub mod error;
pub mod registry;
pub mod traits;

pub use error::{TagError, TagErrorKind};
pub use registry::{TagDefinition, TagFactory, TagRegistry};
pub use traits::{SkipTag, TagCategory, TagEffectType, TagId};
