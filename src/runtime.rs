use crate::database::{get_bytecode, Database};
use crate::rose::{rose_block_exec, RoseEngine, RoseRuntimeImpl};
use crate::scratch::Scratch;

pub enum Error {
    /// A parameter passed to this function was invalid.
    Invalid,
    /// The engine was terminated by callback.
    ScanTerminated,
    /// Unexpected internal error.
    Unknown,
}

/// Initializes SOM state. Used in both block and streaming mode.
fn init_som_state(_rose: &RoseEngine, _state: *mut u8) {}

fn raw_block_exec(rose: &RoseEngine, scratch: &mut Scratch) {
    init_som_state(rose, scratch.core_info.state);

    rose_block_exec(rose);
}

fn pure_literal_block_exec(_rose: &RoseEngine, _scratch: &mut Scratch) {}

fn single_outfix_block_exec(_rose: &RoseEngine, _scratch: &mut Scratch) {}

pub fn scan(db: &Database, _data: &[u8], scratch: &mut Scratch) -> Result<(), Error> {
    let rose = get_bytecode(db);

    match rose.runtime_impl {
        RoseRuntimeImpl::FullRose => raw_block_exec(rose, scratch),
        RoseRuntimeImpl::PureLiteral => pure_literal_block_exec(rose, scratch),
        RoseRuntimeImpl::SingleOutfix => single_outfix_block_exec(rose, scratch),
    }

    Ok(())
}
