//! Engine-neutral typed text queries with rkyv and optional NOTA surfaces.
//!
//! `nota-text-query` provides a readable query AST and deterministic matching
//! evidence. Consumers own field selection, ranking, scoring, and storage.

pub mod error;
pub mod evidence;
pub mod query;
pub mod text;

pub use error::{Error, Result};
pub use evidence::{
    CompositeEvidence, ContainsEvidence, MatchEvidence, NearEvidence, Occurrence, SearchOutcome,
};
pub use query::{NearQuery, Query, QueryTerm, SearchPhrase, SearchWord, WordDistance};
pub use text::{NormalizedWord, SearchText};
