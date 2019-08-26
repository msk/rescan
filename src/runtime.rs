use crate::database::{get_bytecode, Database};
use crate::rose::{rose_block_exec, RoseEngine};

fn raw_block_exec(rose: &RoseEngine) {
    rose_block_exec(rose)
}

pub fn scan(db: &Database, _data: &[u8]) {
    let rose = get_bytecode(db);

    raw_block_exec(rose);
}
