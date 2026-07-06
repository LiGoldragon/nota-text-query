# nota-text-query Architecture

`nota-text-query` is a reusable, engine-neutral Rust library for a readable text
search language. Its public query shape is a typed AST, not regex syntax and not
a Spirit-specific search schema.

The crate owns:

- Query AST records and variants for word, phrase, boolean composition,
  negation, and near matching.
- Deterministic matching semantics over caller-provided text.
- Deterministic match evidence that consumers can inspect.
- NOTA and rkyv derive surfaces for the query and evidence types.

The crate does not own:

- Consumer-specific fields such as Spirit descriptions or component schema
  fields.
- Ranking, scoring, weighting, or field-specific boosts.
- Daemons, storage, subscriptions, or engine state.

Matching is case-insensitive through the crate's normalization path. Consumers
may rank or score the returned evidence, but the library only reports whether
the query matched and where the matching terms occurred.
