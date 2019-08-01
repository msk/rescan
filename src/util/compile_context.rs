use crate::Grey;

pub(crate) struct CompileContext {
    pub(crate) grey: Grey,
}

impl CompileContext {
    pub(crate) fn new(grey: &Grey) -> Self {
        CompileContext { grey: grey.clone() }
    }
}
