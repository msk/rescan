use std::collections::{HashMap, HashSet};

use crate::nfagraph::{NfaBuilder, NfaVertex};
use crate::parser::{PosFlags, Position, PositionInfo};

pub(crate) struct GlushkovBuildState<'a> {
    start_state: Position,
    start_dot_star_state: Position,
    accept_state: Position,
    accept_eod_state: Position,

    builder: &'a mut NfaBuilder,

    /// Map storing successors for each position.
    successors: HashMap<Position, HashSet<PositionInfo>>,
}

impl<'a> GlushkovBuildState<'a> {
    fn new(b: &'a mut NfaBuilder) -> Self {
        let start_state = b.get_start();
        let start_dot_star_state = b.get_start_dot_star();
        let accept_state = b.get_accept();
        let accept_eod_state = b.get_accept_eod();

        let mut bs = GlushkovBuildState {
            start_state,
            start_dot_star_state,
            accept_state,
            accept_eod_state,
            builder: b,
            successors: HashMap::default(),
        };

        let lasts: Vec<PositionInfo> = vec![start_state.into(), start_dot_star_state.into()];
        let firsts: Vec<PositionInfo> = vec![start_dot_star_state.into()];
        bs.connect_regions(&lasts, &firsts);

        bs
    }

    pub(in crate::parser) fn get_builder(&self) -> &NfaBuilder {
        self.builder
    }

    pub(in crate::parser) fn get_builder_mut(&mut self) -> &mut NfaBuilder {
        self.builder
    }

    pub(crate) fn connect_regions(&mut self, lasts: &[PositionInfo], firsts: &[PositionInfo]) {
        for last in lasts {
            self.connect_successors(*last, firsts.to_vec());
        }
    }

    fn connect_successors(&mut self, from: PositionInfo, mut tolist: Vec<PositionInfo>) {
        filter_edges(self, from, &mut tolist);

        let i = tolist.iter().position(|e| e.pos == self.accept_state);
        if let Some(i) = i {
            let fakedot = self.builder.make_position();
            tolist[i] = fakedot.into();
        }

        let succ = &mut self.successors.get_mut(&from.pos).expect("invalid key");

        for to in tolist.iter() {
            succ.insert(*to);
        }
    }

    fn pos_epsilon() -> Position {
        Position::new(NfaVertex::end().index() - 1)
    }
}

fn filter_edges(bs: &GlushkovBuildState, from: PositionInfo, tolist: &mut Vec<PositionInfo>) {
    if from.pos == bs.start_dot_star_state {
        // If we're connecting from start-dotstar, remove all caret flavored
        // positions.
        tolist.retain(|e| !e.flags.contains(PosFlags::NOFLOAT));
        if from.flags.contains(PosFlags::NOFLOAT) {
            tolist.clear();
        }
    } else if from.pos == bs.start_state {
        // If we're connecting from start, we should remove any epsilons that
        // aren't caret flavored.
        tolist.retain(|e| e.pos != GlushkovBuildState::pos_epsilon() || e.flags.is_empty());
        tolist.retain(|e| !e.flags.contains(PosFlags::MUST_FLOAT | PosFlags::NOFLOAT));
    }

    if bs
        .builder
        .get_assert_flag(from.pos)
        .contains(PosFlags::MULTILINE_START)
    {
        // If we have a (mildly boneheaded) pattern like /^$/m, we're right up
        // against the edge of what we can do without true assertion support.
        // Here we have an evil hack to prevent us plugging the `\n` generated
        // by the caret right into accept_eod (which is in the firsts of the
        // dollar).
        //
        // This is due to the 'interesting quirk' that multiline ^ does not not
        // match a newline at the end of buffer.
        tolist.retain(|e| e.pos != bs.accept_eod_state);
    }
}

pub(crate) fn make_glushkov_build_state(b: &mut NfaBuilder) -> GlushkovBuildState {
    GlushkovBuildState::new(b)
}
