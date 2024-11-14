use std::cell::{Cell, UnsafeCell};

#[derive(Clone, Copy)]
enum RefState {
    Unshared,
    Exclusive,
    Shared(usize),
}

struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

impl<T> RefCell<T> {
    fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Shared(1)),
        }
    }

    fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => self.state.set(RefState::Shared(1)),
            RefState::Exclusive => return None,
            RefState::Shared(n) => self.state.set(RefState::Shared(n + 1)),
        }
        Some(Ref { refcell: self })
    }

    fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        match self.state.get() {
            RefState::Unshared => self.state.set(RefState::Exclusive),
            RefState::Exclusive | RefState::Shared(_) => return None
        }
        Some(RefMut { refcell: self })
    }
}

struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> std::ops::Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(1) => self.refcell.state.set(RefState::Unshared),
            RefState::Shared(n) => self.refcell.state.set(RefState::Shared(n - 1)),
            RefState::Unshared | RefState::Exclusive => unreachable!(),
        }
    }
}

struct RefMut<'refcell ,T> {
    refcell: &'refcell RefCell<T>
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> std::ops::DerefMut for RefMut<'_ , T>{
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> std::ops::Drop for RefMut<'_ , T>{
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive => self.refcell.state.set(RefState::Unshared),
            _ => unreachable!()
        }
    }
}
