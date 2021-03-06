use crate::compiler::{BuiltExpression, ExpressionInfo, ParsedExpression};
use crate::nfagraph::{NfaVertex, NgHolder};
use crate::parser::{PosFlags, Position};
use crate::util::CharReach;

pub(crate) struct NfaBuilder {
    graph: NgHolder,
    expr: ExpressionInfo,
}

impl NfaBuilder {
    fn new(parsed: &ParsedExpression) -> Self {
        let graph = NgHolder::new();
        Self {
            graph,
            expr: parsed.expr.clone(),
        }
    }

    pub(crate) fn make_position(&mut self) -> Position {
        self.add_vertex()
    }

    pub(crate) fn get_start(&self) -> Position {
        self.graph.start
    }

    pub(crate) fn get_start_dot_star(&self) -> Position {
        self.graph.start_ds
    }

    pub(crate) fn get_accept(&self) -> Position {
        self.graph.accept
    }

    pub(crate) fn get_accept_eod(&self) -> Position {
        self.graph.accept_eod
    }

    pub(crate) fn add_char_reach(&mut self, pos: NfaVertex, cr: CharReach) {
        self.graph[pos].char_reach |= cr;
    }

    pub(crate) fn get_assert_flag(&self, pos: NfaVertex) -> PosFlags {
        self.graph[pos].assert_flags
    }

    fn add_vertex(&mut self) -> NfaVertex {
        self.graph.add_vertex()
    }

    #[allow(dead_code)]
    pub(crate) fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    pub(crate) fn get_graph(self) -> BuiltExpression {
        BuiltExpression {
            expr: self.expr,
            g: self.graph,
        }
    }
}

pub(crate) fn make_nfa_builder(expr: &ParsedExpression) -> NfaBuilder {
    NfaBuilder::new(expr)
}
