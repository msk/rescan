use crate::rose::RoseEngine;

pub(crate) struct RoseBuild {
    pub(crate) has_som: bool,
}

impl RoseBuild {
    pub(crate) fn build_rose(&self) -> RoseEngine {
        RoseEngine {}
    }
}
