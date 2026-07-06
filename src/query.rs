use crate::error::{Error, Result};
use crate::evidence::{
    CompositeEvidence, ContainsEvidence, MatchEvidence, NearEvidence, NearOccurrencePair,
    Occurrence, SearchOutcome,
};
use crate::text::{NormalizedWord, SearchText};

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
#[rkyv(
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext)),
    serialize_bounds(
        __S: rkyv::ser::Writer + rkyv::ser::Allocator,
        __S::Error: rkyv::rancor::Source,
    ),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
)]
pub enum Query {
    Contains(QueryTerm),
    AllOf(#[rkyv(omit_bounds)] Vec<Query>),
    AnyOf(#[rkyv(omit_bounds)] Vec<Query>),
    Not(#[rkyv(omit_bounds)] Box<Query>),
    Near(NearQuery),
}

impl Query {
    pub fn contains(term: QueryTerm) -> Self {
        Self::Contains(term)
    }

    pub fn all_of(query: Vec<Query>) -> Self {
        Self::AllOf(query)
    }

    pub fn any_of(query: Vec<Query>) -> Self {
        Self::AnyOf(query)
    }

    pub fn negated(query: Query) -> Self {
        Self::Not(Box::new(query))
    }

    pub fn near(left: QueryTerm, right: QueryTerm, distance: WordDistance) -> Self {
        Self::Near(NearQuery::new(left, right, distance))
    }

    pub fn find_in(&self, text: &SearchText) -> SearchOutcome {
        QueryMatcher::new(self, text).outcome()
    }

    pub fn to_rkyv_bytes(&self) -> Result<Vec<u8>> {
        rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .map(|bytes| bytes.to_vec())
            .map_err(|error| Error::RkyvEncode(error.to_string()))
    }

    pub fn from_rkyv_bytes(bytes: &[u8]) -> Result<Self> {
        rkyv::from_bytes::<Self, rkyv::rancor::Error>(bytes)
            .map_err(|error| Error::RkyvDecode(error.to_string()))
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
pub enum QueryTerm {
    Word(SearchWord),
    Phrase(SearchPhrase),
}

impl QueryTerm {
    pub fn word(word: impl Into<String>) -> Self {
        Self::Word(SearchWord::new(word))
    }

    pub fn phrase(words: impl Into<Vec<String>>) -> Self {
        Self::Phrase(SearchPhrase::new(words))
    }

    pub fn occurrences_in(&self, text: &SearchText) -> Vec<Occurrence> {
        match self {
            Self::Word(word) => word.occurrences_in(text),
            Self::Phrase(phrase) => phrase.occurrences_in(text),
        }
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
pub struct SearchWord {
    pub value: String,
}

impl SearchWord {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn normalized(&self) -> NormalizedWord {
        SearchText::new(self.value.as_str())
            .words
            .into_iter()
            .next()
            .unwrap_or_else(|| NormalizedWord(String::new()))
    }

    pub fn occurrences_in(&self, text: &SearchText) -> Vec<Occurrence> {
        let normalized = self.normalized();
        text.words
            .iter()
            .enumerate()
            .filter(|&(_index, word)| word == &normalized)
            .map(|(index, _word)| Occurrence::new(index as u32, index as u32))
            .collect()
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
pub struct SearchPhrase {
    pub words: Vec<String>,
}

impl SearchPhrase {
    pub fn new(words: impl Into<Vec<String>>) -> Self {
        Self {
            words: words.into(),
        }
    }

    pub fn normalized_words(&self) -> Vec<NormalizedWord> {
        self.words
            .iter()
            .flat_map(|word| SearchText::new(word.as_str()).words)
            .collect()
    }

    pub fn occurrences_in(&self, text: &SearchText) -> Vec<Occurrence> {
        let words = self.normalized_words();
        if words.is_empty() || words.len() > text.words.len() {
            return Vec::new();
        }

        text.words
            .windows(words.len())
            .enumerate()
            .filter(|&(_start, candidate)| candidate == words.as_slice())
            .map(|(start, _candidate)| {
                Occurrence::new(start as u32, (start + words.len() - 1) as u32)
            })
            .collect()
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
pub struct NearQuery {
    pub left: QueryTerm,
    pub right: QueryTerm,
    pub distance: WordDistance,
}

impl NearQuery {
    pub fn new(left: QueryTerm, right: QueryTerm, distance: WordDistance) -> Self {
        Self {
            left,
            right,
            distance,
        }
    }

    pub fn pairs_in(&self, text: &SearchText) -> Vec<NearOccurrencePair> {
        let left_occurrences = self.left.occurrences_in(text);
        let right_occurrences = self.right.occurrences_in(text);
        let mut pairs = Vec::new();

        for left in left_occurrences {
            for right in &right_occurrences {
                let gap = left.gap_to(right);
                if gap <= self.distance {
                    pairs.push(NearOccurrencePair::new(left, *right, gap));
                }
            }
        }

        pairs.sort_by_key(|pair| (pair.left.start, pair.right.start, pair.gap.0));
        pairs
    }
}

#[derive(
    nota::NotaEncode,
    nota::NotaDecode,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub struct WordDistance(pub u32);

impl WordDistance {
    pub fn new(distance: u32) -> Self {
        Self(distance)
    }
}

struct QueryMatcher<'query, 'text> {
    query: &'query Query,
    text: &'text SearchText,
}

impl<'query, 'text> QueryMatcher<'query, 'text> {
    fn new(query: &'query Query, text: &'text SearchText) -> Self {
        Self { query, text }
    }

    fn outcome(&self) -> SearchOutcome {
        match self.query {
            Query::Contains(term) => self.contains_outcome(term),
            Query::AllOf(query) => self.all_of_outcome(query),
            Query::AnyOf(query) => self.any_of_outcome(query),
            Query::Not(query) => self.not_outcome(query),
            Query::Near(query) => self.near_outcome(query),
        }
    }

    fn contains_outcome(&self, term: &QueryTerm) -> SearchOutcome {
        let occurrences = term.occurrences_in(self.text);
        if occurrences.is_empty() {
            SearchOutcome::not_matched()
        } else {
            SearchOutcome::matched(MatchEvidence::Contains(ContainsEvidence::new(
                term.clone(),
                occurrences,
            )))
        }
    }

    fn all_of_outcome(&self, query: &[Query]) -> SearchOutcome {
        let mut matches = Vec::new();
        for child in query {
            match child.find_in(self.text) {
                SearchOutcome::Matched(evidence) => matches.push(evidence),
                SearchOutcome::NotMatched => return SearchOutcome::not_matched(),
            }
        }

        SearchOutcome::matched(MatchEvidence::AllOf(CompositeEvidence::new(matches)))
    }

    fn any_of_outcome(&self, query: &[Query]) -> SearchOutcome {
        let matches = query
            .iter()
            .filter_map(|child| child.find_in(self.text).evidence().cloned())
            .collect::<Vec<_>>();

        if matches.is_empty() {
            SearchOutcome::not_matched()
        } else {
            SearchOutcome::matched(MatchEvidence::AnyOf(CompositeEvidence::new(matches)))
        }
    }

    fn not_outcome(&self, query: &Query) -> SearchOutcome {
        match query.find_in(self.text) {
            SearchOutcome::Matched(_) => SearchOutcome::not_matched(),
            SearchOutcome::NotMatched => SearchOutcome::matched(MatchEvidence::Not),
        }
    }

    fn near_outcome(&self, query: &NearQuery) -> SearchOutcome {
        let pairs = query.pairs_in(self.text);
        if pairs.is_empty() {
            SearchOutcome::not_matched()
        } else {
            SearchOutcome::matched(MatchEvidence::Near(NearEvidence::new(
                query.left.clone(),
                query.right.clone(),
                query.distance,
                pairs,
            )))
        }
    }
}
