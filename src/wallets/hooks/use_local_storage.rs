use dioxus::prelude::*;
use std::{
    cell::{Cell, Ref, RefCell, RefMut},
    collections::HashSet,
    rc::Rc,
    sync::Arc,
};

type ProvidedStorage<T> = Rc<RefCell<ProvidedStorageInner<T>>>;

// Tracks all the subscribers to a shared State
pub struct ProvidedStorageInner<T> {
    value: Rc<RefCell<T>>,
    notify_any: Arc<dyn Fn(ScopeId)>,
    consumers: HashSet<ScopeId>,
}

impl<T> ProvidedStorageInner<T> {
    pub(crate) fn notify_consumers(&mut self) {
        for consumer in self.consumers.iter() {
            (self.notify_any)(*consumer);
        }
    }

    pub fn write(&self) -> RefMut<T> {
        self.value.borrow_mut()
    }

    pub fn read(&self) -> Ref<T> {
        self.value.borrow()
    }
}

/// This hook provides some relatively light ergonomics around shared state.
///
/// It is not a substitute for a proper state management system, but it is capable enough to provide use_state - type
/// ergonomics in a pinch, with zero cost.
///
/// # Example
///
/// ## Provider
///
/// ```rust, ignore
///
///
/// ```
///
/// ## Consumer
///
/// ```rust, ignore
///
///
/// ```
///
/// # How it works
///
/// Any time a component calls `write`, every consumer of the state will be notified - excluding the provider.
///
/// Right now, there is not a distinction between read-only and write-only, so every consumer will be notified.
///
///
///
pub fn use_local_storage<T: 'static>(cx: &ScopeState) -> Option<UseLocalStorage<T>> {
    let state = cx.use_hook(|| {
        let scope_id = cx.scope_id();
        let root = cx.consume_context::<ProvidedStorage<T>>();

        if let Some(root) = root.as_ref() {
            root.borrow_mut().consumers.insert(scope_id);
        }

        let value = root.as_ref().map(|f| f.borrow().value.clone());
        LocalStorageInner {
            root,
            value,
            scope_id,
            needs_notification: Cell::new(false),
        }
    });

    state.needs_notification.set(false);
    match (&state.value, &state.root) {
        (Some(value), Some(root)) => Some(UseLocalStorage {
            cx,
            value,
            root,
            needs_notification: &state.needs_notification,
        }),
        _ => None,
    }
}

struct LocalStorageInner<T: 'static> {
    root: Option<ProvidedStorage<T>>,
    value: Option<Rc<RefCell<T>>>,
    scope_id: ScopeId,
    needs_notification: Cell<bool>,
}
impl<T> Drop for LocalStorageInner<T> {
    fn drop(&mut self) {
        // we need to unsubscribe when our component is unounted
        if let Some(root) = &self.root {
            let mut root = root.borrow_mut();
            root.consumers.remove(&self.scope_id);
        }
    }
}

pub struct UseLocalStorage<'a, T: 'static> {
    pub(crate) cx: &'a ScopeState,
    pub(crate) value: &'a Rc<RefCell<T>>,
    pub(crate) root: &'a Rc<RefCell<ProvidedStorageInner<T>>>,
    pub(crate) needs_notification: &'a Cell<bool>,
}

impl<'a, T: 'static> UseLocalStorage<'a, T> {
    pub fn read(&self) -> Ref<'_, T> {
        self.value.borrow()
    }

    pub fn notify_consumers(self) {
        if !self.needs_notification.get() {
            self.root.borrow_mut().notify_consumers();
            self.needs_notification.set(true);
        }
    }

    pub fn read_write(&self) -> (Ref<'_, T>, &Self) {
        (self.read(), self)
    }

    /// Calling "write" will force the component to re-render
    ///
    ///
    /// TODO: We prevent unncessary notifications only in the hook, but we should figure out some more global lock
    pub fn write(&self) -> RefMut<'_, T> {
        self.cx.needs_update();
        self.notify_consumers();
        self.value.borrow_mut()
    }

    /// Allows the ability to write the value without forcing a re-render
    pub fn write_silent(&self) -> RefMut<'_, T> {
        self.value.borrow_mut()
    }

    pub fn inner(&self) -> Rc<RefCell<ProvidedStorageInner<T>>> {
        self.root.clone()
    }
}

impl<T> Copy for UseLocalStorage<'_, T> {}
impl<'a, T> Clone for UseLocalStorage<'a, T>
where
    T: 'static,
{
    fn clone(&self) -> Self {
        UseLocalStorage {
            cx: self.cx,
            value: self.value,
            root: self.root,
            needs_notification: self.needs_notification,
        }
    }
}

/// Provide some state for components down the hierarchy to consume without having to drill props.
pub fn use_local_storage_provider<T: 'static>(cx: &ScopeState, f: impl FnOnce() -> T) {
    cx.use_hook(|| {
        let state: ProvidedStorage<T> = Rc::new(RefCell::new(ProvidedStorageInner {
            value: Rc::new(RefCell::new(f())),
            notify_any: cx.schedule_update_any(),
            consumers: HashSet::new(),
        }));

        cx.provide_context(state);
    });
}
