use crate::parser::{PosFlags, Position};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(crate) struct PositionInfo {
    pub(in crate::parser) pos: Position,
    pub(in crate::parser) flags: PosFlags,
}

impl From<Position> for PositionInfo {
    fn from(pos: Position) -> Self {
        Self {
            pos,
            flags: PosFlags::default(),
        }
    }
}
