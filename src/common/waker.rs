use core::task::RawWakerVTable;
use core::task::RawWaker;
use crate::common::arc::Arc;
use core::task::Waker;
use crate::common::ArcMutex;
use crate::common::blocking_channel::BlockingSender;
use crate::common::task::TaskId;
use crate::{log};
use crate::common::result::Expect;
#[cfg(not(feature = "std"))]
use const_format::formatcp;
#[cfg(not(feature = "std"))]
use crate::no_std::log_fn;

pub struct RARTWaker {
    task_id: TaskId,
    sender: ArcMutex<BlockingSender<TaskId>>,
}

impl RARTWaker {
    pub fn new(task_id: TaskId, sender: ArcMutex<BlockingSender<TaskId>>) -> Self {
        RARTWaker {
            task_id,
            sender,
        }
    }
}

// TODO Explain why this is safe
const VTABLE: RawWakerVTable = unsafe {
    RawWakerVTable::new(
        |s| wake_clone(&*(s as *const RARTWaker)),
        |s| waker_wake(&*(s as *const RARTWaker)),
        |s| waker_wake_by_ref(&*(s as *const RARTWaker)),
        |s| drop(Arc::from_raw(s as *const RARTWaker)),
    )
};

fn wake_clone(s: &RARTWaker) -> RawWaker {
    // TODO Explain why this is safe
    let arc = unsafe { Arc::from_raw(s) };
    core::mem::forget(arc.clone());
    RawWaker::new(Arc::into_raw(arc) as *const (), &VTABLE)
}

// TODO Why this function doesn't decrease the arc counter?
// TODO Or decrease?
// TODO     If yes, where?
fn waker_wake(s: &RARTWaker) {
    let waker_ptr: *const RARTWaker = s;
    // TODO Explain why this is safe
    let waker_arc = unsafe { Arc::from_raw(waker_ptr) };
    let sender = waker_arc.sender.lock().rart_expect("Cannot get waker queue sender");
    sender.send(waker_arc.task_id).rart_expect("Cannot get waker queue sender");
}

fn waker_wake_by_ref(s: &RARTWaker) {
    let sender = s.sender.lock().rart_expect("Cannot get waker sender");
    if let Err(e) = sender.send(s.task_id) {
        #[cfg(not(feature = "std"))]
        log!("waker by ref error: %d", e);
        #[cfg(feature = "std")]
        log!("waker by ref error: {}", e);
    }
}

pub fn rart_waker_into_waker(s: *const RARTWaker) -> Waker {
    let raw_waker = RawWaker::new(s as *const (), &VTABLE);
    // TODO Explain why this is safe
    unsafe { Waker::from_raw(raw_waker) }
}
