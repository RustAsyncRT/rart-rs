use heapless::Deque;
use crate::common::arc::Arc;
use crate::common::ArcMutex;
use crate::common::blocking_mutex::BlockingMutex;
use crate::common::result::RARTError;
use crate::futures::semaphore::Semaphore;

pub struct Channel<T: Sized, const N: usize, const TN: usize> {
    queue: ArcMutex<Deque<T, N>>,
    sem: Semaphore<N, TN>,
}

impl<T: Sized, const N: usize, const TN: usize> Channel<T, N, TN> {
    pub fn new() -> Self {
        let deq = Deque::new();
        Self {
            queue: Arc::new(BlockingMutex::new(deq)),
            sem: Semaphore::new(0),
        }
    }

    pub async fn send(&'static self, data: T) -> Result<(), RARTError> {
        self.sem.wait_give().await;
        let mut queue = self.queue.lock()?;
        if let Err(_) = queue.push_back(data) {
            Err(RARTError::SendError)
        } else {
            Ok(())
        }
    }

    pub async fn recv(&'static self) -> Result<T, RARTError> {
        self.sem.take().await;
        let mut queue = self.queue.lock()?;
        queue.pop_front().ok_or(RARTError::RecvError)
    }
}
