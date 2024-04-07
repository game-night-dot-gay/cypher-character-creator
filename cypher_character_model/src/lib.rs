use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Character is the entry-point to the data model
#[derive(Debug, Deserialize, Serialize)]
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

/// Sentence is the high-level description of the character
#[derive(Debug, Deserialize, Serialize)]
pub struct Sentence {
    pub descriptor: String,
    pub character_type: String,
    pub flavor: Option<String>,
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
            recovery_rolls: RecoveryRolls::new(),
            damage_track: DamageTrack::new(),
            advancement: Advancement::new(),
        }
    }

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
        let points_to_spend = 3 + (effort_level - 1) * 2 - edge;
        if points_to_spend >= pool.current {
            eyre::bail!("Attempted to spend all of the {effort_type} pool points (max {}): {points_to_spend}", pool.current);
        }

        pool.current -= points_to_spend;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Tier {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EffortType {
    Might,
    Speed,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Pool {
    current: u8,
    max: u8,
    edge: u8,
}

impl Pool {
    pub fn new(max: u8, edge: u8) -> Self {
        Self {
            current: max,
            max,
            edge,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecoveryRolls {
    one_action: bool,
    ten_minutes: bool,
    one_hour: bool,
    ten_hours: bool,
}

impl RecoveryRolls {
    pub fn new() -> Self {
        Self {
            one_action: false,
            ten_minutes: false,
            one_hour: false,
            ten_hours: false,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DamageTrack {
    impaired: bool,
    debilitated: bool,
}

impl DamageTrack {
    pub fn new() -> Self {
        Self {
            impaired: false,
            debilitated: false,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Advancement {
    increase_capabilities: bool,
    move_toward_perfection: bool,
    extra_effort: bool,
    skill_training: bool,
    other: bool,
}

impl Advancement {
    pub fn new() -> Self {
        Self {
            increase_capabilities: false,
            move_toward_perfection: false,
            extra_effort: false,
            skill_training: false,
            other: false,
        }
    }
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
}
