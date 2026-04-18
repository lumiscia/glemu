#![allow(clippy::missing_safety_doc, clippy::too_many_arguments)]

mod core;
mod es3;
mod proc_table;

use crate::{registry::with_current_context_mut, state::ContextState};

pub use proc_table::get_proc_address;

pub(crate) static EMPTY_C_STRING: [u8; 1] = [0];

pub(crate) fn with_gl<R: Default>(f: impl FnOnce(&mut ContextState) -> R) -> R {
    with_current_context_mut(f).unwrap_or_default()
}
