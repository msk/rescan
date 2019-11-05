use crate::parser::PosFlags;
use petgraph::graph::{DefaultIx, NodeIndex};
use petgraph::{Direction, Graph};
use rescan_util::{CharReach, ReportId};
use std::collections::HashSet;
use std::ops::{Index, IndexMut};

#[derive(Default)]
pub(crate) struct NfaGraphVertexProps {
    /// Set of characters on which this vertex is reachable.
    pub(in crate::nfagraph) char_reach: CharReach,

    /// Set of reports raised by this vertex.
    pub(super) reports: HashSet<ReportId>,

    /// Flags associated with assertions.
    pub(in crate::nfagraph) assert_flags: PosFlags,
}

#[derive(Default)]
struct NfaGraphEdgeProps {}

pub(crate) struct NgHolder {
    inner: Graph<NfaGraphVertexProps, NfaGraphEdgeProps>,

    pub(in crate::nfagraph) start: NfaVertex,
    pub(in crate::nfagraph) start_ds: NfaVertex,
    pub(in crate::nfagraph) accept: NfaVertex,
    pub(in crate::nfagraph) accept_eod: NfaVertex,
}

impl NgHolder {
    pub(crate) fn new() -> Self {
        let mut inner = Graph::<NfaGraphVertexProps, NfaGraphEdgeProps>::new();

        // Add initial special nodes.
        let start = add_special_vertex(&mut inner);
        let start_ds = add_special_vertex(&mut inner);
        let accept = add_special_vertex(&mut inner);
        let accept_eod = add_special_vertex(&mut inner);

        // Wire up some fake edges for the stylized bits of the NFA.
        inner.add_edge(start, start_ds, NfaGraphEdgeProps::default());
        inner.add_edge(start_ds, start_ds, NfaGraphEdgeProps::default());
        inner.add_edge(accept, accept_eod, NfaGraphEdgeProps::default());

        inner[start].char_reach.set_all();
        inner[start_ds].char_reach.set_all();

        Self {
            inner,
            start,
            start_ds,
            accept,
            accept_eod,
        }
    }

    pub(in crate::nfagraph) fn add_vertex(&mut self) -> NfaVertex {
        let v = NfaGraphVertexProps::default();
        self.inner.add_node(v)
    }

    pub(in crate::nfagraph) fn num_vertices(&self) -> usize {
        self.inner.node_count()
    }

    /// Returns the set of all reports in the graph.
    pub(crate) fn all_reports(&self) -> HashSet<ReportId> {
        let reports = self
            .inner
            .neighbors_directed(self.accept, Direction::Incoming)
            .fold(HashSet::new(), |mut union, v| {
                union.extend(&self.inner[v].reports);
                union
            });
        self.inner
            .neighbors_directed(self.accept_eod, Direction::Incoming)
            .fold(reports, |mut union, v| {
                union.extend(&self.inner[v].reports);
                union
            })
    }

    // Miscellaneous NFA graph utilities.

    /// Returns `true` if the graph contains an edge from one of {`start`,
    /// `start_ds`} to one of {`accept`, `accept_eod`}.
    pub(crate) fn is_vacuous(&self) -> bool {
        self.inner.find_edge(self.start, self.accept).is_some()
            || self.inner.find_edge(self.start, self.accept_eod).is_some()
            || self.inner.find_edge(self.start_ds, self.accept).is_some()
            || self
                .inner
                .find_edge(self.start_ds, self.accept_eod)
                .is_some()
    }
}

pub(crate) type NfaVertex = NodeIndex<DefaultIx>;

impl Index<NfaVertex> for NgHolder {
    type Output = NfaGraphVertexProps;

    fn index(&self, index: NfaVertex) -> &NfaGraphVertexProps {
        self.inner.node_weight(index).expect("invalid index")
    }
}

impl IndexMut<NfaVertex> for NgHolder {
    fn index_mut(&mut self, index: NfaVertex) -> &mut NfaGraphVertexProps {
        self.inner.node_weight_mut(index).expect("invalid index")
    }
}

fn add_special_vertex(g: &mut Graph<NfaGraphVertexProps, NfaGraphEdgeProps>) -> NfaVertex {
    let v = NfaGraphVertexProps::default();
    g.add_node(v)
}
