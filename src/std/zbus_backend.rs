#[cfg(all(feature = "std", feature = "zbus"))]
use crate::common::ArcMutex;
#[cfg(all(feature = "std", feature = "zbus"))]
use crate::futures::zbus::ZbusState;
#[cfg(all(feature = "std", feature = "zbus"))]
use crate::RARTError;

#[cfg(all(feature = "std", feature = "zbus"))]
pub fn zbus_register_observer<T: Clone>(_id: u32, _state: ArcMutex<ZbusState<T>>) {
    todo!()
}

#[cfg(all(feature = "std", feature = "zbus"))]
pub fn zbus_publish<T>(_id: u32, _data: T) -> Result<(), RARTError>
{
    todo!()
}
