use heapless::Deque;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use crate::common::arc::Arc;
use core::sync::atomic::{AtomicBool, Ordering};
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use crate::common::ArcMutex;
use crate::common::blocking_mutex::BlockingMutex;
use crate::common::result::Expect;

pub struct Mutex<T, const TN: usize> {
    data: UnsafeCell<T>,
    is_unlocked: AtomicBool,
    wakers: ArcMutex<Deque<Waker, TN>>,
}

pub struct MutexGuard<T: 'static, const TN: usize> {
    mutex: &'static Mutex<T, TN>,
}

impl<T: 'static, const TN: usize> Mutex<T, TN> {
    pub fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
            is_unlocked: AtomicBool::new(true),
            wakers: Arc::new(BlockingMutex::new(Deque::new())),
        }
    }

    pub async fn lock(&'static self) -> MutexGuard<T, TN> {
        self.await
    }

    fn unlock(&self) {
        self.is_unlocked.store(true, Ordering::Release)
    }
}

impl<T: 'static, const TN: usize> Future for &'static Mutex<T, TN> {
    type Output = MutexGuard<T, TN>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.is_unlocked.compare_exchange(true, false,
                                             Ordering::AcqRel,
                                             Ordering::Relaxed).is_ok() {
            Poll::Ready(MutexGuard { mutex: *self })
        } else {
            let mut wakers = self.wakers.lock()
                .rart_expect("Cannot lock mutex wakers");
            wakers.push_back(cx.waker().clone()).rart_expect("Cannot store mutex waker");
            Poll::Pending
        }
    }
}

impl<T: 'static, const TN: usize> Drop for MutexGuard<T, TN> {
    fn drop(&mut self) {
        let mutex = &*self.mutex;
        mutex.unlock();

        let mut wakers = mutex.wakers.lock()
            .rart_expect("Cannot lock mutex wakers");
        if let Some(next_waker) = wakers.pop_front() {
            next_waker.wake_by_ref();
        }
    }
}

impl<T: 'static, const TN: usize> Deref for MutexGuard<T, TN> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // TODO Explain why this is safe
        unsafe {
            let mutex = &*self.mutex;
            let data_ptr = &*mutex.data.get();
            data_ptr
        }
    }
}

impl<T: 'static, const TN: usize> DerefMut for MutexGuard<T, TN> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // TODO Explain why this is safe
        unsafe {
            let mutex = &*self.mutex;
            let data_ptr = &mut *mutex.data.get();
            data_ptr
        }
    }
}
