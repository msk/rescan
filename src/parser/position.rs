use bitflags::bitflags;

use crate::nfagraph::NfaVertex;

bitflags! {
    #[derive(Default)]
    pub(crate) struct PosFlags: u32 {
        const NOFLOAT = 0x0000_0001; // don't wire to start-dotstar
        const MUST_FLOAT = 0x0000_0010; // don't wire solely to start
        const MULTILINE_START = 0x0000_0100;
    }
}

pub(crate) type Position = NfaVertex;
