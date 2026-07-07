use crate::{Cycle, DiscreteConnection, HolonomyObstruction};
use bil_math_core::{DirectedEdge, EdgeId, InstitutionalGraph, MathError, Matrix};

pub fn path_holonomy(
    graph: &InstitutionalGraph,
    connection: &DiscreteConnection,
    edge_ids: &[EdgeId],
) -> Result<Matrix, MathError> {
    if edge_ids.is_empty() {
        return Err(MathError::InvalidPath(
            "cannot compute holonomy over an empty path".to_string(),
        ));
    }

    let edges = resolve_path_edges(graph, edge_ids)?;
    validate_path_adjacency(&edges)?;
    compose_edge_maps(connection, &edges)
}

pub fn cycle_holonomy(
    graph: &InstitutionalGraph,
    connection: &DiscreteConnection,
    cycle: &Cycle,
) -> Result<Matrix, MathError> {
    if cycle.is_empty() {
        return Err(MathError::InvalidCycle(
            "cannot compute holonomy over an empty cycle".to_string(),
        ));
    }

    let edges = resolve_path_edges(graph, cycle.edges())?;
    validate_path_adjacency(&edges)?;
    validate_cycle_closure(&edges)?;

    let holonomy = compose_edge_maps(connection, &edges)?;

    if holonomy.nrows() != holonomy.ncols() {
        return Err(MathError::NonSquareMatrix {
            rows: holonomy.nrows(),
            cols: holonomy.ncols(),
        });
    }

    Ok(holonomy)
}

pub fn cycle_obstruction(
    graph: &InstitutionalGraph,
    connection: &DiscreteConnection,
    cycle: &Cycle,
) -> Result<HolonomyObstruction, MathError> {
    let holonomy = cycle_holonomy(graph, connection, cycle)?;
    HolonomyObstruction::try_from_holonomy(holonomy)
}

fn resolve_path_edges<'a>(
    graph: &'a InstitutionalGraph,
    edge_ids: &[EdgeId],
) -> Result<Vec<&'a DirectedEdge>, MathError> {
    edge_ids
        .iter()
        .map(|edge_id| {
            graph
                .edge(*edge_id)
                .ok_or(MathError::MissingEdge(edge_id.0))
        })
        .collect()
}

fn validate_path_adjacency(edges: &[&DirectedEdge]) -> Result<(), MathError> {
    for window in edges.windows(2) {
        let previous = window[0];
        let next = window[1];

        if previous.target != next.source {
            return Err(MathError::InvalidPath(format!(
                "edge {:?} targets {:?}, but next edge {:?} starts at {:?}",
                previous.id, previous.target, next.id, next.source
            )));
        }
    }

    Ok(())
}

fn validate_cycle_closure(edges: &[&DirectedEdge]) -> Result<(), MathError> {
    let first = edges
        .first()
        .expect("cycle closure validation requires a non-empty edge list");
    let last = edges
        .last()
        .expect("cycle closure validation requires a non-empty edge list");

    if last.target != first.source {
        return Err(MathError::InvalidCycle(format!(
            "cycle starts at {:?}, but ends at {:?}",
            first.source, last.target
        )));
    }

    Ok(())
}

fn compose_edge_maps(
    connection: &DiscreteConnection,
    edges: &[&DirectedEdge],
) -> Result<Matrix, MathError> {
    let first = edges
        .first()
        .expect("path composition requires a non-empty edge list");
    let first_map = connection
        .transport(first.id)
        .ok_or(MathError::MissingEdge(first.id.0))?;

    let mut accumulated = Matrix::identity(first_map.ncols(), first_map.ncols());

    for edge in edges {
        let map = connection
            .transport(edge.id)
            .ok_or(MathError::MissingEdge(edge.id.0))?;

        if map.ncols() != accumulated.nrows() {
            return Err(MathError::DimensionMismatch {
                expected: accumulated.nrows(),
                actual: map.ncols(),
            });
        }

        accumulated = map * accumulated;
    }

    Ok(accumulated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bil_math_core::{Scalar, VertexId};
    use proptest::prelude::*;
    use std::collections::BTreeMap;

    fn triangle_graph() -> InstitutionalGraph {
        let a = VertexId(0);
        let b = VertexId(1);
        let c = VertexId(2);

        InstitutionalGraph::new(
            vec![a, b, c],
            vec![
                DirectedEdge {
                    id: EdgeId(0),
                    source: a,
                    target: b,
                },
                DirectedEdge {
                    id: EdgeId(1),
                    source: b,
                    target: c,
                },
                DirectedEdge {
                    id: EdgeId(2),
                    source: c,
                    target: a,
                },
            ],
        )
    }

    fn line_graph() -> InstitutionalGraph {
        let a = VertexId(0);
        let b = VertexId(1);
        let c = VertexId(2);
        let d = VertexId(3);

        InstitutionalGraph::new(
            vec![a, b, c, d],
            vec![
                DirectedEdge {
                    id: EdgeId(0),
                    source: a,
                    target: b,
                },
                DirectedEdge {
                    id: EdgeId(1),
                    source: b,
                    target: c,
                },
                DirectedEdge {
                    id: EdgeId(2),
                    source: c,
                    target: d,
                },
            ],
        )
    }

    fn connection(transports: Vec<(EdgeId, Matrix)>) -> DiscreteConnection {
        DiscreteConnection::new(transports.into_iter().collect::<BTreeMap<_, _>>())
    }

    fn identity_connection(dim: usize) -> DiscreteConnection {
        connection(vec![
            (EdgeId(0), Matrix::identity(dim, dim)),
            (EdgeId(1), Matrix::identity(dim, dim)),
            (EdgeId(2), Matrix::identity(dim, dim)),
        ])
    }

    fn max_abs_diff(left: &Matrix, right: &Matrix) -> Scalar {
        (left - right)
            .iter()
            .fold(0.0, |max, value| max.max(value.abs()))
    }

    #[test]
    fn hol_lem_001_identity_loop_has_zero_obstruction() {
        let graph = triangle_graph();
        let connection = identity_connection(2);
        let cycle = Cycle::new(vec![EdgeId(0), EdgeId(1), EdgeId(2)]);

        let obstruction = cycle_obstruction(&graph, &connection, &cycle).unwrap();

        assert!(obstruction.is_trivial(1e-12));
        assert_eq!(obstruction.identity_deviation, 0.0);
    }

    #[test]
    fn hol_thm_001_non_identity_loop_has_positive_obstruction() {
        let graph = triangle_graph();
        let connection = connection(vec![
            (EdgeId(0), Matrix::from_diagonal_element(2, 2, 1.0)),
            (EdgeId(1), Matrix::from_diagonal_element(2, 2, 1.0)),
            (
                EdgeId(2),
                Matrix::from_row_slice(2, 2, &[1.0, 0.25, 0.0, 1.0]),
            ),
        ]);
        let cycle = Cycle::new(vec![EdgeId(0), EdgeId(1), EdgeId(2)]);

        let obstruction = cycle_obstruction(&graph, &connection, &cycle).unwrap();

        assert!(obstruction.identity_deviation > 0.0);
        assert!(!obstruction.is_trivial(1e-12));
    }

    #[test]
    fn hol_def_003_missing_transport_returns_missing_edge() {
        let graph = triangle_graph();
        let connection = connection(vec![
            (EdgeId(0), Matrix::identity(2, 2)),
            (EdgeId(1), Matrix::identity(2, 2)),
        ]);
        let cycle = Cycle::new(vec![EdgeId(0), EdgeId(1), EdgeId(2)]);

        let error = cycle_holonomy(&graph, &connection, &cycle).unwrap_err();

        assert!(matches!(error, MathError::MissingEdge(2)));
    }

    #[test]
    fn hol_lem_003_non_composable_path_returns_adjacency_error() {
        let graph = triangle_graph();
        let connection = identity_connection(2);

        let error = path_holonomy(&graph, &connection, &[EdgeId(0), EdgeId(2)]).unwrap_err();

        assert!(matches!(error, MathError::InvalidPath(_)));
    }

    #[test]
    fn hol_lem_003_non_composable_path_returns_dimension_error() {
        let graph = triangle_graph();
        let connection = connection(vec![
            (EdgeId(0), Matrix::identity(2, 2)),
            (EdgeId(1), Matrix::identity(3, 3)),
            (EdgeId(2), Matrix::identity(3, 3)),
        ]);

        let error = path_holonomy(&graph, &connection, &[EdgeId(0), EdgeId(1)]).unwrap_err();

        assert!(matches!(
            error,
            MathError::DimensionMismatch {
                expected: 2,
                actual: 3
            }
        ));
    }

    #[test]
    fn hol_thm_002_non_closed_cycle_is_rejected() {
        let graph = triangle_graph();
        let connection = identity_connection(2);
        let cycle = Cycle::new(vec![EdgeId(0), EdgeId(1)]);

        let error = cycle_holonomy(&graph, &connection, &cycle).unwrap_err();

        assert!(matches!(error, MathError::InvalidCycle(_)));
    }

    #[test]
    fn hol_thm_002_non_square_cycle_holonomy_is_rejected() {
        let graph = triangle_graph();
        let connection = connection(vec![
            (EdgeId(0), Matrix::identity(3, 2)),
            (EdgeId(1), Matrix::identity(4, 3)),
            (EdgeId(2), Matrix::identity(5, 4)),
        ]);
        let cycle = Cycle::new(vec![EdgeId(0), EdgeId(1), EdgeId(2)]);

        let error = cycle_holonomy(&graph, &connection, &cycle).unwrap_err();

        assert!(matches!(
            error,
            MathError::NonSquareMatrix { rows: 5, cols: 2 }
        ));
    }

    #[test]
    fn hol_lem_002_path_composition_is_right_to_left() {
        let graph = line_graph();
        let first = Matrix::from_row_slice(2, 2, &[1.0, 2.0, 0.0, 1.0]);
        let second = Matrix::from_row_slice(2, 2, &[2.0, 0.0, 0.0, 3.0]);
        let third = Matrix::from_row_slice(2, 2, &[1.0, 0.0, 4.0, 1.0]);
        let connection = connection(vec![
            (EdgeId(0), first.clone()),
            (EdgeId(1), second.clone()),
            (EdgeId(2), third.clone()),
        ]);

        let holonomy =
            path_holonomy(&graph, &connection, &[EdgeId(0), EdgeId(1), EdgeId(2)]).unwrap();

        assert_eq!(holonomy, third * second * first);
    }

    proptest! {
        #[test]
        fn hol_lem_001_identity_connection_property_over_valid_cycle(dim in 1usize..6) {
            let graph = triangle_graph();
            let connection = identity_connection(dim);
            let cycle = Cycle::new(vec![EdgeId(0), EdgeId(1), EdgeId(2)]);

            let obstruction = cycle_obstruction(&graph, &connection, &cycle).unwrap();

            prop_assert!(obstruction.is_trivial(1e-12));
        }

        #[test]
        fn hol_lem_002_path_composition_is_associative_for_compatible_maps(
            first_entries in proptest::collection::vec(-3i32..=3, 4),
            second_entries in proptest::collection::vec(-3i32..=3, 4),
            third_entries in proptest::collection::vec(-3i32..=3, 4),
        ) {
            let graph = line_graph();
            let first_values = first_entries.iter().map(|value| *value as Scalar).collect::<Vec<_>>();
            let second_values = second_entries.iter().map(|value| *value as Scalar).collect::<Vec<_>>();
            let third_values = third_entries.iter().map(|value| *value as Scalar).collect::<Vec<_>>();
            let first = Matrix::from_row_slice(2, 2, &first_values);
            let second = Matrix::from_row_slice(2, 2, &second_values);
            let third = Matrix::from_row_slice(2, 2, &third_values);
            let connection = connection(vec![
                (EdgeId(0), first.clone()),
                (EdgeId(1), second.clone()),
                (EdgeId(2), third.clone()),
            ]);

            let holonomy = path_holonomy(&graph, &connection, &[EdgeId(0), EdgeId(1), EdgeId(2)])
                .unwrap();
            let left_grouping = (&third * &second) * &first;
            let right_grouping = &third * (&second * &first);

            prop_assert!(max_abs_diff(&holonomy, &right_grouping) <= 1e-9);
            prop_assert!(max_abs_diff(&left_grouping, &right_grouping) <= 1e-9);
        }
    }
}
