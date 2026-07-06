use crate::query::{QueryTerm, WordDistance};

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
pub enum SearchOutcome {
    Matched(MatchEvidence),
    NotMatched,
}

impl SearchOutcome {
    pub fn matched(evidence: MatchEvidence) -> Self {
        Self::Matched(evidence)
    }

    pub fn not_matched() -> Self {
        Self::NotMatched
    }

    pub fn evidence(&self) -> Option<&MatchEvidence> {
        match self {
            Self::Matched(evidence) => Some(evidence),
            Self::NotMatched => None,
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
#[rkyv(
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext)),
    serialize_bounds(
        __S: rkyv::ser::Writer + rkyv::ser::Allocator,
        __S::Error: rkyv::rancor::Source,
    ),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
)]
pub enum MatchEvidence {
    Contains(ContainsEvidence),
    AllOf(CompositeEvidence),
    AnyOf(CompositeEvidence),
    Not,
    Near(NearEvidence),
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
pub struct ContainsEvidence {
    pub term: QueryTerm,
    pub occurrences: Vec<Occurrence>,
}

impl ContainsEvidence {
    pub fn new(term: QueryTerm, occurrences: Vec<Occurrence>) -> Self {
        Self { term, occurrences }
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
#[rkyv(
    bytecheck(bounds(
        __C: rkyv::validation::ArchiveContext,
        __C::Error: rkyv::rancor::Source,
    )),
    serialize_bounds(
        __S: rkyv::ser::Writer + rkyv::ser::Allocator,
        __S::Error: rkyv::rancor::Source,
    ),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
)]
pub struct CompositeEvidence {
    #[rkyv(omit_bounds)]
    pub matches: Vec<MatchEvidence>,
}

impl CompositeEvidence {
    pub fn new(matches: Vec<MatchEvidence>) -> Self {
        Self { matches }
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
pub struct NearEvidence {
    pub left: QueryTerm,
    pub right: QueryTerm,
    pub distance: WordDistance,
    pub pairs: Vec<NearOccurrencePair>,
}

impl NearEvidence {
    pub fn new(
        left: QueryTerm,
        right: QueryTerm,
        distance: WordDistance,
        pairs: Vec<NearOccurrencePair>,
    ) -> Self {
        Self {
            left,
            right,
            distance,
            pairs,
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
pub struct NearOccurrencePair {
    pub left: Occurrence,
    pub right: Occurrence,
    pub gap: WordDistance,
}

impl NearOccurrencePair {
    pub fn new(left: Occurrence, right: Occurrence, gap: WordDistance) -> Self {
        Self { left, right, gap }
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
pub struct Occurrence {
    pub start: u32,
    pub end: u32,
}

impl Occurrence {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn gap_to(&self, other: &Self) -> WordDistance {
        if self.end < other.start {
            WordDistance(other.start - self.end - 1)
        } else if other.end < self.start {
            WordDistance(self.start - other.end - 1)
        } else {
            WordDistance(0)
        }
    }
}
