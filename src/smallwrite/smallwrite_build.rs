use crate::compiler::ExpressionInfo;
use crate::nfagraph::NgHolder;
use crate::util::{CompileContext, ReportManager};
use crate::SomType;
use std::ptr::NonNull;

/// Small-write engine build interface.
pub(crate) struct SmallWriteBuild<'a> {
    pub(crate) rm: NonNull<ReportManager<'a>>,
    _cc: &'a CompileContext,

    poisoned: bool,
}

impl<'a> SmallWriteBuild<'a> {
    /// Construct a usable `SmallWrite` builder.
    pub(crate) fn new(num_patterns: usize, cc: &'a CompileContext) -> Self {
        let poisoned =
            !cc.grey.allow_small_write || num_patterns > cc.grey.small_write_max_patterns;
        Self {
            rm: NonNull::dangling(),
            _cc: cc,
            poisoned,
        }
    }

    #[allow(dead_code)]
    fn add(&mut self, g: &NgHolder, expr: &ExpressionInfo) {
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
    }
}
