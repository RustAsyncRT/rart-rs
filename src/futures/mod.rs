pub mod time;
pub mod mutex;
pub mod trigger;
pub mod semaphore;
pub mod channel;
pub mod zbus;
pub mod zbus_backend {
    #[cfg(feature = "std")]
    pub use crate::std::zbus_backend::{zbus_publish, zbus_register_observer};
    #[cfg(not(feature = "std"))]
    pub use crate::no_std::zbus_backend::{zbus_publish, zbus_register_observer, zbus_observer};
}
