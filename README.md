# nota-text-query

Engine-neutral typed text query language with readable NOTA and rkyv surfaces.

Core query variants:

- `Contains(Word(...))`
- `Contains(Phrase([...]))`
- `AllOf([...])`
- `AnyOf([...])`
- `Not(...)`
- `(Near ((Word (left)) (Word (right)) distance))`

The crate reports deterministic match evidence. Ranking and field scoring are
left to consumers.
