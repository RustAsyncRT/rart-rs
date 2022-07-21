use crate::common::ArcMutex;
use crate::futures::zbus::ZbusState;
use crate::{Expect, RARTError};
use crate::no_std::arc::Arc;
use crate::no_std::blocking_mutex::BlockingMutex;

#[repr(C)]
pub struct zbus_observer {
    enabled: bool,
    queue: *const (),
    callback: unsafe extern "C" fn(u32),
}

impl zbus_observer {
    pub const fn new() -> Self {
        Self {
            enabled: true,
            queue: core::ptr::null(),
            callback: rtos_zbus_default_listener_callback,
        }
    }
}

unsafe impl Sync for zbus_observer {}

pub fn zbus_register_observer<T: Clone>(id: u32, state: ArcMutex<ZbusState<T>>) {
    let state_ptr = Arc::into_raw(state) as *const ();

    unsafe {
        rtos_zbus_register_observer(id, state_ptr, rtos_zbus_callback::<T>);
    }
}

pub fn zbus_publish<T>(id: u32, data: T) -> Result<(), RARTError>
{
    let data_ptr = &data as *const T as *const ();
    let data_size = core::mem::size_of::<T>() as u32;
    let err = unsafe {
        rtos_zbus_publish(id, data_ptr, data_size)
    };
    if err != 0 {
        Err(RARTError::CError(err))
    } else {
        Ok(())
    }
}

extern "C" fn rtos_zbus_callback<T: Clone>(state: *const (), data: *const ()) {
    let state = unsafe { Arc::from_raw(state as *const BlockingMutex<ZbusState<T>>) };
    let mut state = state.lock().rart_expect("Cannot lock at rtos zbus callback");
    if let ZbusState::Waiting(waker) = &*state {
        waker.wake_by_ref();
    }
    unsafe {
        let data: T = (*(data as *const T)).clone();
        *state = ZbusState::Completed(data);
    }
}

extern "C" {
    fn rtos_zbus_register_observer(id: u32, state: *const (), callback: unsafe extern "C" fn(*const (), *const ()));
    fn rtos_zbus_publish(id: u32, data: *const (), size: u32) -> i32;
    fn rtos_zbus_default_listener_callback(idx: u32);
}
