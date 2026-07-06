#[derive(
    nota::NotaEncode,
    nota::NotaDecode,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub struct NormalizedWord(pub String);

impl NormalizedWord {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(
    nota::NotaEncode,
    nota::NotaDecode,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub struct SearchText {
    pub original: String,
    pub words: Vec<NormalizedWord>,
}

impl SearchText {
    pub fn new(text: impl Into<String>) -> Self {
        let original = text.into();
        let words = TextNormalizer::new(original.as_str()).words();
        Self { original, words }
    }
}

struct TextNormalizer {
    text: String,
}

impl TextNormalizer {
    fn new(text: &str) -> Self {
        Self {
            text: text.to_owned(),
        }
    }

    fn words(&self) -> Vec<NormalizedWord> {
        let mut words = Vec::new();
        let mut current = String::new();

        for character in self.text.chars() {
            if character.is_alphanumeric() {
                current.extend(character.to_lowercase());
            } else if !current.is_empty() {
                words.push(NormalizedWord(std::mem::take(&mut current)));
            }
        }

        if !current.is_empty() {
            words.push(NormalizedWord(current));
        }

        words
    }
}
