use crate::futures::time::DelayState;
use crate::no_std::arc::Arc;
use crate::common::blocking_mutex::BlockingMutex;
use crate::common::logger::*;
use crate::common::result::Expect;
use crate::{MCError, trace};

pub fn timer_init() {
    // TODO Explain why this is safe
    unsafe {
        rtos_timer_init();
    }
}

pub fn timer_new_delay(state: Arc<BlockingMutex<DelayState>>, timeout: u32) -> Result<(), MCError> {
    let state_ptr = Arc::into_raw(state) as *const ();

    // TODO Explain why this is safe
    unsafe {
        rtos_timer_reschedule(rtos_timer_timeout, state_ptr, timeout);
    }

    Ok(())
}

#[no_mangle]
pub extern "C" fn rtos_timer_timeout(state: *const ()) {
    // TODO Explain why this is safe
    trace!();
    let state = unsafe { Arc::from_raw(state as *const BlockingMutex<DelayState>) };
    trace!();
    let mut state = state.lock().mc_expect("Cannot lock at rtos timer timeout");
    trace!();

    if let DelayState::Waiting(waker) = &*state {
        trace!();
        waker.wake_by_ref();
    }
    trace!();
    *state = DelayState::Completed;
    trace!();
}

extern "C" {
    pub fn rtos_timer_init();
    pub fn rtos_timer_reschedule(callback: unsafe extern "C" fn(*const ()), state: *const (), timeout: u32);
}
