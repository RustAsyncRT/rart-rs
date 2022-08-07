#[cfg(feature = "zbus")]
use core::future::Future;
#[cfg(feature = "zbus")]
use core::marker::PhantomData;
#[cfg(feature = "zbus")]
use core::pin::Pin;
#[cfg(feature = "zbus")]
use core::task::{Context, Poll};
#[cfg(feature = "zbus")]
use core::task::Waker;
#[cfg(feature = "zbus")]
use core::time::Duration;
#[cfg(feature = "zbus")]
use crate::common::arc::Arc;
#[cfg(feature = "zbus")]
use crate::common::ArcMutex;
#[cfg(feature = "zbus")]
use crate::common::blocking_mutex::BlockingMutex;
#[cfg(feature = "zbus")]
use crate::{RARTError};
#[cfg(feature = "zbus")]
use crate::delay;
#[cfg(feature = "zbus")]
use crate::futures::zbus_backend::{zbus_publish, zbus_register_observer};

#[cfg(feature = "zbus")]
pub struct ZbusChannel<T: Clone> {
    marker: PhantomData<T>,
    id: u32,
}

#[cfg(feature = "zbus")]
impl<T: Clone> ZbusChannel<T> {
    pub fn new(id: u32) -> Self {
        Self {
            marker: PhantomData,
            id,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub async fn read(&self) -> T {
        let receiver = ZbusReceiver::new(self);
        receiver.await
    }

    pub fn try_publish(&self, data: T) -> Result<(), RARTError> {
        zbus_publish(self.id(), data)
    }

    pub async fn publish(&self, data: T) {
        while let Err(_) = self.try_publish(data.clone()) {
            delay(Duration::from_secs(1)).await;
        }
    }
}

#[cfg(feature = "zbus")]
struct ZbusReceiver<'a, T: Clone> {
    channel: &'a ZbusChannel<T>,
    state: ArcMutex<ZbusState<T>>,
}

#[cfg(feature = "zbus")]
pub enum ZbusState<T> {
    None,
    Waiting(Waker),
    #[allow(dead_code)]
    Completed(T),
}

#[cfg(feature = "zbus")]
impl<'a, T: Clone> ZbusReceiver<'a, T> {
    pub fn new(channel: &'a ZbusChannel<T>) -> Self {
        Self {
            channel,
            state: Arc::new(BlockingMutex::new(ZbusState::None)),
        }
    }
}

#[cfg(feature = "zbus")]
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
