use std::collections::HashMap;
use std::lazy::Lazy;
use std::mem;
use std::sync::Mutex;
use crate::{Gpio, Peripheral};
use crate::peripherals::PeripheralKind;

struct GpioState {
    port: u32,
    pin: u32,
    state: bool,
}

static mut GPIOS_STATE: Lazy<Mutex<HashMap<usize, GpioState>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[cfg(feature = "std")]
pub unsafe fn peripheral_open(peripheral_kind: usize, config: *const ()) -> usize
{
    if peripheral_kind != Gpio::peripheral_kind() {
        return 0;
    }

    let cfg: &GpioState = mem::transmute(config);

    let mut gpios_state = GPIOS_STATE.lock().unwrap();
    let id = gpios_state.len();
    gpios_state.insert(id, GpioState { port: cfg.port, pin: cfg.pin, state: false });

    id
}

#[cfg(feature = "std")]
pub unsafe fn peripheral_close(peripheral_kind: usize, id: usize)
{
    if peripheral_kind != Gpio::peripheral_kind() {
        return;
    }

    let mut gpios_state = GPIOS_STATE.lock().unwrap();
    gpios_state.remove(&id);
}

#[cfg(feature = "std")]
pub unsafe fn peripheral_write(peripheral_kind: usize, id: usize, data: *const ())
{
    if peripheral_kind != Gpio::peripheral_kind() {
        return;
    }

    let mut gpios_state = GPIOS_STATE.lock().unwrap();

    if !gpios_state.contains_key(&id) {
        return;
    }

    let data: &bool = mem::transmute(data);
    let state = gpios_state.get_mut(&id).unwrap();
    state.state = *data;
}

#[cfg(feature = "std")]
pub unsafe fn peripheral_read(peripheral_kind: usize, id: usize, data: *mut ())
{
    if peripheral_kind != Gpio::peripheral_kind() {
        return;
    }

    let gpios_state = GPIOS_STATE.lock().unwrap();

    if !gpios_state.contains_key(&id) {
        return;
    }

    let data: &mut bool = mem::transmute(data);
    let state = gpios_state.get(&id).unwrap();
    *data = state.state;
}

#[cfg(feature = "std")]
pub fn read_gpio(gpio: &Peripheral<bool, Gpio>) -> bool {
    let mut data = false;
    unsafe { peripheral_read(1, gpio.get_id(), &mut data as *mut bool as *mut ()) };
    data
}