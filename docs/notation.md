# Notation

## Graphs

An institutional graph is a directed graph `G = (V, E)`.

- `v in V` is an institutional site.
- `e in E` is an oriented workflow edge.
- `s(e)` is the source vertex of `e`.
- `t(e)` is the target vertex of `e`.

An ordered edge list `(e0, ..., en)` is a path when
`t(ei) = s(ei+1)` for every adjacent pair. It is a cycle when it is non-empty,
is a path, and `t(en) = s(e0)`.

## Linear State

All local states are column vectors. A transport map on an edge
`e: u -> v` is a matrix

```text
T(e): X_u -> X_v
```

where `X_u` is the source coordinate space and `X_v` is the target coordinate
space.

## Composition

For an ordered path `(e0, e1, e2)`, the path holonomy is

```text
H(e0, e1, e2) = T(e2) * T(e1) * T(e0)
```

This is right-to-left composition because column vectors are acted on from the
left:

```text
x_final = T(e2) * T(e1) * T(e0) * x_initial
```

## Cycle Obstruction

For a closed cycle `c`, cycle holonomy `H(c)` must be square before it can be
compared with identity. The holonomy obstruction is measured as

```text
obstruction(c) = || H(c) - I ||
```

The current Rust implementation uses the matrix norm supplied by `nalgebra`.
A cycle is trivial at tolerance `epsilon` when

```text
|| H(c) - I || <= epsilon
```

## Error Conditions

The executable proof spine rejects:

- empty paths;
- empty cycles;
- edge IDs missing from the institutional graph;
- edge transports missing from the connection;
- adjacent edges that do not connect;
- non-closed cycles;
- matrix dimensions that do not compose;
- non-square cycle holonomy before obstruction measurement.
