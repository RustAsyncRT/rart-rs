use core::future::Future;
use core::marker::PhantomData;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::task::Waker;
use crate::common::arc::Arc;
use crate::common::ArcMutex;
use crate::common::blocking_mutex::BlockingMutex;
use crate::{delay_secs, RARTError};
use crate::futures::zbus_backend::{zbus_publish, zbus_register_observer};

pub struct ZbusChannel<T: Clone> {
    marker: PhantomData<T>,
    id: u32,
}

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
            delay_secs(1).await;
        }
    }
}

struct ZbusReceiver<'a, T: Clone> {
    channel: &'a ZbusChannel<T>,
    state: ArcMutex<ZbusState<T>>,
}

pub enum ZbusState<T> {
    None,
    Waiting(Waker),
    #[allow(dead_code)]
    Completed(T),
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
