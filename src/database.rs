use crate::rose::RoseEngine;

pub struct Database {
    rose: RoseEngine,
}

impl Database {
    pub(crate) fn new(rose: RoseEngine) -> Self {
        Database { rose }
    }
}

pub(crate) fn get_bytecode(db: &Database) -> &RoseEngine {
    &db.rose
}
