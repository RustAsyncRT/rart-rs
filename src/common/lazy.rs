use core::cell::UnsafeCell;
use crate::common::UnsafeOption;
use crate::common::result::Expect;

pub struct Lazy<T: Sized> {
    data: UnsafeOption<T>,
}

// TODO Explain why this is safe
unsafe impl<T: Sized> Sync for Lazy<T> {}

unsafe impl<T: Sized> Send for Lazy<T> {}

impl<T: Sized> Lazy<T> {
    pub const fn new() -> Self {
        Self {
            data: UnsafeCell::new(None)
        }
    }

    pub fn init(&self, data: T) {
        // TODO Explain why this is safe
        unsafe {
            let data_ptr = &mut *self.data.get();
            if data_ptr.is_none() {
                *data_ptr = Some(data);
            }
        }
    }

    pub fn data(&self) -> &T {
        // TODO Explain why this is safe
        unsafe {
            let data_ptr = &*self.data.get();
            data_ptr.as_ref().rart_expect("Cannot get the data from empty lazy")
        }
    }
}
