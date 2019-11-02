use crate::database::{get_bytecode, Database};
use crate::rose::{rose_block_exec, RoseEngine};
use crate::scratch::Scratch;

pub enum Error {
    /// A parameter passed to this function was invalid.
    Invalid,
    /// The engine was terminated by callback.
    ScanTerminated,
    /// Unexpected internal error.
    Unknown,
}

fn raw_block_exec(rose: &RoseEngine) {
    rose_block_exec(rose)
}

pub fn scan(db: &Database, _data: &[u8], _scratch: &mut Scratch) -> Result<(), Error> {
    let rose = get_bytecode(db);

    raw_block_exec(rose);

    Ok(())
}
