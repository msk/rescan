use petgraph::graph::{DefaultIx, NodeIndex};
use petgraph::Graph;
use std::ops::{Index, IndexMut};

use crate::parser::PosFlags;
use rescan_util::CharReach;

#[derive(Default)]
pub(crate) struct NfaGraphVertexProps {
    /// Set of characters on which this vertex is reachable.
    pub(in crate::nfagraph) char_reach: CharReach,

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

        NgHolder {
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
