//! # The GameNight.gay `cypher_character_model` library
//!
//! ## Purpose
//!
//! This library provides a Rust implementation that is compatible with Monte
//! Cook Games' Cypher System
//!
//! ## [`Character`]
//!
//! The main attributes associated with your character from a high-level. The
//! choices made in the `sentence` will impact your [`CharacterStats`], as well
//! as your available skills and abilities.
//!
//! ```
//! # use cypher_character_model::{Character, Sentence};
//! let character = Character {
//!     name: "Ferris".to_string(),
//!     pronouns: "any".to_string(),
//!     sentence: Sentence {
//!       descriptor: "Fast".to_string(),
//!       character_type: "Explorer".to_string(),
//!       flavor: Some("Technology".to_string()),
//!       focus: "Helps Their Friends".to_string(),
//!     }
//! };
//! assert_eq!(
//!     &character.to_string(),
//!     "Ferris (any) is a Fast Explorer (Technology) who Helps Their Friends"
//! );
//! ```
//!
//! ## [`CharacterStats`]
//!
//! The lower-level attributes associated with your character. Once you have
//! your character sentence, that will guide the allocation of points into
//! your might, speed, and intellect pools.
//!
//! ```
//! # use cypher_character_model::CharacterStats;
//! // Create a new, Level 1 character with the following might, speed, and intelligence stats
//! let stats = CharacterStats::new(12, 1, 10, 0, 10, 0);
//! ```

#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// `Character` is the entry-point to the data model
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Character {
    /// The name of the character
    pub name: String,
    /// The pronouns of the character
    pub pronouns: String,
    /// The Cypher System sentence that describes the character
    pub sentence: Sentence,
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}) is a {}",
            self.name, self.pronouns, self.sentence
        )
    }
}

/// `Sentence` is the high-level description of the character
///
/// The Sentence determines lower-level capabilitis such as skills and
/// abilities.
///
/// ([Cypher System Sentence
/// Information](https://cypher-system.com/cypher-system-characters/))
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sentence {
    /// The "adjective" of the sentence
    pub descriptor: String,

    /// The "noun" of the sentence
    ///
    /// This determines the core of your character. The base ruleset has four
    /// character types (Warrior, Adept, Explorer, and Speaker).
    pub character_type: String,

    /// An optional modifier for the [`character_type`]
    ///
    /// This allows for flavoring the character type to your campaign's setting
    /// and your goals. Without delving any deeper, a "Stealthy" flavor Explorer
    /// and a "Technology" flavor Explorer bring to mind different archtypes.
    pub flavor: Option<String>,

    /// The "verb" of the sentence
    ///
    /// This rounds out the character and makes them unique within the party.
    pub focus: String,
}

impl Display for Sentence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let flavor_display = self
            .flavor
            .as_ref()
            .map(|f| format!(" ({f})"))
            .unwrap_or_default();
        write!(
            f,
            "{} {}{} who {}",
            self.descriptor, self.character_type, flavor_display, self.focus
        )
    }
}

/// `CharacterStats` is the specific data about a character at a point in time
///
/// The stats cover things like level, pools, damage, recovery etc. This struct
/// provides an interface for creating a character based on the high-level
/// sentence, and an interface for tracking a character over the course of play.
#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterStats {
    tier: Tier,
    effort: u8,
    xp: u8,
    might: Pool,
    speed: Pool,
    intellect: Pool,
    recovery_rolls: RecoveryRolls,
    damage_track: DamageTrack,
    advancement: Advancement,
}

impl CharacterStats {
    /// Construct a new, Level 1 character
    ///
    /// Currently, this function does not attempt to validate the values added
    /// to the pools and edges.
    pub fn new(
        might_pool: u8,
        might_edge: u8,
        speed_pool: u8,
        speed_edge: u8,
        intellect_pool: u8,
        intellect_edge: u8,
    ) -> Self {
        Self {
            tier: Tier::One,
            effort: 1,
            xp: 0,
            might: Pool::new(might_pool, might_edge),
            speed: Pool::new(speed_pool, speed_edge),
            intellect: Pool::new(intellect_pool, intellect_edge),
            recovery_rolls: RecoveryRolls::default(),
            damage_track: DamageTrack::Hale,
            advancement: Advancement::default(),
        }
    }

    /// Modifies the character based on a certain level of effort and edge
    ///
    /// This function validates that there is enough effort and edge available,
    /// as well as tracking the impact of the damage track on the cost of
    /// effort.
    pub fn spend_effort(
        &mut self,
        effort_type: EffortType,
        effort_level: u8,
        edge: u8,
    ) -> eyre::Result<()> {
        if effort_level == 0 {
            eyre::bail!("Attempted to spend zero effort");
        }
        if self.effort < effort_level {
            eyre::bail!(
                "Attempted to spend more effort than allowed (max {}): {effort_level}",
                self.effort
            );
        }
        let pool = match effort_type {
            EffortType::Might => &mut self.might,
            EffortType::Speed => &mut self.speed,
            EffortType::Intellect => &mut self.intellect,
        };
        if pool.edge < edge {
            eyre::bail!(
                "Attempted to apply more edge than available (max {}): {edge}",
                pool.edge
            );
        }
        let mut points_to_spend = 3 + (effort_level - 1) * 2 - edge;
        if self.damage_track != DamageTrack::Hale {
            points_to_spend += effort_level;
        }
        if points_to_spend >= pool.current {
            eyre::bail!("Attempted to spend all of the {effort_type} pool points (max {}): {points_to_spend}", pool.current);
        }

        pool.current -= points_to_spend;
        Ok(())
    }
}

/// `Tier` is the level of a character, from 1-6
#[derive(Debug, Deserialize, Serialize)]
pub enum Tier {
    /// First tier
    One,
    /// Second tier
    Two,
    /// Third tier
    Three,
    /// Fourth tier
    Four,
    /// Fifth tier
    Five,
    /// Sixth tier
    Six,
}

/// `EffortType` is the different categorization of pools
#[derive(Debug, Deserialize, Serialize)]
pub enum EffortType {
    /// Effort related to physical tasks
    Might,
    /// Effort related to movement/range tasks
    Speed,
    /// Effort related to knowledge and interpersonal tasks
    Intellect,
}

impl Display for EffortType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            EffortType::Might => "Might",
            EffortType::Speed => "Speed",
            EffortType::Intellect => "Intellect",
        };
        write!(f, "{}", display)
    }
}

/// `Pool` is a collection of skill points and skill edge
///
/// There is one pool per [`EffortType`]. Pools have a maximum number of points,
/// a current number of points, and a level of edge that impacts how many points
/// are needed to apply levels of effort.
#[derive(Debug, Deserialize, Serialize)]
pub struct Pool {
    current: u8,
    max: u8,
    edge: u8,
}

impl Pool {
    /// Construct a new pool with the maximum current number of points
    pub fn new(max: u8, edge: u8) -> Self {
        Self {
            current: max,
            max,
            edge,
        }
    }
}

/// `RecoveryRolls` tracks the actions a character has taken to recover in a day
///
/// There are four rolls per day: 1 action, 10 minutes, 1 hour, and 10 hours.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RecoveryRolls {
    one_action: bool,
    ten_minutes: bool,
    one_hour: bool,
    ten_hours: bool,
}

/// `DamageTrack` tracks the level of damage a character has experienced
///
/// Characters go down the damage track when a pool goes to zero points or when
/// an effect specifically causes it. Characters can go back up the damage track
/// through being healed or performing recovery rolls.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum DamageTrack {
    /// Fully healthy
    Hale,
    /// One pool reduced to zero points
    ///
    /// - Levels of effort cost 1 additional point per level
    /// - Major and minor effects are ignored
    /// - 17-20 rolls only do +1 damage
    Impaired,
    /// Two pools reduced to zero points
    ///
    /// - Cannot move more than an immediate distance
    /// - Cannot move at all if the speed pool is at 0
    Debilitated,
}

/// `Advancement` tracks character progression towards levelling up
///
/// Advancements cost 4xp and each advancement can only be done at most once per
/// level. A character reaches the next level after four advancements.
///
/// - Increase Capabilities: +4 points to a character's stat pools
/// - Move Toward Perfection: +1 edge
/// - Extra Effort: +1 effort
/// - Skill Training: train a new skill, become specialized in a trained skill,
///   or remove an inhibition in an inhibited skill
/// - Other: An agreed upon character advancement between the player and GM
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Advancement {
    increase_capabilities: bool,
    move_toward_perfection: bool,
    extra_effort: bool,
    skill_training: bool,
    other: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_charachter_should_display_as_the_sentence() -> eyre::Result<()> {
        let character = Character {
            name: r#"Lt. Commander Jane "JJ" Jones"#.to_string(),
            pronouns: "she/her".to_string(),
            sentence: Sentence {
                descriptor: "Impulsive".to_string(),
                character_type: "Explorer".to_string(),
                flavor: Some("Combat".to_string()),
                focus: "Sailed Beneath The Jolly Roger".to_string(),
            },
        };
        let actual = character.to_string();
        let expected = r#"Lt. Commander Jane "JJ" Jones (she/her) is a Impulsive Explorer (Combat) who Sailed Beneath The Jolly Roger"#;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn spending_effort_should_reject_zero_effort() -> eyre::Result<()> {
        let mut character_stats = CharacterStats::new(5, 1, 5, 0, 5, 0);

        let error = character_stats
            .spend_effort(EffortType::Might, 0, 0)
            .expect_err("Should have failed");

        assert_eq!(error.to_string(), "Attempted to spend zero effort");

        Ok(())
    }

    #[test]
    fn spending_effort_should_reject_too_much_effort() -> eyre::Result<()> {
        let mut character_stats = CharacterStats::new(5, 1, 5, 0, 5, 0);

        let error = character_stats
            .spend_effort(EffortType::Might, 2, 0)
            .expect_err("Should have failed");

        assert_eq!(
            error.to_string(),
            "Attempted to spend more effort than allowed (max 1): 2"
        );

        Ok(())
    }

    #[test]
    fn spending_effort_should_reject_too_mutch_edge() -> eyre::Result<()> {
        let mut character_stats = CharacterStats::new(5, 1, 5, 0, 5, 0);

        let error = character_stats
            .spend_effort(EffortType::Might, 1, 2)
            .expect_err("Should have failed");

        assert_eq!(
            error.to_string(),
            "Attempted to apply more edge than available (max 1): 2"
        );

        Ok(())
    }

    #[test]
    fn spending_effort_should_reject_exhausting_the_pool() -> eyre::Result<()> {
        let mut character_stats = CharacterStats::new(2, 1, 5, 0, 5, 0);

        let error = character_stats
            .spend_effort(EffortType::Might, 1, 0)
            .expect_err("Should have failed");

        assert_eq!(
            error.to_string(),
            "Attempted to spend all of the Might pool points (max 2): 3"
        );

        let error = character_stats
            .spend_effort(EffortType::Might, 1, 1)
            .expect_err("Should have failed");

        assert_eq!(
            error.to_string(),
            "Attempted to spend all of the Might pool points (max 2): 2"
        );

        Ok(())
    }

    #[test]
    fn spending_effort_should_succeed() -> eyre::Result<()> {
        let mut character_stats = CharacterStats::new(10, 1, 5, 0, 5, 0);
        character_stats.effort = 2;

        character_stats.spend_effort(EffortType::Might, 1, 1)?;
        assert_eq!(
            character_stats.might.current, 8,
            "Spending 1 level of effort with 1 edge costs 2 points"
        );

        character_stats.spend_effort(EffortType::Might, 2, 1)?;
        assert_eq!(
            character_stats.might.current, 4,
            "Spending 2 levels of effort with 1 edge costs 4 points"
        );

        Ok(())
    }

    #[test]
    fn spending_effort_should_cost_extra_when_impaired() -> eyre::Result<()> {
        let mut character_stats = CharacterStats::new(10, 1, 5, 0, 5, 0);
        character_stats.effort = 2;
        character_stats.speed.current = 0;
        character_stats.damage_track = DamageTrack::Impaired;

        character_stats.spend_effort(EffortType::Might, 1, 1)?;
        assert_eq!(
            character_stats.might.current, 7,
            "Spending 1 level of effort with 1 edge (while impaired) costs 3 points"
        );

        character_stats.spend_effort(EffortType::Might, 2, 1)?;
        assert_eq!(
            character_stats.might.current, 1,
            "Spending 2 levels of effort with 1 edge (while impaired) costs 6 points"
        );

        Ok(())
    }
}
