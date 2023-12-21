/// Character is the entry-point to the data model
#[derive(Debug)]
pub struct Character {
    /// The name of the character
    pub name: String,
    /// The pronouns of the character
    pub pronouns: String,
    /// The Cypher System sentence that describes the character
    pub sentence: Sentence,
}

/// Sentence is the high-level description of the character
#[derive(Debug)]
pub struct Sentence {
    pub descriptor: String,
    pub character_type: String,
    pub flavor: Option<String>,
    pub focus: String,
}
