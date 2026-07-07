use bil_math_core::{DirectedEdge, EdgeId, InstitutionalGraph, Matrix, Scalar, VertexId};
use bil_math_holonomy::{cycle_obstruction, Cycle, DiscreteConnection};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub struct HolonomyLoopReport {
    pub identity_deviation: Scalar,
    pub holonomy_trace: Scalar,
    pub tolerance: Scalar,
    pub is_trivial: bool,
}

pub fn run_holonomy_loop_obstruction() -> HolonomyLoopReport {
    let underwriting = VertexId(0);
    let model_service = VertexId(1);
    let compliance = VertexId(2);

    let edge_underwriting_to_model = EdgeId(0);
    let edge_model_to_compliance = EdgeId(1);
    let edge_compliance_to_underwriting = EdgeId(2);

    let graph = InstitutionalGraph::new(
        vec![underwriting, model_service, compliance],
        vec![
            DirectedEdge {
                id: edge_underwriting_to_model,
                source: underwriting,
                target: model_service,
            },
            DirectedEdge {
                id: edge_model_to_compliance,
                source: model_service,
                target: compliance,
            },
            DirectedEdge {
                id: edge_compliance_to_underwriting,
                source: compliance,
                target: underwriting,
            },
        ],
    );

    let mut transports = BTreeMap::new();
    transports.insert(edge_underwriting_to_model, Matrix::identity(2, 2));
    transports.insert(
        edge_model_to_compliance,
        Matrix::from_row_slice(2, 2, &[1.0, 0.15, 0.0, 1.0]),
    );
    transports.insert(edge_compliance_to_underwriting, Matrix::identity(2, 2));

    let connection = DiscreteConnection::new(transports);
    let cycle = Cycle::new(vec![
        edge_underwriting_to_model,
        edge_model_to_compliance,
        edge_compliance_to_underwriting,
    ]);
    let obstruction = cycle_obstruction(&graph, &connection, &cycle).unwrap();
    let tolerance = 1e-9;
    let holonomy_trace = (0..obstruction.holonomy.nrows())
        .map(|index| obstruction.holonomy[(index, index)])
        .sum();

    HolonomyLoopReport {
        identity_deviation: obstruction.identity_deviation,
        holonomy_trace,
        tolerance,
        is_trivial: obstruction.is_trivial(tolerance),
    }
}
