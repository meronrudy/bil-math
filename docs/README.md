# BIL Math Study Map

This directory is the plain-Markdown proof spine for the repository. It is
designed to be readable on GitHub and directly cross-linked to executable Rust
tests.

## Reading Order

1. [notation.md](notation.md): shared notation and composition conventions.
2. [proof-spine.md](proof-spine.md): definition, lemma, and theorem index.
3. [holonomy-obstruction.md](holonomy-obstruction.md): the first worked proof
   chapter.
4. [trace-matrix.md](trace-matrix.md): mapping from math claims to Rust APIs,
   tests, and examples.

## Current Focus

The first serious mathematical thread is holonomy as obstruction theory:

- institutional workflows are directed graphs;
- local semantic states are column vectors;
- edge transports move state from source vertex to target vertex;
- a closed loop has trivial semantics only when its holonomy is identity;
- nontrivial deviation from identity is the obstruction.

Sheaf residuals, policy lattices, and provenance DAGs remain supporting
chapters until the holonomy spine is stable.

## Validation Contract

Each named holonomy claim should have at least one executable witness in Rust.
The source of truth for that mapping is [trace-matrix.md](trace-matrix.md).
