# Proof Spine

This file is the theorem and dependency index. The first chapter is holonomy
obstruction theory.

## Holonomy Definitions

- `HOL-DEF-001`: An institutional path is a non-empty ordered edge list with
  matching adjacent endpoints.
- `HOL-DEF-002`: An institutional cycle is a non-empty institutional path whose
  final target is its initial source.
- `HOL-DEF-003`: An edge transport maps source-state column vectors to
  target-state column vectors.
- `HOL-DEF-004`: Path holonomy is the right-to-left ordered product of edge
  transports.
- `HOL-DEF-005`: Cycle holonomy is path holonomy for a graph-closed cycle and
  must be square.
- `HOL-DEF-006`: Holonomy obstruction is `||H - I||` for square cycle holonomy
  `H`.

## Holonomy Lemmas

- `HOL-LEM-001`: An identity connection has zero obstruction around any valid
  cycle.
- `HOL-LEM-002`: Compatible path composition is associative and agrees with
  right-to-left ordered multiplication on column vectors.
- `HOL-LEM-003`: Invalid paths or cycles are rejected before obstruction
  measurement.

## Holonomy Theorems

- `HOL-THM-001`: A closed loop with non-identity holonomy has positive
  obstruction.
- `HOL-THM-002`: Cycle obstruction is defined only for closed cycles whose
  composed holonomy is square.

## Dependency Order

```text
HOL-DEF-001 path
  -> HOL-DEF-002 cycle
  -> HOL-DEF-003 transport
  -> HOL-DEF-004 path holonomy
  -> HOL-DEF-005 cycle holonomy
  -> HOL-DEF-006 obstruction
  -> HOL-LEM-001 identity triviality
  -> HOL-LEM-002 composition law
  -> HOL-LEM-003 rejection of invalid structure
  -> HOL-THM-001 nontrivial loop obstruction
  -> HOL-THM-002 obstruction domain
```

The executable mapping is maintained in [trace-matrix.md](trace-matrix.md).
