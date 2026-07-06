# nota-text-query agent instructions

Read `ARCHITECTURE.md` before changing the public query types or matching
semantics.

This repo is a Rust library crate. Keep it engine-neutral: no Spirit-specific
fields, dependencies, ranking policy, storage schema, daemon behavior, or CLI
surface belongs here.

Use Jujutsu for version control. Run the narrow Rust check through the flake
before committing changes that touch the crate surface.
