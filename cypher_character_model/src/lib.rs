use std::fmt::Display;

use serde::{Deserialize, Serialize};

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
}
