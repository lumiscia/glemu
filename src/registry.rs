use crate::state::ContextState;
use crate::types::ContextId;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use web_sys::WebGl2RenderingContext as Gl;

thread_local! {
    static GL_CONTEXTS: RefCell<HashMap<ContextId, ContextState>> = RefCell::new(HashMap::new());
    static GL_CURRENT: Cell<Option<ContextId>> = const { Cell::new(None) };
    static GL_NEXT_CONTEXT_ID: Cell<u32> = const { Cell::new(1) };
}

pub fn register_gl_context(gl: Gl) -> ContextId {
    let context_id = GL_NEXT_CONTEXT_ID.with(|next| {
        let raw = next.get();
        next.set(raw.checked_add(1).expect("context id overflowed"));
        ContextId::new(raw).expect("context ids start at one")
    });

    GL_CONTEXTS.with(|contexts| {
        contexts
            .borrow_mut()
            .insert(context_id, ContextState::new(gl));
    });
    GL_CURRENT.with(|current| current.set(Some(context_id)));

    context_id
}

pub fn set_gl_context(context_id: ContextId) -> bool {
    let exists = GL_CONTEXTS.with(|contexts| contexts.borrow().contains_key(&context_id));
    if exists {
        GL_CURRENT.with(|current| current.set(Some(context_id)));
    }
    exists
}

pub fn drop_gl_context(context_id: ContextId) -> bool {
    let removed = GL_CONTEXTS.with(|contexts| contexts.borrow_mut().remove(&context_id));
    if removed.is_some() {
        GL_CURRENT.with(|current| {
            if current.get() == Some(context_id) {
                current.set(None);
            }
        });
    }
    removed.is_some()
}

pub fn current_context_id() -> Option<ContextId> {
    GL_CURRENT.with(Cell::get)
}

pub(crate) fn has_gl_context(context_id: ContextId) -> bool {
    GL_CONTEXTS.with(|contexts| contexts.borrow().contains_key(&context_id))
}

pub(crate) fn with_context<R>(
    context_id: ContextId,
    f: impl FnOnce(&ContextState) -> R,
) -> Option<R> {
    GL_CONTEXTS.with(|contexts| contexts.borrow().get(&context_id).map(f))
}

pub(crate) fn with_context_mut<R>(
    context_id: ContextId,
    f: impl FnOnce(&mut ContextState) -> R,
) -> Option<R> {
    GL_CONTEXTS.with(|contexts| contexts.borrow_mut().get_mut(&context_id).map(f))
}

pub(crate) fn with_current_context_mut<R>(f: impl FnOnce(&mut ContextState) -> R) -> Option<R> {
    let context_id = current_context_id()?;
    with_context_mut(context_id, f)
}
