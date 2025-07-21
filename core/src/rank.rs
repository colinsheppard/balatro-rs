pub struct Level {
    pub level: usize,
    pub chips: usize,
    pub mult: usize,
}

/// All the different possible hand ranks.
/// For each hand rank the u32 corresponds to
/// the strength of the hand in comparison to others
/// of the same rank.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
    FiveOfAKind,
    FlushHouse,
    FlushFive,
}

impl HandRank {
    pub(crate) fn level(&self) -> Level {
        match self {
            Self::HighCard => Level {
                level: 1,
                chips: 5,
                mult: 1,
            },
            Self::OnePair => Level {
                level: 1,
                chips: 10,
                mult: 2,
            },
            Self::TwoPair => Level {
                level: 1,
                chips: 20,
                mult: 2,
            },
            Self::ThreeOfAKind => Level {
                level: 1,
                chips: 30,
                mult: 3,
            },
            Self::Straight => Level {
                level: 1,
                chips: 30,
                mult: 4,
            },
            Self::Flush => Level {
                level: 1,
                chips: 35,
                mult: 4,
            },
            Self::FullHouse => Level {
                level: 1,
                chips: 40,
                mult: 4,
            },
            Self::FourOfAKind => Level {
                level: 1,
                chips: 60,
                mult: 7,
            },
            Self::StraightFlush => Level {
                level: 1,
                chips: 100,
                mult: 8,
            },
            Self::RoyalFlush => Level {
                level: 1,
                chips: 100,
                mult: 8,
            },
            Self::FiveOfAKind => Level {
                level: 1,
                chips: 120,
                mult: 12,
            },
            Self::FlushHouse => Level {
                level: 1,
                chips: 140,
                mult: 14,
            },
            Self::FlushFive => Level {
                level: 1,
                chips: 160,
                mult: 16,
            },
        }
    }
}

impl std::fmt::Display for HandRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HighCard => write!(f, "High Card"),
            Self::OnePair => write!(f, "One Pair"),
            Self::TwoPair => write!(f, "Two Pair"),
            Self::ThreeOfAKind => write!(f, "Three of a Kind"),
            Self::Straight => write!(f, "Straight"),
            Self::Flush => write!(f, "Flush"),
            Self::FullHouse => write!(f, "Full House"),
            Self::FourOfAKind => write!(f, "Four of a Kind"),
            Self::StraightFlush => write!(f, "Straight Flush"),
            Self::RoyalFlush => write!(f, "Royal Flush"),
            Self::FiveOfAKind => write!(f, "Five of a Kind"),
            Self::FlushHouse => write!(f, "Flush House"),
            Self::FlushFive => write!(f, "Flush Five"),
        }
    }
}
