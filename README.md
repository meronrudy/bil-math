# BIL Math

Pure Rust mathematical foundations for studying regulated AI assurance as an
executable proof spine.

This repository is organized for PhD-level mathematical development. The goal
is to keep each central definition readable as mathematics, executable as Rust,
and traceable through tests and small examples.

## Study Thesis

Regulated AI assurance is treated here as a consistency problem over
institutional workflows.

Local systems such as underwriting, model service, compliance review, human
approval, and audit replay each carry local semantic state. BIL Math studies
whether those local states remain coherent when transported through the
workflow. In the first proof thread, a nontrivial loop transport is a holonomy
obstruction: after moving around a closed institutional loop, the state does
not return to itself.

## Current Proof Spine

The first worked chapter is holonomy obstruction theory:

- column vectors represent local semantic states;
- edge transport maps source state to target state;
- ordered path holonomy composes maps right-to-left;
- cycle holonomy is valid only for graph-closed, dimension-compatible loops;
- obstruction is the norm of the deviation from identity.

Start with:

- [docs/README.md](docs/README.md) for the study map;
- [docs/notation.md](docs/notation.md) for conventions;
- [docs/holonomy-obstruction.md](docs/holonomy-obstruction.md) for the main
  chapter;
- [docs/trace-matrix.md](docs/trace-matrix.md) for theorem-to-code links.

## Rust Workspace

The crates are intentionally small:

- `bil-math-core`: IDs, institutional graphs, scalar/vector/matrix aliases,
  and shared errors.
- `bil-math-holonomy`: paths, cycles, transport maps, holonomy, and
  obstruction measurement.
- `bil-math-sheaf`: cellular sheaf primitives, coboundary, Laplacian,
  residuals, and global-section checks.
- `bil-math-lattice`: finite policy-order primitives and only-tighten checks.
- `bil-math-provenance`: decision-event DAGs and replay invariants.
- `bil-math-examples`: executable study examples.
- `bil-math-cli`: command-line entrypoints for examples.

## Run

```sh
cargo test --workspace
cargo run -p bil-math-cli -- holonomy-loop
cargo run -p bil-math-cli -- toy-bank
```

The holonomy example prints obstruction data for a closed institutional loop:

```text
holonomy_identity_deviation=0.15
holonomy_trace=2
holonomy_tolerance=0.000000001
holonomy_is_trivial=false
```

## Repository Boundary

This repo contains mathematical definitions, typed Rust reference
implementations, proof notes, tests, and toy examples.

It does not contain production BIL APIs, customer data, bank integrations,
pricing, investor materials, or hardware RTL. It also does not introduce Lean,
LaTeX build tooling, or mdBook in this pass; the proof spine is plain Markdown
linked to executable Rust witnesses.
