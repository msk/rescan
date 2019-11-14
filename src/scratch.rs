pub(crate) struct CoreInfo {
    pub(crate) state: *mut u8,
}

pub struct Scratch {
    pub(crate) core_info: CoreInfo,
}
