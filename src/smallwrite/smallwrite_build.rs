//! Small-write engine build code.

use crate::compiler::ExpressionInfo;
use crate::nfagraph::NgHolder;
use crate::util::{CompileContext, ReportId, ReportManager, Ue2Literal};
use crate::SomType;
use petgraph::graph::{DefaultIx, NodeIndex};
use petgraph::{Direction, Graph};
use std::collections::HashSet;
use std::ptr::NonNull;

const MAX_TRIE_VERTICES: usize = 8_000;

struct LitTrieVertexProps {
    /// Character reached on this vertex.
    c: u8,
    /// Managed reports fired on this vertex.
    reports: HashSet<ReportId>,
}

impl LitTrieVertexProps {
    fn with_byte(c: u8) -> Self {
        Self {
            c,
            reports: HashSet::new(),
        }
    }
}

impl Default for LitTrieVertexProps {
    fn default() -> Self {
        Self {
            c: 0,
            reports: HashSet::new(),
        }
    }
}

struct LitTrieEdgeProps;

type LitTrieVertex = NodeIndex<DefaultIx>;

/// A graph used to store a trie of literals (for later AC construction into a
/// DFA).
struct LitTrie {
    inner: Graph<LitTrieVertexProps, LitTrieEdgeProps>,

    root: LitTrieVertex,
}

impl LitTrie {
    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        self.inner.node_count() == 1
    }

    #[allow(dead_code)]
    fn all_reports(&self) -> HashSet<ReportId> {
        self.inner.node_indices().fold(HashSet::new(), |mut s, v| {
            s.extend(&self.inner[v].reports);
            s
        })
    }
}

impl Default for LitTrie {
    fn default() -> Self {
        let mut g = Graph::<LitTrieVertexProps, LitTrieEdgeProps>::default();
        let root = g.add_node(LitTrieVertexProps::default());
        Self { inner: g, root }
    }
}

/// Small-write engine build interface.
pub(crate) struct SmallWriteBuild<'a> {
    pub(crate) rm: NonNull<ReportManager<'a>>,
    cc: &'a CompileContext,

    lit_trie: LitTrie,
    lit_trie_nocase: LitTrie,
    num_literals: usize,
    poisoned: bool,
}

impl<'a> SmallWriteBuild<'a> {
    /// Construct a usable `SmallWrite` builder.
    pub(crate) fn new(num_patterns: usize, cc: &'a CompileContext) -> Self {
        let poisoned =
            !cc.grey.allow_small_write || num_patterns > cc.grey.small_write_max_patterns;
        Self {
            rm: NonNull::dangling(),
            cc,
            lit_trie: LitTrie::default(),
            lit_trie_nocase: LitTrie::default(),
            num_literals: 0,
            poisoned,
        }
    }

    #[allow(clippy::blocks_in_if_conditions)]
    #[allow(dead_code)]
    pub(crate) fn add_expression(&mut self, g: &NgHolder, expr: &ExpressionInfo) {
        // If the graph is poisoned (i.e. we can't build a SmallWrite version),
        // we don't even try.
        if self.poisoned {
            return;
        }

        // No SOM support in small-write engine.
        if expr.som != SomType::None {
            self.poisoned = true;
            return;
        }

        // No vacuous graph support in small-write engine.
        if g.is_vacuous() {
            self.poisoned = true;
            return;
        }

        if g.all_reports()
            .into_iter()
            .any(|id| unsafe { self.rm.as_ref().get_report(id).min_length > 0 })
        {
            self.poisoned = true;
            return;
        }

        unimplemented!();
    }

    pub(crate) fn add_literal(&mut self, literal: &Ue2Literal, r: ReportId) {
        // If the graph is poisoned (i.e. we can't build a `SmallWrite`
        // version), we don't even try.
        if self.poisoned {
            return;
        }

        if literal.len() > self.cc.grey.small_write_largest_buffer {
            return; // too long
        }

        self.num_literals += 1;
        if self.num_literals > self.cc.grey.small_write_max_literals {
            // Exceeded literal limit.
            self.poisoned = true;
            return;
        }

        let trie = if literal.any_nocase() {
            &mut self.lit_trie_nocase
        } else {
            &mut self.lit_trie
        };
        if !add_to_trie(literal, r, trie) {
            self.poisoned = true;
        }
    }
}

fn add_to_trie(literal: &Ue2Literal, report: ReportId, trie: &mut LitTrie) -> bool {
    let mut u = trie.root;
    for c in literal.iter() {
        u = trie
            .inner
            .neighbors_directed(u, Direction::Outgoing)
            .find(|v| trie.inner[*v].c == c.c)
            .unwrap_or_else(|| {
                let next = trie.inner.add_node(LitTrieVertexProps::with_byte(c.c));
                trie.inner.add_edge(u, next, LitTrieEdgeProps {});
                next
            });
    }

    trie.inner[u].reports.insert(report);

    trie.inner.node_count() <= MAX_TRIE_VERTICES
}
