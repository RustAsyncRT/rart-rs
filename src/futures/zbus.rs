use core::future::Future;
use core::marker::PhantomData;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::task::Waker;
use crate::common::arc::Arc;
use crate::common::ArcMutex;
use crate::common::blocking_mutex::BlockingMutex;
use crate::{delay_secs, Expect, log, log_fn, MCError};
use const_format::formatcp;

#[derive(Clone)]
pub enum ZbusChannelIndex {
    Ping,
    Pong,
}

impl Into<u32> for ZbusChannelIndex {
    fn into(self) -> u32 {
        match self {
            ZbusChannelIndex::Ping => 0,
            ZbusChannelIndex::Pong => 1,
        }
    }
}

pub struct ZbusChannel<T: Clone> {
    marker: PhantomData<T>,
    id: ZbusChannelIndex,
}

impl<T: Clone> ZbusChannel<T> {
    pub fn new(id: ZbusChannelIndex) -> Self {
        Self {
            marker: PhantomData,
            id,
        }
    }

    pub fn id(&self) -> u32 {
        self.id.clone().into()
    }

    pub async fn read(&self) -> T {
        let receiver = ZbusReceiver::new(self);
        receiver.await
    }

    pub fn try_publish(&self, data: T) -> Result<(), MCError> {
        let err = unsafe {
            let data_ptr = &data as *const T as *const ();
            rtos_zbus_publish(self.id(), data_ptr, core::mem::size_of::<T>() as u32)
        };
        if err != 0 {
            Err(MCError::Generic)
        } else {
            Ok(())
        }
    }

    pub async fn publish(&self, data: T) {
        while let Err(_) = self.try_publish(data.clone()) {
            delay_secs(1).await;
        }
    }
}

pub enum ZbusState<T> {
    None,
    Waiting(Waker),
    Completed(T),
}

struct ZbusReceiver<'a, T: Clone> {
    channel: &'a ZbusChannel<T>,
    state: ArcMutex<ZbusState<T>>,
}

impl<'a, T: Clone> ZbusReceiver<'a, T> {
    pub fn new(channel: &'a ZbusChannel<T>) -> Self {
        Self {
            channel,
            state: Arc::new(BlockingMutex::new(ZbusState::None)),
        }
    }
}

impl<'a, T: Clone> Future for ZbusReceiver<'a, T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Ok(mut state) = self.state.lock() {
            match &mut *state {
                ZbusState::None => {
                    *state = ZbusState::Waiting(cx.waker().clone());
                    zbus_register_observer(self.channel.id(), self.state.clone());
                    Poll::Pending
                }
                ZbusState::Waiting(_) => {
                    panic!("The \"Waiting\" state must be unreachable at the zbus future poll")
                }
                ZbusState::Completed(data) => {
                    Poll::Ready(data.clone())
                }
            }
        } else {
            panic!("Cannot get zbus future mutex");
        }
    }
}

pub fn zbus_register_observer<T: Clone>(id: u32, state: ArcMutex<ZbusState<T>>) {
    let state_ptr = Arc::into_raw(state) as *const ();

    unsafe {
        rtos_zbus_register_observer(id, state_ptr, rtos_zbus_callback::<T>);
    }
}

pub extern "C" fn rtos_zbus_callback<T: Clone>(state: *const (), data: *const ()) {
    let state = unsafe { Arc::from_raw(state as *const BlockingMutex<ZbusState<T>>) };
    let mut state = state.lock().mc_expect("Cannot lock at rtos zbus callback");
    if let ZbusState::Waiting(waker) = &*state {
        waker.wake_by_ref();
    }
    unsafe {
        let data: T = (*(data as *const T)).clone();
        *state = ZbusState::Completed(data);
    }
}

extern "C" {
    pub fn rtos_zbus_register_observer(id: u32, state: *const (), callback: unsafe extern "C" fn(*const (), *const ()));
    pub fn rtos_zbus_publish(id: u32, data: *const (), size: u32) -> i32;
    pub fn rtos_zbus_default_listener_callback(idx: u32);
}

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
