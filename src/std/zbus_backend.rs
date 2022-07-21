use crate::common::ArcMutex;
use crate::futures::zbus::ZbusState;
use crate::RARTError;

pub fn zbus_register_observer<T: Clone>(_id: u32, _state: ArcMutex<ZbusState<T>>) {
    todo!()
}

pub fn zbus_publish<T>(_id: u32, _data: T) -> Result<(), RARTError>
{
    todo!()
}
