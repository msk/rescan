use super::{RoseBuild, RoseResources, RoseRuntimeImpl};
use crate::util::CompileContext;

#[derive(Default)]
pub(super) struct BuildContext {
    /// Resources in use (tracked as programs are added).
    pub(super) resources: RoseResources,
}

fn is_pure_floating(resources: &RoseResources, cc: &CompileContext) -> bool {
    if !resources.has_floating {
        return false;
    }

    if resources.has_outfixes || resources.has_suffixes || resources.has_leftfixes {
        return false;
    }

    if resources.has_anchored {
        return false;
    }

    if resources.has_eod {
        return false;
    }

    if resources.has_states {
        return false;
    }

    if resources.has_lit_delay {
        return false;
    }

    if cc.streaming && resources.has_lit_check {
        return false;
    }

    if resources.checks_groups {
        return false;
    }

    true
}

fn is_single_outfix(_tbi: &RoseBuild) -> bool {
    false
}

pub(super) fn pick_runtime_impl(build: &RoseBuild, resources: &RoseResources) -> RoseRuntimeImpl {
    if is_pure_floating(resources, build.cc) {
        return RoseRuntimeImpl::PureLiteral;
    }

    if is_single_outfix(build) {
        return RoseRuntimeImpl::SingleOutfix;
    }

    RoseRuntimeImpl::FullRose
}

pub(super) struct DerivedBoundaryReports {}
