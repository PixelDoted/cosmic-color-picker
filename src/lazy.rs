use std::cell::{Cell, OnceCell};

/// A copy to std's [`Lazy`]
pub struct Lazy<T, F = fn() -> T> {
    cell: OnceCell<T>,
    init: Cell<F>,
}

impl<T, F: Fn() -> T> Lazy<T, F> {
    pub fn get_inner(&self) -> &'static T {
        &self.cell.get_or_init(self.init.into_inner())
    }
}
