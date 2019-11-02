use crate::SomType;

#[derive(Clone)]
pub(crate) struct ExpressionInfo {
    pub(in crate::compiler) utf8: bool,
    pub(in crate::compiler) prefilter: bool,
    pub(crate) som: SomType,
}
