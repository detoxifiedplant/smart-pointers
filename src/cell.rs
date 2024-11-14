use std::cell::UnsafeCell;

struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }

    fn set(&self, value: T) {
        unsafe { 
            *self.value.get() = value;
        }
    }
}
